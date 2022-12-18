
use std::env;
extern crate dotenv;
use dotenv::dotenv;
use chrono::{DateTime, Utc, Duration};


use mongodb::{
    bson::{extjson::de::Error},
    bson::{doc},
    results::{ InsertOneResult, UpdateResult},
    sync::{Client, Collection},
    options::{FindOptions, UpdateOptions},
};
use rocket::http::private::cookie::Expiration;
use crate::models::measurements::{Measurements, self};



pub struct MongoRepo {
    daily_measurements: Collection<Measurements>,
    minutely_measurements: Collection<Measurements>,
    secondly_measurements: Collection<Measurements>,
    hourly_measurements: Collection<Measurements>,
    irrigation: Collection<Measurements>
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
        let minutely_measurements: Collection<Measurements> = db.collection("minutely_measurements");
        let hourly_measurements: Collection<Measurements> = db.collection("hourly_measurements");
        let secondly_measurements: Collection<Measurements> = db.collection("secondly_measurements");
        let irrigation: Collection<Measurements> = db.collection("irrigation");

        MongoRepo { daily_measurements, secondly_measurements, minutely_measurements, 
            hourly_measurements, irrigation }
    }

    // apply all check here
    pub fn get_all_measurements(&self, sensor: &String) -> Result<Measurements, Error> {
        let filter = doc! {"sensor_name": sensor};
        println!("filter: {} {:?}", sensor, filter);
        let sensor_detail = self
            .daily_measurements
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        println!("response:{:?}", sensor_detail);
        Ok(sensor_detail.unwrap())
    }

    // apply all check here filter issue
    pub fn get_last_month_measurements(&self, sensor: &String) -> Result<Measurements, Error> {
        let filter = doc! {
        "sensor_name": sensor, 
        "timestamp": {
            $lte: Utc::now(),
            $gte: Utc::now() - Duration::days(30)
        }
    };
        let sensor_detail = self
            .daily_measurements
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        println!("response:{:?}", sensor_detail);
        Ok(sensor_detail.unwrap())
    }

    // apply all check here filter issue
    pub fn get_last_day_measurements(&self, sensor: &String) -> Result<Measurements, Error> {
        let filter = doc! {
        "sensor_name": sensor, 
        "timestamp": {
            $lte: Utc::now(),
            $gte: Utc::now() - Duration::days(1)
        }
    };
        let sensor_detail = self
            .daily_measurements
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        println!("response:{:?}", sensor_detail);
        Ok(sensor_detail.unwrap())
    }

    // apply all check here filter issue
    pub fn get_last_week_measurements(&self, sensor: &String) -> Result<Measurements, Error> {
        let filter = doc! {
        "sensor_name": sensor, 
        "timestamp": {
            $lte: Utc::now(),
            $gte: Utc::now() - Duration::days(7)
        }
    };
        let sensor_detail = self
            .daily_measurements
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        println!("response:{:?}", sensor_detail);
        Ok(sensor_detail.unwrap())
    }

    pub fn get_irrigation(&self, sensor: &String) -> Result<Measurements, Error> {
        let filter = doc! {
            "sensor_name": sensor, 
        };
        let irrigation_detail = self
            .irrigation
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        println!("response:{:?}", irrigation_detail);
        Ok(irrigation_detail.unwrap())
    }

    // find a way to pass options as parameters and date format
    pub fn set_measurement(&self, new_measurement: Measurements) -> Result<UpdateResult, Error> {

        let current_date = new_measurement.timestamp.format("%Y-%d-%mT%H:%M:SS %z");
        let current_day = new_measurement.timestamp.format("%Y-%d-%mT%00:00:00 %z");
        let current_hour = new_measurement.timestamp.format("%Y-%d-%mT%H:00:00 %z");
        let current_minute = new_measurement.timestamp.format("%Y-%d-%mT%H:%M:00 %z");

        
        let updated_daily = self
            .daily_measurements
            .update_one(
                doc! {"timestamp": new_measurement.timestamp},
                doc! {
                    "$set":
                    {
                        "sensor_name": new_measurement.sensor_name.to_owned(),
                        "capacity": new_measurement.capacity
                    },
                }, 
             None,
            )
            .ok()
            .expect("Error updating daily measurements");

        let hourly_minutely = self
            .hourly_measurements
            .update_one(
                doc! {"timestamp": new_measurement.timestamp},
                doc! {
                    "$set":
                    {
                        "sensor_name": new_measurement.sensor_name.to_owned(),
                        "capacity": new_measurement.capacity
                    },
                },
             None,
            )
            .ok()
            .expect("Error updating hourly measurements");

        let updated_minutely = self
            .minutely_measurements
            .update_one(
                doc! {"timestamp": new_measurement.timestamp},
                doc! {
                    "$set":
                    {
                        "sensor_name": new_measurement.sensor_name.to_owned(),
                        "capacity": new_measurement.capacity
                    },
                }, 
             None,
            )
            .ok()
            .expect("Error updating minutely measurements");

        let updated_secondly = self
            .secondly_measurements
            .update_one(
                doc! {"timestamp": new_measurement.timestamp},
                doc! {
                        "$set":
                        {
                            "sensor_name": new_measurement.sensor_name.to_owned(),
                            "capacity": new_measurement.capacity
                        },
                }, 
            None,
            )
            .ok()
            .expect("Error updating secondly measurements");

        Ok(updated_secondly)
    }
}


