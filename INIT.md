## Project initialization

### 1. Create a new project
```bash
GIT_REPO=sample-web-rust
mkdir $GIT_REPO
cd $GIT_REPO
cargo init .
```

## Rocket Web Framework

```
cargo add rocket
```

```rust
// file: src/main.rs
#[macro_use]
extern crate rocket;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
```

```bash
cargo run
```


## Add an API endpoint

```toml
[dependencies]
rocket = { version = "0.5.0", features = ["json"] }
```

```rust
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: String,
}

#[get("/hello")]
fn hello_api() -> Json<Message> {
    Json(Message {
        message: "Hello, world!".to_string(),
    })
}

// ...
    .mount("/api", routes![hello_api])
```


## Dynamic Paths

```rust
#[get("/hello_advanced/<name>/<age>/<cool>")]
fn hello_advanced(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

// ...
    .mount("/", routes![hello, hello_advanced])
```


## Serve static files

```rust
use rocket::fs::FileServer;

// ...
    .mount("/static", FileServer::from("static"))
```


## Async requests

```rust
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

// ...
    .mount("/async", routes![delay])
```
