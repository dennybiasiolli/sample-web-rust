#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: String,
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/hello")]
fn hello_api() -> Json<Message> {
    Json(Message {
        message: "Hello, world!".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/api", routes![hello_api])
}
