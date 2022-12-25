#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

// extern crate rocket_cors;
// use rocket_cors::{Cors, CorsOptions};

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;


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

// fn cors_fairing() -> Cors {
//     CorsOptions::default()
//         .to_cors()
//         .expect("Cors fairing cannot be created")
// }



#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
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
                routes::measurements::set_measurement,
                routes::measurements::get_all_measurements,
                routes::measurements::get_last_month_measurements,
                routes::measurements::get_last_day_measurements,
                routes::measurements::get_last_week_measurements,
                routes::measurements::get_last_hour_measurements,
                routes::measurements::get_last_minute_measurements,
                routes::measurements::get_irrigations,
                routes::measurements::get_preference,
                routes::measurements::update_preference,
                routes::measurements::get_sensors,
                routes::measurements::irrigation,
            ],
        )
        .attach(CORS)
        // .attach(database::Db::fairing())
        // .attach(cors_fairing())
        // .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}