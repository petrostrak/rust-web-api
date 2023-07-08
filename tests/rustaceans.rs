use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK)
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!(
            {"email":"pit.trak@gmail.com",
            "name": "petros trak"}
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let json = response.json::<Value>().unwrap();
    assert_eq!(
        json,
        json!({
            "id": json["id"],
            "email":"pit.trak@gmail.com",
            "name": "petros trak",
            "created_at": json["created_at"]
        })
    );
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!(
            {"email":"pit.trak@gmail.com",
            "name": "petros trak"}
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let json = response.json::<Value>().unwrap();

    let response = client
        .put(format!("http://127.0.0.1:8000/rustaceans/{}", json["id"]))
        .json(&json!({"email":"trak.pit@gmail.com",
            "name": "trak petros"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<Value>().unwrap();
    assert_eq!(
        json,
        json!({
            "id": json["id"],
            "email":"trak.pit@gmail.com",
            "name": "trak petros",
            "created_at": json["created_at"]
        })
    );
}
