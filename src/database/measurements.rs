use super::schema::students;

#[derive(Queryable)]
pub struct daily_measurement {
    pub sensor_name: String,
    pub capacity: i32,
    pub timestamp: DateTime,
}

#[derive(Queryable)]
pub struct hourly_measurement {
    pub sensor_name: String,
    pub capacity: i32,
    pub timestamp: DateTime,
}

#[derive(Queryable)]
pub struct minutely_measurement {
    pub sensor_name: String,
    pub capacity: i32,
    pub timestamp: DateTime,
}

#[derive(Queryable)]
pub struct secondly_measurement {
    pub sensor_name: String,
    pub capacity: i32,
    pub timestamp: DateTime,
}

use sqlx::mysql::MySqlPoolOptions;

#[async_std::main]
async fn dailyMeasurement(sensor_name: &str) -> Result<daily_measurement, sqlx::Error>  {
    sqlx::query("select * from daily_measurement where sensor_name = ? and deleted = 0")
        .bind(sensorName)
        .bind(1)
        .execute(&pool)
        .await?;
    Ok(())
}

#[async_std::main]
async fn hourlyMeasurement(sensor_name: &str, capacity: i32, timestamp: DateTime) -> Result<hourly_measurement, sqlx::Error> {
    sqlx::query("select * from hourly_measurement where sensor_name = ? and deleted = 0 and capacity = ? and timestamp = ?")
        .bind(sensorName)
        .bind(capacity)
        .bind(timestamp)
        .bind(1)
        .execute(&pool)
        .await?;
    Ok(())
}

#[async_std::main]
async fn minutelyMeasurement(sensor_name: &str, capacity: i32, timestamp: DateTime) -> Result<minutely_measurement, sqlx::Error> {
    sqlx::query("select * from minutely_measurement where sensor_name = ? and deleted = 0 and capacity = ? and timestamp = ?")
        .bind(sensorName)
        .bind(capacity)
        .bind(timestamp)
        .bind(1)
        .execute(&pool)
        .await?;
    Ok(())
}

#[async_std::main]
async fn secondlyMeasurement(sensor_name: &str, capacity: i32, timestamp: DateTime) -> Result<secondly_measurement, sqlx::Error> {
    sqlx::query("select * from secondly_measurement where sensor_name = ? and deleted = 0 and capacity = ? and timestamp = ?")
        .bind(sensorName)
        .bind(capacity)
        .bind(timestamp)    
        .bind(1)
        .execute(&pool)
        .await?;
    Ok(())
}
