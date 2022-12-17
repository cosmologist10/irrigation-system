use serde::Deserialize;
use rocket::serde::json::{json, Json, Value};
use rocket::{http::Status, State};

use mongodb::results::InsertOneResult;


use crate::models::measurements::{Measurements, self};
use crate::repository::mongodb::MongoRepo;

// use irrigation::models::measurements;

#[get("/measurement/<sensor_name>")]
pub async fn get_all_measurements(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Measurements>, Status> {
    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    let response = db.get_data(&sensor);
    match response {
        Ok(measurements) => Ok(Json(measurements)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/measurement", data = "<new_measurement>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_measurement: Json<Measurements>,
) -> Result<Json<InsertOneResult>, Status> {
    let new_doc = Measurements {
        sensor_name: new_measurement.sensor_name.to_owned(),
        capacity: new_measurement.capacity.to_owned(),
        timestamp: new_measurement.timestamp.to_owned(),
    };
    let measurement_detail = db.insert_data(new_doc);
    match measurement_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// #[get("/<sensorName>")]
// fn getIrrigations() -> &'static str {
//     "Hello, world!"
// }

// use crate::models::measurements::Measurements;



// #[derive(Deserialize)]
// struct MeasurementQueryFilter {
//     sensor_name: &'a str,
//     start: DateTime<Utc>,
//     end: DateTime<Utc>,
// }

// #[get("/all/<sensor_name>")]
// fn get_all_measurements(sensor_name: &str) -> Value {
//     json!({ "Sensor name": sensor_name })
// }

// #[get("/month/<sensor_name>")]
// fn get_last_month_measurements(sensor_name:&str) -> Value {
//     let query_filter = MeasurementQueryFilter {
//         sensor_name: sensor_name,
//         start: Utc::now(),
//         end: Utc::now() - Duration::month(1)
//     };
//     json!({ "Sensor name": sensor_name })
//     // return daily_measurements with sensor name,lte and gte
// }


// #[get("/week/<sensor_name>")]
// fn get_last_week_measurements(sensor_name: &str) -> Value {
//     let query_filter = MeasurementQueryFilter{
//         sensor_name: sensor_name,
//         start: Utc::now(),
//         end: Utc::now() - Duration::days(7)
//     };
//     json!({ "Sensor name": sensor_name })
//     // return getHourlyMeasurements with sensor name,lte and gte
// }

// #[get("/day/<sensor_name>")]
// fn get_last_day_measurements(sensor_name: &str) -> Value {
//     let query_filter = MeasurementQueryFilter{
//         sensor_name,
//         start: Utc::now(),
//         end: Utc::now() - Duration::days(1)
//     };
//     json!({ "Sensor name": sensor_name })
//     // return getHourlyMeasurements with sensor name,lte and gte
// }

// #[get("/hour/<sensor_name>")]
// fn get_last_hour_measurements(sensor_name: &str) -> Value {
//     let query_filter = MeasurementQueryFilter{
//         sensor_name,
//         start: Utc::now(),
//         end: Utc::now() - Duration::hour(1)
//     };
//     json!({ "Sensor name": sensor_name })
//     // return getMinutelyMeasurements with sensor name,lte and gte
// }

// #[get("/minute/<sensor_name>")]
// fn get_last_minute_measurements(sensor_name: &str) -> Value {
//     let query_filter = MeasurementQueryFilter{
//         sensor_name,
//         start: Utc::now(),
//         end: Utc::now() - Duration::hour(1)
//     };
//     json!({ "Sensor name": sensor_name })
//     // return getSecondlyMeasurements with sensor name,lte and gte
// }



// #[get("/<sensorName>")]
// fn getPreference() -> &'static str {
//     "Hello, world!"
// }

// #[put("/<sensorName>")]
// fn updatePreferences() -> &'static str {
//     "Hello, world!"
// }



// #[get("/")]
// fn getSensorNames() -> &'static str {
//     "Hello, world!"
// }

// #[get("/irrigation/<sensorName>")]
// fn irrigation() -> &'static str {
//     "Hello, world!"
// }



