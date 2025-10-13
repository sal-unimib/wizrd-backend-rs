use crate::constants::{
    multilevel_source, VehicleKind, AIR_QUALITY_SOURCE, ROUTING_ALGO_DIJKSTRA, TRAFFIC_SOURCE,
};
use crate::dto::{RouteRequest, Way};
use geo_traits::to_geo::ToGeoGeometry;
use rocket_db_pools::sqlx;
use sqlx::{PgConnection, Row};
use wkt::Wkt;

pub async fn calculate(
    conn: &mut PgConnection,
    rr: &RouteRequest,
) -> Result<Vec<Way>, sqlx::Error> {
    let query = from_route_request(rr).map_err(|_e| {
        eprintln!("{:?}", _e);
        sqlx::Error::Protocol(format!("Could not build query: {}", _e))
    })?;

    let rows = sqlx::query(&query)
        .bind(rr.start_id)
        .bind(rr.goal_id)
        .fetch_all(conn)
        .await?;

    let mut ways = Vec::new();
    for row in rows {
        // XXX [jm] You _must_ use lowercase column names
        let geom_wkt: Wkt = row
            .try_get::<&str, &str>("__geom")
            .map_err(|_e| {
                eprintln!("{:?}", _e);
                sqlx::Error::Protocol(format!("Could not retrieve Geometry: {}", _e))
            })?
            .parse::<Wkt>()
            .map_err(|_e| {
                eprintln!("{:?}", _e);
                sqlx::Error::Protocol(format!("Could not deserialize WKT: {}", _e))
            })?;

        ways.push(Way::new(
            row.get("id"),
            row.get("source"),
            row.get("x1"),
            row.get("y1"),
            row.get("target"),
            row.get("x2"),
            row.get("y2"),
            row.get("__distance"),
            row.get("name"),
            row.get("highway"),
            geom_wkt.to_geometry(),
            row.get("green"),
            rr.vehicle_kind,
        ));
    }

    Ok(ways)
}

fn calculate_cost_query(rr: &RouteRequest) -> Result<String, String> {
    let air_quality_profile = multilevel_source(AIR_QUALITY_SOURCE, rr.air_quality_level);
    let traffic_profile = multilevel_source(TRAFFIC_SOURCE, rr.traffic_level);
    let cost_query = format!(
        r#"
        CALCULATE_COST (
            W.ID,
            ''{}''::TEXT,
            ''{}''::TEXT,
            ''{}''::TEXT,
            {},
            {},
            {},
            {},
            {},
            {},
            W.LENGTH_M,
            W.HIGHWAY,
            W.MAXSPEED_FORWARD,
            W.GREEN,
            W.SURFACE,
            W.ONE_WAY,
            D.{},
            D.{}
        )
        "#,
        rr.vehicle_kind,
        air_quality_profile,
        traffic_profile,
        rr.consider_air_quality.unwrap_or(false),
        rr.consider_distance.unwrap_or(true),
        rr.consider_green_areas.unwrap_or(false),
        rr.consider_safety.unwrap_or(false),
        rr.consider_traffic.unwrap_or(false),
        rr.consider_mobility_impairment.unwrap_or(false),
        multilevel_source(AIR_QUALITY_SOURCE, rr.air_quality_level),
        multilevel_source(TRAFFIC_SOURCE, rr.traffic_level),
    );

    Ok(cost_query)
}

fn routing_query(rr: &RouteRequest, cost_call: &str) -> Result<String, String> {
    match rr.routing_algorithm_kind.as_str() {
        ROUTING_ALGO_DIJKSTRA => {
            // If the transport method is not Pedestrian, we use Directed Dijkstra to comply with
            // road regulations
            let directed = rr.vehicle_kind != VehicleKind::None;
            Ok(format!(
                r#"
                PGR_DIJKSTRA (
                    'SELECT
                        W.ID,
                        W.SOURCE,
                        W.TARGET,
                        C.COST,
                        C.REVERSE_COST
                    FROM
                        WAYS W
                        JOIN DATA D ON D.ID = W.ID
                        CROSS JOIN {} C',
                        {},
                        {},
                        {}
                )
                "#,
                cost_call, rr.start_id, rr.goal_id, directed
            ))
        }
        _ => Err(format!(
            "Requested routing algorithm \"{}\" is not implemented",
            rr.routing_algorithm_kind
        )),
    }
}

fn from_route_request(rr: &RouteRequest) -> Result<String, String> {
    let calculate_cost_query = calculate_cost_query(rr)?;
    let routing_query = routing_query(rr, &calculate_cost_query)?;
    let query = format!(
        r#"
        SELECT
            W.ID,
            W.SOURCE,
            W.X1,
            W.Y1,
            W.TARGET,
            W.X2,
            W.Y2,
            R.SEQ,
            W.NAME,
            SUM(W.LENGTH_M) AS __DISTANCE,
            W.HIGHWAY,
            W.GREEN,
            ST_ASTEXT (ST_LINEMERGE (ST_COLLECT (W.THE_GEOM))) AS __GEOM
        FROM
            {} AS R
            JOIN WAYS W ON W.ID = R.EDGE
        GROUP BY
            W.ID,
            W.SOURCE,
            W.X1,
            W.Y1,
            W.TARGET,
            W.X2,
            W.Y2,
            R.SEQ,
            W.NAME,
            W.HIGHWAY,
            W.GREEN;
        "#,
        routing_query
    );

    // eprintln!("---------------------------------------------------------------------------------");
    // eprintln!("QUERY:\n{}", query);
    // eprintln!("---------------------------------------------------------------------------------");

    Ok(query)
}
