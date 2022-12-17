
use std::env;
extern crate dotenv;
use dotenv::dotenv;
use chrono::{DateTime, Utc, Duration};


use mongodb::{
    bson::{extjson::de::Error},
    bson::{doc},
    results::{ InsertOneResult},
    sync::{Client, Collection},
};
use crate::models::measurements::Measurements;

pub struct MongoRepo {
    daily_measurements: Collection<Measurements>,
    // minutely_measurements: Collection<Measurements>,
    // secondly_measurements: Collection<Measurements>,
    // hourly_measurements: Collection<Measurements>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGODB_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("irrigation");
        let daily_measurements: Collection<Measurements> = db.collection("daily_measurements");
        MongoRepo { daily_measurements }
    }

    pub fn get_data(&self, sensor: &String) -> Result<Measurements, Error> {
        let filter = doc! {"sensor_name": sensor};
        println!("filter: {} {:?}", sensor, filter);
        let sensor_detail = self
            .daily_measurements
            .find_one(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        println!("response:{:?}", sensor_detail);
        Ok(sensor_detail.unwrap())
    }

    pub fn insert_data(&self, new_measurement: Measurements) -> Result<InsertOneResult, Error> {

        let user = self
            .daily_measurements
            .insert_one(new_measurement, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
}
