use serde::Serialize;


#[derive(Serialize)]
pub struct Measurements<'a> {
    pub sensorName: String,
    pub capacity: i32,
    pub timestamp: DateTime<Utc>,
}

pub struct AllMeasurements<'a>{
    pub sensorName: String,
}

pub struct QueryMeasurement<'a> {
    pub sensorName: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}


impl Measurement {

    pub fn getAllMeasurements(&self, queryFilter: AllMeasurements) -> Measurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }

    pub fn getDailyMeasurements(&self, queryFilter: QueryMeasurement) -> Measurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }

    pub fn getHourlyMeasurements(&self, queryFilter: QueryMeasurement) -> Measurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }

    pub fn getMinutelyMeasurements(&self, queryFilter: QueryMeasurement) -> Measurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }

    pub fn getSecondlyMeasurements(&self, queryFilter: QueryMeasurement) -> Measurements {
        DailyMeasurements {
            sensorName: '',
            capacity: 10,
            timestamp: 2022-10-27T10:48:57.578Z,
        }
    }
}
