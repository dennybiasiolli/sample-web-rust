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

        .mount("/api", routes![hello_api])
```


## Serve static files

```rust
use rocket::fs::FileServer;

        .mount("/static", FileServer::from("static"))
```
