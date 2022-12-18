
use serde::Deserialize;
use rocket::serde::json::{json, Json, Value};
use rocket::{http::Status, State};

use chrono::{DateTime, Utc, Duration};


use mongodb::results::{InsertOneResult, UpdateResult};


use crate::models::measurements::{Measurements, RequestMeasurements};
use crate::repository::mongodb::MongoRepo;

// use irrigation::models::measurements;

#[get("/all_measurement/<sensor_name>")]
pub async fn get_all_measurements(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Measurements>, Status> {
    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    let response = db.get_all_measurements(&sensor);
    match response {
        Ok(measurements) => Ok(Json(measurements)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_last_month_measurements/<sensor_name>")]
pub async fn get_last_month_measurements(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Measurements>, Status> {
    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    let response = db.get_last_month_measurements(&sensor);
    match response {
        Ok(measurements) => Ok(Json(measurements)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_last_day_measurements/<sensor_name>")]
pub async fn get_last_day_measurements(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Measurements>, Status> {
    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    let response = db.get_last_day_measurements(&sensor);
    match response {
        Ok(measurements) => Ok(Json(measurements)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_last_week_measurements/<sensor_name>")]
pub async fn get_last_week_measurements(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Measurements>, Status> {
    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    let response = db.get_last_week_measurements(&sensor);
    match response {
        Ok(measurements) => Ok(Json(measurements)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_last_hour_measurements/<sensor_name>")]
pub async fn get_last_hour_measurements(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Measurements>, Status> {
    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    let response = db.get_last_hour_measurements(&sensor);
    match response {
        Ok(measurements) => Ok(Json(measurements)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_last_minute_measurements/<sensor_name>")]
pub async fn get_last_minute_measurements(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Measurements>, Status> {
    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    let response = db.get_last_minute_measurements(&sensor);
    match response {
        Ok(measurements) => Ok(Json(measurements)),
        Err(_) => Err(Status::InternalServerError),
    }
}


// TODO: Add rate
#[post("/measurement", data = "<new_measurement>")]
pub fn set_measurement(
    db: &State<MongoRepo>,
    new_measurement: Json<RequestMeasurements>,
) -> Result<Json<UpdateResult>, Status> {
    let now: DateTime<Utc> = Utc::now();

    let new_doc = Measurements {
        sensor_name: new_measurement.sensor_name.to_owned(),
        capacity: new_measurement.capacity.to_owned(),
        timestamp: now,
    };
    let measurement_detail = db.set_measurement(new_doc);
    match measurement_detail {
        Ok(measurement) => Ok(Json(measurement)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[get("/get_irrigations/<sensor_name>")]
pub fn get_irrigations(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Measurements>, Status> {
    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    let response = db.get_irrigations(&sensor);
    match response {
        Ok(measurements) => Ok(Json(measurements)),
        Err(_) => Err(Status::InternalServerError),
    }
}



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



