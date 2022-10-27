use rocket::serde::json::{json, Json, Value};
use rocket::State;
use serde::Deserialize;


#[get("/all/<sensor_name>")]
fn get_all_measurements(sensor_name: &str) -> String {
    format!("Sensor name : {}!", sensor_name)
    // res.json(await measurementService.getDailyMeasurements(queryFilter));
}

#[get("/month/<sensorName>")]
fn get_last_month_measurements(sensor_name: &str) -> String {
    format!("Sensor name : {}!", sensor_name)
}

#[get("/week/<sensorName>")]
fn get_last_week_measurements(sensor_name: &str) -> String {
    format!("Sensor name : {}!", sensor_name)
}

#[get("/day/<sensorName>")]
fn get_last_day_measurements(sensor_name: &str) -> String {
    format!("Sensor name : {}!", sensor_name)
}

#[get("/hour/<sensorName>")]
fn get_last_hour_measurements(sensor_name: &str) -> String {
    format!("Sensor name : {}!", sensor_name)
}

#[get("/minute/<sensorName>")]
fn get_last_minute_measurements(sensor_name: &str) -> String {
    format!("Sensor name : {}!", sensor_name)
}



// #[post("/<sensorName>")]
// fn setMeasurement() -> &'static str {
//     "Hello, world!"
// }