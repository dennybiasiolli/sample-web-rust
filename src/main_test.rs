use super::{rocket, Message};
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_hello_route() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let req = client.get("/hello"); // or uri!(super::hello);
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}

#[test]
fn test_api_hello_route() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let req = client.get("/api/hello"); // or uri!(super::hello_api);
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_json::<Message>().unwrap(),
        Message {
            message: "Hello, world!".to_string()
        }
    );
}

#[test]
fn test_static_route() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let req = client.get("/static/");
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    assert!(body.contains("<title>Static HTML page</title>"));
    assert!(body.contains("<h1>Static HTML page</h1>"));
}

#[test]
fn test_delay_route() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let req = client.get("/async/delay/1");
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Waited for 1 seconds");
}
