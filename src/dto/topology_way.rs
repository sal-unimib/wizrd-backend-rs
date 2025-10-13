use geojson::Feature;
use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopologyWay {
    pub id: i64,
    pub geom: Feature,
    pub data: Option<f64>
}
