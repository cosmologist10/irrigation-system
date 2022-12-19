use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurements {
    pub sensor_name: String,
    pub capacity: i32,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestMeasurements {
    pub sensor_name: String,
    pub capacity: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preference {
    pub sensor_name: String,
    pub irrigation_time_in_seconds: i32,
    pub min_irrigation_interval_in_minutes: i32,
    pub capacity_buffer: i32,
    pub signal_pin: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sensors{
    pub sensor_name: String,
}
