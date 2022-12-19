
use serde::Deserialize;
use rocket::serde::json::{json, Json, Value};
use rocket::{http::Status, State};

use chrono::{DateTime, Utc, Duration};


use mongodb::results::{InsertOneResult, UpdateResult};


use crate::models::measurements::{Measurements, RequestMeasurements, Preference};
use crate::repository::mongodb::MongoRepo;

// use irrigation::models::measurements;

#[get("/all_measurement/<sensor_name>")]
pub async fn get_all_measurements(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Vec<Measurements>>, Status> {
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
) -> Result<Json<Vec<Measurements>>, Status> {
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
) -> Result<Json<Vec<Measurements>>, Status> {
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
) -> Result<Json<Vec<Measurements>>, Status> {
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
) -> Result<Json<Vec<Measurements>>, Status> {
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
) -> Result<Json<Vec<Measurements>>, Status> {
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


// TODO: Add rate limiter
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


#[get("/irrigations/<sensor_name>")]
pub fn get_irrigations(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Vec<Measurements>>, Status> {
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

#[get("/sensors")]
pub fn get_sensors(
    db: &State<MongoRepo>,
) -> Result<Json<Vec<String>>, Status> {
    
    let response = db.get_sensor_names();
    
    match response {
        Ok(preferences) => Ok(Json(preferences)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/sensors/irrigation/<sensor_name>")]
pub fn irrigation(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<InsertOneResult>, Status> {

    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    
    let response = db.irrigation(sensor);
    
    match response {
        Ok(id) => Ok(Json(id)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/preferences/<sensor_name>")]
pub fn get_preference(
    db: &State<MongoRepo>,
    sensor_name: String
) -> Result<Json<Preference>, Status> {
    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    let response = db.get_preference(sensor);
    match response {
        Ok(preferences) => Ok(Json(preferences)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/preferences/<sensor_name>", format = "json", data = "<preference>")]
pub fn update_preference(
    db: &State<MongoRepo>,
    sensor_name: String,
    preference: Json<Preference>,
) -> Result<Json<Preference>, Status> {
    let sensor = sensor_name;
    if sensor.is_empty() {
        return Err(Status::BadRequest);
    };
    let response = db.update_preference(preference);
    match response {
        Ok(preferences) => Ok(Json(preferences)),
        Err(_) => Err(Status::InternalServerError),
    }
}

