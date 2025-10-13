use crate::db::Db;
use crate::model::point::Point;
use crate::repository::point::*;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::Acquire;

#[get("/point?<lat>&<lon>")]
pub async fn get_nearest_point(
    mut pool: Connection<Db>,
    lat: f64,
    lon: f64,
) -> Result<Json<Point>, Status> {
    let conn = pool.acquire().await.map_err(|_| Status::InternalServerError)?;
    let point = find_nearest_point(conn, lat, lon)
        .await
        .map_err(|_e| {
            eprintln!("Error finding nearest point: {:?}", _e);
            Status::InternalServerError
        })?;
    
    Ok(Json(point))
}

#[get("/point/<id>")]
pub async fn get_point_by_id(
    mut pool: Connection<Db>,
    id: i64,
) -> Result<Json<Point>, Status> {
    let conn = pool.acquire().await.map_err(|_| Status::InternalServerError)?;
    let point = get_point(conn, id)
        .await
        .map_err(|_e| {
            eprintln!("Error getting point by id {}: {:?}", id, _e);
            Status::InternalServerError
        })?;
    
    Ok(Json(point))
}