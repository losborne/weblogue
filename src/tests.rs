use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn test_index() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body_string().is_some());
}

#[test]
fn test_post_content() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body_string().unwrap()
                    .find("class=\"post\"")
                    .is_some());
}

