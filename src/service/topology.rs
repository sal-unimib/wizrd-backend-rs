use rocket::http::Status;
use crate::dto::TopologyWay;
use crate::repository;
use sqlx::PgConnection;

pub async fn get(conn: &mut PgConnection, filter: &str, source: &str) -> Result<Vec<TopologyWay>, Status> {
    let mut filter = match filter {
        "none" => Some("WHERE false"),
        "all" => Some(""),
        "roads" => Some(
        "WHERE w.highway IN ('primary', 'secondary', 'tertiary', 'primary_link', 'secondary_link', 'tertiary_link', 'road', 'living_street', 'unclassified', 'residential', 'service')"),
        "footways" => Some("WHERE w.highway IN ('footway', 'footpath', 'pedestrian', 'steps', 'foot_crossing')"),
        "cycleways" => Some("WHERE w.highway IN ('cycleway', 'cycle_crossing')"),
        "greenways" => Some("WHERE green IS true"),
        _ => None
    }.ok_or(Status::BadRequest)?;

    let source = match source {
        "none" => Some("NULL"),
        "pm2_low" => Some("pm2_low"),
        "pm2_medium" => Some("pm2_medium"),
        "pm2" | "pm2_high" => Some("pm2_high"),
        "traffic_low" => Some("traffic_low"),
        "traffic_medium" => Some("traffic_medium"),
        "traffic" | "traffic_high" => Some("traffic_high"),
        "comf_n" => Some("comf_n"),
        "qualinf_n" => Some("qualinf_n"),
        "sic_n" => Some("sic_n"),
        "acc_n" => Some("acc_n"),
        "walk_n" => Some("walk_n"),
        _ => None
    }.ok_or(Status::BadRequest)?;

    let f = String::from(filter) + " AND " + source + " IS NOT NULL";
    if source != "NULL" {
        filter = f.as_str();
    }

    repository::topology::filter(conn, filter, source).await.map_err(
        |_e| {
            eprintln!("{:?}", _e);
            Status::InternalServerError
        }
    )
}
