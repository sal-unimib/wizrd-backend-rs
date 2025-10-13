use std::collections::HashMap;
use rocket::http::Status;
use sqlx::PgConnection;
use crate::model::Vehicle;
use crate::repository;

pub async fn get(conn: &mut PgConnection, kind: &str) -> Result<Vehicle, Status> {
    let tm = repository::vehicle::get_vehicle(conn, kind)
        .await
        .map_err(|_e| {
            eprintln!("{:?}", _e);
            Status::InternalServerError
        })?;
    tm.ok_or(Status::NotFound)
}

pub async fn get_all(
    conn: &mut PgConnection,
) -> Result<HashMap<String, Vehicle>, Status> {
    let mut ret = HashMap::new();
    repository::vehicle::get_vehicles(conn)
        .await
        .map_err(|_e| {
            eprintln!("{:?}", _e);
            Status::InternalServerError
        })?
        .iter()
        .for_each(|t| {
            ret.insert(t.kind.to_string(), t.clone());
        });
    Ok(ret)
}