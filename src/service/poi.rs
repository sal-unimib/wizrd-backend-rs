use crate::model::Poi;
use crate::repository;
use rocket::http::Status;
use sqlx::PgConnection;

pub async fn filter(conn: &mut PgConnection, kind: Option<&str>) -> Result<Vec<Poi>, Status> {
    let pois = match kind {
        Some(t) => repository::poi::get_all_by_kind(conn, t).await,
        None => repository::poi::get_all(conn).await,
    }
    .map_err(|_e| {
        eprintln!("{:?}", _e);
        Status::InternalServerError
    })?;
    Ok(pois)
}

pub async fn nearest_by_kind(
    conn: &mut PgConnection,
    lat: f64,
    lon: f64,
    kind: Option<&str>,
) -> Result<Option<Poi>, Status> {
    let poi = match kind {
        Some(t) => repository::poi::find_nearest_by_kind(conn, lat, lon, t).await,
        None => repository::poi::find_nearest(conn, lat, lon).await,
    }
    .map_err(|_e| {
        eprintln!("{:?}", _e);
        Status::InternalServerError
    })?;
    Ok(poi)
}
