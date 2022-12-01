#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "{\"msg\":\"Hello World\"}"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

