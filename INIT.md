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

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
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

#[get("/")]
fn index_api() -> Json<Message> {
    Json(Message {
        message: "Hello, world!".to_string(),
    })
}

// mount the API endpoint
rocket::build()
    .mount("/", routes![index])
    .mount("/api", routes![index_api]) // <- add this line
```
