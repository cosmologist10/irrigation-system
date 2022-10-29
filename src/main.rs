use irrigation;
use rocket;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    irrigation::rocket().launch().await?;
    Ok(())
}
