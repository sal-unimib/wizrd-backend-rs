use crate::constants::VehicleKind;
use crate::dto::Way;
use crate::model::{Point, Vehicle};
use crate::service;
use geojson::Feature;
use geojson::Geometry;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub start: Point,
    pub goal: Point,
    pub real_start: Feature,
    pub real_goal: Feature,
    pub ways: Vec<Way>,
}

impl Route {
    pub fn new(
        start: Point,
        goal: Point,
        real_start_coords: &Vec<f64>,
        real_goal_coords: &Vec<f64>,
        mut ways: Vec<Way>,
    ) -> Self {
        /*
        let real_start = Feature::from(Geometry::from(geojson::Value::LineString(vec![
            vec![real_start_coords[1], real_start_coords[0]],
            vec![start.lon, start.lat],
        ])));
        let real_goal = Feature::from(Geometry::from(geojson::Value::LineString(vec![
            vec![real_goal_coords[1], real_goal_coords[0]],
            vec![goal.lon, goal.lat],
        ])));*/

        let real_start = Self::feature_from_coords(
            real_start_coords[1],
            real_start_coords[0],
            start.lon,
            start.lat,
        );

        let real_goal =
            Self::feature_from_coords(real_goal_coords[1], real_goal_coords[0], goal.lon, goal.lat);

        let sorted_ways = Self::sort_ways(start.id, goal.id, &mut ways);

        Self {
            start,
            goal,
            real_start,
            real_goal,
            ways: sorted_ways,
        }
    }

    fn feature_from_coords(lon1: f64, lat1: f64, lon2: f64, lat2: f64) -> Feature {
        Feature::from(Geometry::from(geojson::Value::LineString(vec![
            vec![lon1, lat1],
            vec![lon2, lat2],
        ])))
    }

    pub fn _total_distance(&self) -> f64 {
        self.ways.iter().map(|way| way.distance).sum()
    }

    pub fn total_time(&self, vehicles: &HashMap<String, Vehicle>) -> f64 {
        self.ways
            .iter()
            .map(|way| {
                let veh = vehicles.get(&way.vehicle_kind.to_string()).unwrap(); // XXX We assume the Vehicle exists!
                way.distance / veh.speed
            })
            .sum()
    }

    fn add_hop(hop_id: i64, from: &mut Vec<Way>, to: &mut Vec<Way>) -> Result<i64, String> {
        let pos = from
            .iter()
            .position(|way| way.source == hop_id || way.target == hop_id);

        match pos {
            Some(index) => {
                let mut way = from.remove(index);
                if way.target == hop_id {
                    way.swap_source_and_target();
                }
                let next_hop = way.target;
                to.push(way);
                Ok(next_hop)
            }
            None => Err(format!("Unable to find Way {} in Route", hop_id)),
        }
    }

    fn sort_ways(source_id: i64, destination_id: i64, ways: &mut Vec<Way>) -> Vec<Way> {
        if ways.is_empty() {
            return Vec::new();
        }

        let mut sorted_ways = Vec::new();
        let mut next_hop_id = source_id;

        while next_hop_id != destination_id {
            match Self::add_hop(next_hop_id, ways, &mut sorted_ways) {
                Ok(next) => next_hop_id = next,
                Err(_e) => {
                    eprintln!("{:?}", _e);
                    break;
                }
            }
        }

        sorted_ways
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteContainer {
    pub routes: Vec<Route>,
    pub requested_vehicle_kind: VehicleKind,
    pub is_fallback: bool,
}

impl RouteContainer {
    pub fn new(routes: Vec<Route>, requested_vehicle_kind: VehicleKind, is_fallback: bool) -> Self {
        Self {
            routes,
            requested_vehicle_kind,
            is_fallback,
        }
    }

    pub fn _total_distance(&self) -> f64 {
        self.routes
            .iter()
            .map(|route| route._total_distance())
            .sum()
    }

    pub async fn total_time_min(&self, conn: &mut PgConnection) -> Result<f64, Status> {
        let tms = service::vehicle::get_all(conn).await?;
        let time = self.routes.iter().map(|route| route.total_time(&tms)).sum();
        Ok(time)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRequest {
    pub start_id: i64,
    pub goal_id: i64,

    pub real_start_coords: Vec<f64>,
    pub real_goal_coords: Vec<f64>,

    pub vehicle_kind: VehicleKind,
    pub routing_algorithm_kind: String,

    pub consider_air_quality: Option<bool>,
    pub consider_distance: Option<bool>,
    pub consider_green_areas: Option<bool>,
    pub consider_mobility_impairment: Option<bool>,
    pub consider_safety: Option<bool>,
    pub consider_traffic: Option<bool>,

    pub air_quality_level: i32,
    pub traffic_level: i32,
}

impl RouteRequest {
    pub fn set_real_start_coords(&mut self, lat: f64, lon: f64) {
        self.real_start_coords = vec![lat, lon];
    }

    pub fn set_real_goal_coords(&mut self, lat: f64, lon: f64) {
        self.real_goal_coords = vec![lat, lon];
    }
}
