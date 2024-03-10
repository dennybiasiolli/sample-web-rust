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
