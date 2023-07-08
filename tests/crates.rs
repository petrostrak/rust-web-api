use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

fn create_test_crate(client: &Client) -> Value {
    let response = client
        .post("http://127.0.0.1:8000/crates")
        .json(&json!({
            "rustacean_id":1,
            "code":"c0d3",
            "name":"petros",
            "version": "0.1",
            "description":Some("My new crate!")
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json::<Value>().unwrap()
}

fn clean_test_crates(client: &Client, crt: Value) {
    let response = client
        .delete(format!("http://127.0.0.1:8000/crates/{}", crt["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
