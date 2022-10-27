use serde::Serialize;


#[derive(Serialize)]
pub struct Measurements<'a> {
    pub sensorName: String,
    pub capacity: i32,
    pub timestamp: DateTime<Utc>,
}


impl Measurement {
    pub fn getDailyMeasurements(&self, sensor_name: String) -> Measurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }

    pub fn getLastMonthMeasurements(&self, sensor_name: String) -> Measurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }

    pub fn getLastWeekMeasurements(&self, sensor_name: String) -> Measurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }

    pub fn getLastHourMeasurements(&self, sensor_name: String) -> Measurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }

    pub fn getLastMinuteMeasurements(&self, sensor_name: String) -> DailyMeasurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }
}
