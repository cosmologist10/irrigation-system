
use std::{env, any};
extern crate dotenv;
use dotenv::dotenv;
use chrono::{DateTime, Utc, Duration, Timelike};



use mongodb::{
    bson::{extjson::de::Error},
    bson::{doc},
    results::{ InsertOneResult, UpdateResult},
    sync::{Client, Collection},
    options::{FindOptions, UpdateOptions, IndexOptions},
};
use rocket::response;

use crate::models::measurements::{Measurements, Preference};


pub struct MongoRepo {
    daily_measurements: Collection<Measurements>,
    minutely_measurements: Collection<Measurements>,
    secondly_measurements: Collection<Measurements>,
    hourly_measurements: Collection<Measurements>,
    irrigation: Collection<Measurements>,
    preferences: Collection<Preference>
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
        let preferences: Collection<Preference> = db.collection("preferences");

        MongoRepo { daily_measurements, secondly_measurements, minutely_measurements, 
            hourly_measurements, irrigation, preferences }
    }

    pub fn change_type(&self, v: u32) -> i64 {
        i64::from(v) // or v.into()
    }

    // TODO: can setup expire_after while index creation
    pub fn set_measurement(&self, new_measurement: Measurements) -> Result<UpdateResult, Error> {
        let mut current_day = new_measurement.timestamp;
        let hour =  self.change_type(current_day.hour());
        let minute =  self.change_type(current_day.minute());
        let second =  self.change_type(current_day.second());
        let timezone = self.change_type(current_day.timestamp_subsec_nanos());

        current_day =  current_day - Duration::nanoseconds(timezone);
        self.update_secondly_measurement(new_measurement.sensor_name.to_owned(), new_measurement.capacity, current_day);

        current_day =  current_day - Duration::seconds(second);
        self.update_hourly_measurement(new_measurement.sensor_name.to_owned(), new_measurement.capacity, current_day);
    
        current_day =  current_day - Duration::hours(hour);
        self.update_minutely_measurement(new_measurement.sensor_name.to_owned(), new_measurement.capacity, current_day);
        
        current_day =  current_day - Duration::minutes(minute);
        let updated_daily = self
                .daily_measurements
                .update_one(
                    doc! {"timestamp": current_day},
                    doc! {
                            "$set":
                            {
                                "sensor_name": new_measurement.sensor_name.to_owned(),
                                "capacity": new_measurement.capacity
                            },
                    },
                    Some(UpdateOptions::builder().upsert(true).build()),
                )
                .ok()
                .expect("Error updating daily measurements");
    
        Ok(updated_daily)
    }

    // update hourly measurements
    pub fn update_hourly_measurement(&self, sensor: String, capacity: i32, timestamp: DateTime<Utc>) -> Result<UpdateResult, Error> {
        let updated_hourly = self
                .hourly_measurements
                .update_one(
                    doc! {"timestamp": timestamp},
                    doc! {
                            "$set":
                            {
                                "sensor_name": sensor.to_owned(),
                                "capacity": capacity
                            },
                    }, 
                    Some(UpdateOptions::builder().upsert(true).build()),
                )
                .ok()
                .expect("Error updating secondly measurements");
    
        Ok(updated_hourly)
    }

    // update minutely measurements
    pub fn update_minutely_measurement(&self, sensor: String, capacity: i32, timestamp: DateTime<Utc>) -> Result<UpdateResult, Error> {
        let updated_minutely = self
                .minutely_measurements
                .update_one(
                    doc! {"timestamp": timestamp},
                    doc! {
                            "$set":
                            {
                                "sensor_name": sensor.to_owned(),
                                "capacity": capacity
                            },
                    }, 
                    Some(UpdateOptions::builder().upsert(true).build()),
                )
                .ok()
                .expect("Error updating minutely measurements");
    
        Ok(updated_minutely)
    }

    // update secondly measurements
    pub fn update_secondly_measurement(&self, sensor: String, capacity: i32, timestamp: DateTime<Utc>) -> Result<UpdateResult, Error> {
        let updated_secondly = self
                .secondly_measurements
                .update_one(
                    doc! {"timestamp": timestamp},
                    doc! {
                            "$set":
                            {
                                "sensor_name": sensor.to_owned(),
                                "capacity": capacity
                            },
                    }, 
                    Some(UpdateOptions::builder().upsert(true).build()),
                )
                .ok()
                .expect("Error updating secondly measurements");
    
        Ok(updated_secondly)
    }
    
    pub fn get_all_measurements(&self, sensor: &String) -> Result<Vec<Measurements>, Error> {
        let filter = doc! {"sensor_name": sensor};
        println!("filter: {} {:?}", sensor, filter);
        let sensor_detail = self
            .daily_measurements
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");

        let mut response : Vec<Measurements> = Vec::new();
            
        for doc in sensor_detail {
            response.push(doc.unwrap())
        };
        
        Ok(response)
    }

    pub fn get_last_month_measurements(&self, sensor: &String) -> Result<Vec<Measurements>, Error> {
        let filter = doc! {
                "sensor_name": sensor, 
                "timestamp": Utc::now() - Duration::days(30)
        };
        let sensor_detail = self
            .daily_measurements
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        println!("response:{:?}", sensor_detail);
        
        let mut response : Vec<Measurements> = Vec::new();
            
        for doc in sensor_detail {
            response.push(doc.unwrap())
        };
        
        Ok(response)
    }

    pub fn get_last_day_measurements(&self, sensor: &String) -> Result<Vec<Measurements>, Error> {
        let filter = doc! {
            "sensor_name": sensor, 
            "timestamp": {
                "$lte" : Utc::now(),
                "$gte" : Utc::now() - Duration::days(1),
            }
        };
        let sensor_detail = self
            .hourly_measurements
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        
 
        let mut response : Vec<Measurements> = Vec::new();
            
        for doc in sensor_detail {
            response.push(doc.unwrap())
        };
        
        Ok(response)
    }

    pub fn get_last_week_measurements(&self, sensor: &String) -> Result<Vec<Measurements>, Error> {
        let filter = doc! {
            "sensor_name": sensor, 
            "timestamp": {
                "$lte" : Utc::now(),
                "$gte" : Utc::now() - Duration::weeks(1)
            }
        };
        let sensor_detail = self
            .hourly_measurements
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        
        let mut response : Vec<Measurements> = Vec::new();
            
        for doc in sensor_detail {
            response.push(doc.unwrap())
        };
        
        Ok(response)
    }

    pub fn get_last_hour_measurements(&self, sensor: &String) -> Result<Vec<Measurements>, Error> {
        let filter = doc! {
            "sensor_name": sensor, 
            "timestamp": {
                "$lte" : Utc::now(),
                "$gte" : Utc::now() - Duration::hours(1)
            }
        };
        let sensor_detail = self
            .minutely_measurements
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        
        let mut response : Vec<Measurements> = Vec::new();
            
        for doc in sensor_detail {
            response.push(doc.unwrap())
        };
        
        Ok(response)
    }

    pub fn get_last_minute_measurements(&self, sensor: &String) -> Result<Vec<Measurements>, Error> {
        let filter = doc! {
            "sensor_name": sensor, 
            "timestamp": {
                "$lte" : Utc::now(),
                "$gte" : Utc::now() - Duration::minutes(1)
            }
        };
        let sensor_detail = self
            .secondly_measurements
            .find(filter, None)
            .ok()
            .expect("Error getting sensor's detail");
        
        let mut response : Vec<Measurements> = Vec::new();
            
        for doc in sensor_detail {
            response.push(doc.unwrap())
        };
        
        Ok(response)
    }

    // TODO: Implement find here
    pub fn get_irrigations(&self, sensor: &String) -> Result<Measurements, Error> {
        let filter = doc! {
            "sensor_name": sensor, 
        };
        let irrigation_detail = self
            .irrigation
            .find_one(filter, None)
            .ok()
            .expect("Error getting irrigations detail");
        println!("response:{:?}", irrigation_detail);
        Ok(irrigation_detail.unwrap())
    }


    // // TODO: Implement find here
    // pub fn get_sensor_names(&self) -> Result<Preference, Error>{
    //     let preference = self
    //         .preferences
    //         .find_one({}, None)
    //         .ok()
    //         .expect("Error updating daily measurements");
    //     Ok(preference.unwrap())
    // }


    // pub fn irrigation(&self, sensor: &String) -> any{
    //     let preferences = self.get_preference(sensor);
    //     let sensor_result = self.irrigate(irrigation_time_in_seconds, sensor);
    //     if(sensor_result){
    //         self.set_irrigation();
    //     }
    // }

    // // get preference
    // // TODO: pass options
    // pub fn get_preference(&self, sensor: &String) -> Result<Preference, Error>{
    //     let preference = self
    //         .preferences
    //         .find_one_and_update(
    //             doc! {"sensor_name": sensor},
    //             doc! {
    //                 "$set":
    //                 {
    //                     "irrigation_time_in_seconds": 15,
    //                     "min_irrigation_interval_in_minutes": 10,
    //                     "capacity_buffer": 500,
    //                     "signal_pin": 18
    //                 },
    //             }, 
    //          None,
    //         )
    //         .ok()
    //         .expect("Error updating daily measurements");
    //         Ok(preference.unwrap())
    // }

    // // update preference
    // // TODO: pass options
    // pub fn update_preference(&self, preferences: Preference) -> Result<Preference, Error> {

    //     let preference = self
    //         .preferences
    //         .find_one_and_update(
    //             doc! {"sensor_name": preferences.sensor_name.to_owned()},
    //             doc! {
    //                 "$set":
    //                 preferences,
    //             }, 
    //          None,
    //         )
    //         .ok()
    //         .expect("Error updating daily measurements");
    //         Ok(preference.unwrap())
    // }
}


