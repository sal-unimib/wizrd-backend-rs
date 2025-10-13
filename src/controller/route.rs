use crate::db::Db;
use crate::dto::{RouteContainer, RouteRequest};
use crate::service;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::Acquire;

#[post("/route", data = "<route_request>")]
pub async fn post_route(
    mut db: Connection<Db>,
    route_request: Json<RouteRequest>,
) -> Result<Json<RouteContainer>, Status> {
    let conn = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let route = service::route::calculate(conn, &route_request).await?;
    Ok(Json(route))
}

#[post("/route/lumi", data = "<route_request>")]
pub async fn post_lumi_route(
    mut db: Connection<Db>,
    route_request: Json<RouteRequest>,
) -> Result<Json<RouteContainer>, Status> {
    let conn = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let lumi_route = service::route::calculate_lumi(conn, &route_request).await?;
    Ok(Json(lumi_route))
}
