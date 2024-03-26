#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::tokio::time::{sleep, Duration};

#[cfg(test)]
mod main_test;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/api", routes![hello_api])
        .mount("/static", FileServer::from("static"))
        .mount("/async", routes![delay])
}
