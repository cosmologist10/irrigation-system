use rocket::serde::json::{json, Json, Value};
use rocket::State;
use serde::Deserialize;
use chrono::prelude::*;



#[get("/all/<sensor_name>")]
fn get_all_measurements(sensor_name: &str) -> String {
    let query_filter = {
        sensor_name
    }
    format!("Sensor name : {}!", sensor_name);
    // return daily_measurements with sensor name
}

#[get("/month/<sensorName>")]
fn get_last_month_measurements(sensor_name: &str) -> String {
    let query_filter = {
        sensor_name,
        let start: DateTime<Utc> = Utc::now(),
        let end: DateTime<Utc> = Utc::now() - Duration::month(1),
    }
    format!("Sensor name : {}!", query_filter);
    // return daily_measurements with sensor name,lte and gte
}


#[get("/week/<sensorName>")]
fn get_last_week_measurements(sensor_name: &str) -> String {
    let query_filter = {
        sensor_name,
        let start: DateTime<Utc> = Utc::now(),
        let end: DateTime<Utc> = Utc::now() - Duration::days(7)
    }
    format!("Sensor name : {}!", sensor_name);
    // return getHourlyMeasurements with sensor name,lte and gte
}

#[get("/day/<sensorName>")]
fn get_last_day_measurements(sensor_name: &str) -> String {
    let query_filter = {
        sensor_name,
        let start: DateTime<Utc> = Utc::now(),
        let end: DateTime<Utc> = Utc::now() - Duration::days(1)
    }
    format!("Sensor name : {}!", sensor_name);
    // return getHourlyMeasurements with sensor name,lte and gte
}

#[get("/hour/<sensorName>")]
fn get_last_hour_measurements(sensor_name: &str) -> String {
    let query_filter = {
        sensor_name,
        let start: DateTime<Utc> = Utc::now(),
        let end: DateTime<Utc> = Utc::now() - Duration::hour(1)
    }
    format!("Sensor name : {}!", sensor_name);
    // return getMinutelyMeasurements with sensor name,lte and gte
}

#[get("/minute/<sensorName>")]
fn get_last_minute_measurements(sensor_name: &str) -> String {
    let query_filter = {
        sensor_name,
        let start: DateTime<Utc> = Utc::now(),
        let end: DateTime<Utc> = Utc::now() - Duration::hour(1)
    }
    format!("Sensor name : {}!", sensor_name);
    // return getSecondlyMeasurements with sensor name,lte and gte
}
