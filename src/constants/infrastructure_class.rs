use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfrastructureClass {
    Cycling,
    Mixed,
    Pedestrian,
    Road,
    Steps,
}

impl From<&str> for InfrastructureClass {
    fn from(highway: &str) -> Self {
        // Refers to the "highway" field in the OSM dataset
        match highway {
            "cycling" | "cycleway" | "cycle_crossing" => Self::Cycling,
            "footway" | "footpath" | "pedestrian" | "foot_crossing" => Self::Pedestrian,
            "road" | "primary" | "primary_link" | "secondary" | "secondary_link"
            | "tertiary" | "tertiary_link" => Self::Road,
            "steps" => Self::Steps,
            "living_street" | "path" | "residential" | "service" | "track" | "unclassified" => {
                Self::Mixed
            }
            _ => Self::Mixed,
        }
    }
}

impl InfrastructureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cycling => "cycling",
            Self::Mixed => "mixed",
            Self::Pedestrian => "pedestrian",
            Self::Road => "road",
            Self::Steps => "steps",
        }
    }
}

impl fmt::Display for InfrastructureClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}