use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error},
    results::{ InsertOneResult},
    sync::{Client, Collection},
};
use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub fn get_all_measurements(&self, sensor_name: String) -> Result<Measurements, Error> {
        let all_measurements = irrigation
            .find_one(Some(doc! { "sensor_name":  sensor_name }), None)
            .await?
            .expect("Data not found");

        // Deserialize the document into a Movie instance
        let loaded_all_measurements_struct: Measurements = bson::from_bson(Bson::Document(all_measurements))?;  
        println!("Movie loaded from collection: {:?}", loaded_all_measurements_struct);
    }

    pub fn get_last_month_measurements(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let measurements = irrigation
            .find_one(Some(doc! { 
                "sensor_name":  sensor_name,
                timestamp: {
                    $lte: new Date(),
                    $gte: new Date(new Date().setMonth(new Date().getMonth() - 1))
                }
             }), None)
            .await?
            .expect("Data not found");

        // Deserialize the document into a Movie instance
        let loaded_all_measurements_struct: Measurements = bson::from_bson(Bson::Document(all_measurements))?;  
        println!("Movie loaded from collection: {:?}", loaded_all_measurements_struct);
    }

//     pub fn get_last_week_measurements(&self, new_user: User) -> Result<InsertOneResult, Error> {
//         let new_doc = User {
//             id: None,
//             name: new_user.name,
//             location: new_user.location,
//             title: new_user.title,
//         };
//         let user = self
//             .col
//             .insert_one(new_doc, None)
//             .ok()
//             .expect("Error creating user");
//         Ok(user)
//     }

//     pub fn get_last_day_measurements(&self, new_user: User) -> Result<InsertOneResult, Error> {
//         let new_doc = User {
//             id: None,
//             name: new_user.name,
//             location: new_user.location,
//             title: new_user.title,
//         };
//         let user = self
//             .col
//             .insert_one(new_doc, None)
//             .ok()
//             .expect("Error creating user");
//         Ok(user)
//     }

//     pub fn get_last_hour_measurements(&self, new_user: User) -> Result<InsertOneResult, Error> {
//         let new_doc = User {
//             id: None,
//             name: new_user.name,
//             location: new_user.location,
//             title: new_user.title,
//         };
//         let user = self
//             .col
//             .insert_one(new_doc, None)
//             .ok()
//             .expect("Error creating user");
//         Ok(user)
//     }

//     pub fn getPreference(&self, new_user: User) -> Result<InsertOneResult, Error> {
//         let new_doc = User {
//             id: None,
//             name: new_user.name,
//             location: new_user.location,
//             title: new_user.title,
//         };
//         let user = self
//             .col
//             .insert_one(new_doc, None)
//             .ok()
//             .expect("Error creating user");
//         Ok(user)
//     }

//     pub fn updatePreferences(&self, new_user: User) -> Result<InsertOneResult, Error> {
//         let new_doc = User {
//             id: None,
//             name: new_user.name,
//             location: new_user.location,
//             title: new_user.title,
//         };
//         let user = self
//             .col
//             .insert_one(new_doc, None)
//             .ok()
//             .expect("Error creating user");
//         Ok(user)

//            // // Initialize struct to be inserted:
//    // let daily_measurements = Measurements {
//    //    id: None,
//    //    sensor_name: "test_4".to_owned(),
//    //    capacity: 50,
//    //    timestamp: chrono::offset::Utc::now()
//    // };

//    // // Convert `captain_marvel` to a Bson instance:
//    // let serialized_movie = bson::to_bson(&daily_measurements)?;
//    // let document = serialized_movie.as_document().unwrap();

//    // // Insert into the collection and extract the inserted_id value:
//    // let insert_result = irrigation.insert_one(document.to_owned(), None).await?;
//    // let inserted_id = insert_result
//    //    .inserted_id
//    //    .as_object_id()
//    //    .expect("Retrieved _id should have been of type ObjectId");
//    // println!("Captain Marvel document ID: {:?}", inserted_id);
//     }

//     pub fn getSensorNames(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
}