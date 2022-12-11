use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Measurements {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    sensor_name: String,
    capacity: i32,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct preference {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    sensor_name: String,
    irrigation_time_in_seconds: i32,
    min_irrigation_interval_in_minutes: i32,
    capacity_buffer: i32,
    signal_pin: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct sensors {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    sensor_name: String,
}