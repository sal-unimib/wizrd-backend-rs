use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::constants::VehicleKind;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vehicle {
    pub kind: VehicleKind,
    pub speed: f64,
    pub cost: f64,
    pub green: i32,
}
