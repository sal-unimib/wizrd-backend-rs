use crate::db::Db;
use crate::model::Vehicle;
use crate::service;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::Acquire;

#[get("/vehicle/all")]
pub async fn get_vehicles(
    mut db: Connection<Db>,
) -> Result<Json<Vec<Vehicle>>, Status> {
    let conn = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;

    let methods = service::vehicle::get_all(conn)
        .await
        .map_err(|_e| {
            eprintln!("{:?}", _e);
            Status::InternalServerError
        })?;
    Ok(Json(
        methods.values().cloned().collect::<Vec<Vehicle>>(),
    ))
}

#[get("/vehicle?<kind>")]
pub async fn get_vehicle(
    mut db: Connection<Db>,
    kind: &str,
) -> Result<Json<Vehicle>, Status> {
    let conn = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;


    let method = service::vehicle::get(conn, kind)
        .await
        .map_err(|_e| {
            eprintln!("{:?}", _e);
            Status::NotFound
        })?;
    Ok(Json(method))
}
