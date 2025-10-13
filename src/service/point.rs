use rocket::http::Status;
use sqlx::PgConnection;
use crate::model::Point;
use crate::repository::point;

pub async fn from_id(conn: &mut PgConnection, id: i64) -> Result<Point, Status> {
    point::get_point(conn, id).await.map_err(|_e| {
        eprintln!("{:?}", _e);
        Status::InternalServerError
    })
}

pub async fn nearest(conn: &mut PgConnection, lat: f64, lon: f64) -> Result<Point, Status> {
    point::find_nearest_point(conn, lat, lon)
        .await
        .map_err(|_e| {
            eprintln!("{:?}", _e);
            Status::InternalServerError
        })
}