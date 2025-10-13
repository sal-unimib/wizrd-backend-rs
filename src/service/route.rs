use crate::dto::{Route, RouteContainer, RouteRequest};
use crate::model::{Point};
use crate::repository;
use crate::service;
use rocket::http::Status;
use sqlx::PgConnection;
use crate::constants::VehicleKind;

pub async fn calculate(
    conn: &mut PgConnection,
    rr: &RouteRequest,
) -> Result<RouteContainer, Status> {
    let ways = repository::route::calculate(conn, rr).await.map_err(|_e| {
        eprintln!("{:?}", _e);
        Status::InternalServerError
    })?;

    let start = service::point::from_id(conn, rr.start_id).await?;
    let goal = service::point::from_id(conn, rr.goal_id).await?;

    let route_container = RouteContainer::new(
        vec![Route::new(
            start,
            goal,
            &rr.real_start_coords,
            &rr.real_goal_coords,
            ways,
        )],
        rr.vehicle_kind.clone(),
        false,
    );

    Ok(route_container)
}

pub async fn calculate_lumi(
    conn: &mut PgConnection,
    rr: &RouteRequest,
) -> Result<RouteContainer, Status> {
    let start = service::point::from_id(conn, rr.start_id).await?;
    let goal = service::point::from_id(conn, rr.goal_id).await?;

    let lumi1 = service::poi::nearest_by_kind(conn, start.lat, start.lon, Some("lumi")).await?;
    let lumi2 = service::poi::nearest_by_kind(conn, goal.lat, goal.lon, Some("lumi")).await?;

    // Calculate Ped Route
    let mut ped_request = rr.clone();
    ped_request.vehicle_kind = VehicleKind::None;

    let ped_route_container = RouteContainer::new(
        vec![request_route(conn, &start, &goal, &ped_request).await],
        rr.vehicle_kind.clone(),
        true,
    );

    // If no LUMI were found -> Ped Route
    if lumi1.is_none() || lumi2.is_none() {
        return Ok(ped_route_container);
    }

    let lumi1 = lumi1.unwrap();
    let lumi2 = lumi2.unwrap();

    // If start/goal share the same nearest LUMI -> Ped Route
    if lumi1.id == lumi2.id {
        return Ok(ped_route_container);
    }

    let mut lumi_routes = Vec::new();

    {
        // Get the points in the graph nearest to LUMI stations
        let l1_nearest = service::point::nearest(conn, lumi1.lat, lumi1.lon).await?;
        let l2_nearest = service::point::nearest(conn, lumi2.lat, lumi2.lon).await?;

        {
            // Walk from the Start to the first LUMI
            let mut walk_to_l1 = rr.clone();
            walk_to_l1.vehicle_kind = VehicleKind::None;
            walk_to_l1.goal_id = l1_nearest.id;
            walk_to_l1.set_real_goal_coords(lumi1.lat, lumi1.lon);
            lumi_routes.push(request_route(conn, &start, &l1_nearest, &walk_to_l1).await);
        }

        {
            // Rent a vehicle from the first LUMI to the second LUMI
            let mut rent_to_l2 = rr.clone();
            rent_to_l2.start_id = l1_nearest.id;
            rent_to_l2.set_real_start_coords(lumi1.lat, lumi1.lon);
            rent_to_l2.goal_id = l2_nearest.id;
            rent_to_l2.set_real_goal_coords(lumi2.lat, lumi2.lon);
            lumi_routes.push(request_route(conn, &l1_nearest, &l2_nearest, &rent_to_l2).await);
        }

        {
            // Walk from the second LUMI to the Goal
            let mut walk_to_goal = rr.clone();
            walk_to_goal.vehicle_kind = VehicleKind::None;
            walk_to_goal.start_id = l2_nearest.id;
            walk_to_goal.set_real_start_coords(lumi2.lat, lumi2.lon);
            lumi_routes.push(request_route(conn, &l2_nearest, &goal, &walk_to_goal).await);
        }
    }

    let lumi_route_container =
        RouteContainer::new(lumi_routes, rr.vehicle_kind.clone(), false);

    // Compare total times
    let t_ped = ped_route_container.total_time_min(conn).await?;
    let t_lumi = lumi_route_container.total_time_min(conn).await?;

    if t_ped < t_lumi {
        Ok(ped_route_container)
    } else {
        Ok(lumi_route_container)
    }
}

async fn request_route(
    conn: &mut PgConnection,
    p1: &Point,
    p2: &Point,
    rr: &RouteRequest,
) -> Route {
    // Calculate a route; if the calculation fails, return an empty route
    let ways = repository::route::calculate(conn, rr).await.map_err(|_e| {
        eprintln!("{:?}", _e);
        Status::InternalServerError
    });
    Route::new(
        p1.clone(),
        p2.clone(),
        &rr.real_start_coords,
        &rr.real_goal_coords,
        ways.unwrap_or(Vec::new()),
    )
}
