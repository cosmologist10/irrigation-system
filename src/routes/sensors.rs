
#[get("/")]
fn getSensorNames() -> &'static str {
    "Hello, world!"
}

#[get("/irrigation/<sensorName>")]
fn irrigation() -> &'static str {
    "Hello, world!"
}