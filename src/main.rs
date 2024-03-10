#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn index_api() -> Json<Message> {
    Json(Message {
        message: "Hello, world!".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![index_api])
}
