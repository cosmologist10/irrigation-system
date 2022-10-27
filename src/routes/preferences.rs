
#[get("/<sensorName>")]
fn getPreference() -> &'static str {
    "Hello, world!"
}

#[put("/<sensorName>")]
fn updatePreferences() -> &'static str {
    "Hello, world!"
}
