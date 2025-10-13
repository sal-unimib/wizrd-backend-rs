use crate::constants::{multilevel_source, AIR_QUALITY_SOURCE, TRAFFIC_SOURCE};
use crate::db::Db;
use crate::dto::TopologyWay;
use crate::service;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::Acquire;

#[get("/topology/all")]
pub async fn get_all_topology(mut pool: Connection<Db>) -> Result<Json<Vec<TopologyWay>>, Status> {
    let conn = pool
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let topo = service::topology::get(conn, "all", "none").await?;
    Ok(Json(topo))
}

#[get("/topology?<filter>&<source>&<level>")]
pub async fn get_topology(
    mut pool: Connection<Db>,
    filter: &str,
    mut source: &str,
    level: Option<i32>,
) -> Result<Json<Vec<TopologyWay>>, Status> {
    let conn = pool
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;

    let mls: String;
    if source == TRAFFIC_SOURCE || source == AIR_QUALITY_SOURCE {
        // If we're looking for AirQuality/Traffic, level must not be None
        let level = level.ok_or(Status::BadRequest)?;
        mls = multilevel_source(source, level);
        source = mls.as_str();
    }

    eprintln!(
        "filter: {:?}, source: {:?}, level: {:?}",
        filter, source, level
    );

    let topo = service::topology::get(conn, filter, source).await?;
    Ok(Json(topo))
}
