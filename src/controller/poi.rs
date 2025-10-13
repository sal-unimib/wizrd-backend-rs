use crate::db::Db;
use crate::model::poi::Poi;
use crate::service;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::Acquire;

#[get("/poi/all?<kind>")]
pub async fn get_all(
    mut pool: Connection<Db>,
    kind: Option<&str>,
) -> Result<Json<Vec<Poi>>, Status> {
    let conn = pool
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let pois = service::poi::filter(&mut *conn, kind).await?;
    Ok(Json(pois))
}

#[get("/poi/nearest?<lat>&<lon>&<kind>")]
pub async fn get_nearest(
    mut pool: Connection<Db>,
    lat: f64,
    lon: f64,
    kind: Option<&str>,
) -> Result<Json<Option<Poi>>, Status> {
    let conn = pool
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let pois = service::poi::nearest_by_kind(conn, lat, lon, kind).await?;
    Ok(Json(pois))
}
