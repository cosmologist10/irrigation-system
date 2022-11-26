// use std::any;

use rocket::serde::json::{json, Value};
use chrono::{DateTime, Duration, Utc};

use rocket::State;
use serde::Deserialize;
use chrono::prelude::*;

#[derive(Deserialize)]
struct MeasurementQueryFilter {
    sensor_name: &'a str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}


#[get("/all/<sensor_name>")]
fn get_all_measurements(sensor_name: &str) -> Value {
    json!({ "Sensor name": sensor_name })
}

#[get("/month/<sensor_name>")]
fn get_last_month_measurements(sensor_name:&str) -> Value {
    let query_filter = MeasurementQueryFilter {
        sensor_name: sensor_name,
        start: Utc::now(),
        end: Utc::now() - Duration::month(1)
    };
    json!({ "Sensor name": sensor_name })
    // return daily_measurements with sensor name,lte and gte
}


#[get("/week/<sensor_name>")]
fn get_last_week_measurements(sensor_name: &str) -> Value {
    let query_filter = MeasurementQueryFilter{
        sensor_name: sensor_name,
        start: Utc::now(),
        end: Utc::now() - Duration::days(7)
    };
    json!({ "Sensor name": sensor_name })
    // return getHourlyMeasurements with sensor name,lte and gte
}

#[get("/day/<sensor_name>")]
fn get_last_day_measurements(sensor_name: &str) -> Value {
    let query_filter = MeasurementQueryFilter{
        sensor_name,
        start: Utc::now(),
        end: Utc::now() - Duration::days(1)
    };
    json!({ "Sensor name": sensor_name })
    // return getHourlyMeasurements with sensor name,lte and gte
}

#[get("/hour/<sensor_name>")]
fn get_last_hour_measurements(sensor_name: &str) -> Value {
    let query_filter = MeasurementQueryFilter{
        sensor_name,
        start: Utc::now(),
        end: Utc::now() - Duration::hour(1)
    };
    json!({ "Sensor name": sensor_name })
    // return getMinutelyMeasurements with sensor name,lte and gte
}

#[get("/minute/<sensor_name>")]
fn get_last_minute_measurements(sensor_name: &str) -> Value {
    let query_filter = MeasurementQueryFilter{
        sensor_name,
        start: Utc::now(),
        end: Utc::now() - Duration::hour(1)
    };
    json!({ "Sensor name": sensor_name })
    // return getSecondlyMeasurements with sensor name,lte and gte
}
