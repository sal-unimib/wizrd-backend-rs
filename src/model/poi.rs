use rocket_db_pools::sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Poi {
    pub id: i64,
    #[sqlx(rename = "type")]
    pub kind: String,
    pub lon: f64,
    pub lat: f64,
    pub name: String,
    pub description: Option<String>,
    pub bikes: Option<i32>,
    pub scooters: Option<i32>,
    pub ebikes: Option<i32>,
}
