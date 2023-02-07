#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = libadmin::rocket().launch().await?;
    Ok(())
}
