use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Point {
    pub id: i64,
    pub name: Option<String>,
    pub highway: Option<String>,
    pub green: Option<bool>,
    pub lat: f64,
    pub lon: f64,
}
