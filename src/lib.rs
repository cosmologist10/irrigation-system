#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

extern crate rocket_cors;
use rocket_cors::{Cors, CorsOptions};


mod routes;
mod repository;
mod models;

use repository::mongodb::MongoRepo;



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
    let db = MongoRepo::init();
    // dotenv().ok();
    // rocket::custom(config::from_env());
    rocket::build()
        .manage(db)
        .mount(
            "/app",
            routes![
                routes::measurements::get_all_measurements,
                // routes::irrigations::get_last_month_measurements,
                // routes::irrigations::get_last_week_measurements,
                // routes::irrigations::get_last_day_measurements,
                // routes::irrigations::get_last_hour_measurements,
                // routes::irrigations::get_last_minute_measurements,
                // routes::irrigations::set_measurement,
                // routes::irrigations::get_irrigations,
                // routes::irrigations::get_sensor_names,
                // routes::irrigations::irrigation,
                // routes::irrigations::get_preference,
                // routes::irrigations::update_preferences,
            ],
        )
        // .attach(database::Db::fairing())
        // .attach(cors_fairing())
        // .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}