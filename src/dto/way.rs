use crate::constants;
use crate::constants::{InfrastructureClass, VehicleKind};
use geo_types;
use geojson::Feature;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Way {
    pub id: i64,
    pub source: i64,
    pub x1: f64,
    pub y1: f64,
    pub target: i64,
    pub x2: f64,
    pub y2: f64,
    pub distance: f64,
    pub name: Option<String>,
    pub kind: String,
    pub vehicle_kind: VehicleKind,
    pub class: InfrastructureClass,
    pub geom: Feature,
    pub green: bool,
}

impl Way {
    pub fn new(
        id: i64,
        source: i64,
        x1: f64,
        y1: f64,
        target: i64,
        x2: f64,
        y2: f64,
        distance: f64,
        name: Option<String>,
        highway: &str,
        geo_types_geom: geo_types::Geometry,
        green: bool,
        requested_vehicle_kind: VehicleKind,
    ) -> Self {
        let class = constants::InfrastructureClass::from(highway);
        let vehicle_kind = match class {
            InfrastructureClass::Pedestrian => {
                if requested_vehicle_kind != VehicleKind::None && !green {
                    VehicleKind::None
                } else {
                    requested_vehicle_kind
                }
            }
            InfrastructureClass::Steps => VehicleKind::None,
            // TODO Disallow peds on roads
            _ => requested_vehicle_kind,
        };

        Way {
            id,
            source,
            x1,
            y1,
            target,
            x2,
            y2,
            distance,
            name,
            kind: highway.to_string(),
            vehicle_kind,
            class,
            geom: Feature::from(geojson::Geometry::from(&geo_types_geom)),
            green,
        }
    }
    pub fn swap_source_and_target(&mut self) {
        std::mem::swap(&mut self.source, &mut self.target);
        std::mem::swap(&mut self.x1, &mut self.x2);
        std::mem::swap(&mut self.y1, &mut self.y2);
    }
}
