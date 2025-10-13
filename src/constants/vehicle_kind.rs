use rocket::serde::{Deserialize, Serialize};
use std::fmt;
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "vehicle_kind", rename_all = "lowercase")]
pub enum VehicleKind {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "bike")]
    Bike,

    #[serde(rename = "ebike")]
    Ebike,

    #[serde(rename = "scooter")]
    Scooter,
}

impl VehicleKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "pedestrian",
            Self::Bike => "bike",
            Self::Ebike => "ebike",
            Self::Scooter => "scooter",
        }
    }
}

impl fmt::Display for VehicleKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for VehicleKind {
    fn from(s: &str) -> Self {
        match s {
            "bike" => Self::Bike,
            "ebike" => Self::Ebike,
            "scooter" => Self::Scooter,
            "none" => Self::None,
            _ => panic!("Unexpected Vehicle Kind: {}", s),
        }
    }
}
