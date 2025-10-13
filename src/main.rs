mod constants;
mod controller;
mod db;
mod dto;
mod model;
mod repository;
mod service;

use db::Db;
use rocket::http::Method;
use rocket::{launch, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_db_pools::Database;

#[launch]
async fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error creating CORS");

    rocket::build().attach(cors).attach(Db::init()).mount(
        "/wizrd",
        routes![
            controller::poi::get_all,
            controller::poi::get_nearest,
            controller::point::get_nearest_point,
            controller::point::get_point_by_id,
            controller::route::post_lumi_route,
            controller::route::post_route,
            controller::topology::get_all_topology,
            controller::topology::get_topology,
            controller::vehicle::get_vehicle,
            controller::vehicle::get_vehicles,
        ],
    )
}
