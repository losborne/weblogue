use super::rocket;
use rocket::local::{Client, LocalResponse};
use rocket::http::Method::*;
use rocket::http::{Status};
use rocket_contrib::templates::Template;

macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => ({
        let client = Client::new(rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    })
}

macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => ({
        let client = Client::new(rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    })
}
#[test]
fn test_index() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Status::Ok);
}

