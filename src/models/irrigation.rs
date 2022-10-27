use serde::Serialize;


#[derive(Serialize)]
pub struct Measurements<'a> {
    pub sensorName: String,
    pub capacity: i32,
    pub timestamp: DateTime<Utc>,
}


impl Irrigation{
    pub fn getIrrigations(&self, sensor_name: String) -> Measurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }
}
