#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

extern crate rocket_cors;
use rocket_cors::{Cors, CorsOptions};

use dotenv::dotenv;

mod routes;
mod database;
mod config;


#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn cors_fairing() -> Cors {
    CorsOptions::default()
        .to_cors()
        .expect("Cors fairing cannot be created")
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::custom(config::from_env())
    rocket::build()
        .mount(
            "/app",
            routes![
                routes::measurements::get_all_measurements,
                routes::measurements::get_last_month_measurements,
                routes::measurements::get_last_week_measurements,
                routes::measurements::get_last_day_measurements,
                routes::measurements::get_last_hour_measurements,
                routes::measurements::get_last_minute_measurements,
                // routes::measurements::set_measurement,
                routes::irrigations::get_irrigations,
                routes::sensors::get_sensor_names,
                routes::sensors::irrigation,
                routes::preferences::get_preference,
                routes::preferences::update_preferences,
            ],
        )
        .attach(database::Db::fairing())
        .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}