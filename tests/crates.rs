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

#[test]
fn test_get_crates() {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:8000/crates").send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let crate_1 = create_test_crate(&client);
    let crate_2 = create_test_crate(&client);

    let response = client.get("http://127.0.0.1:8000/crates").send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response_json = response.json::<Value>().unwrap();
    assert!(response_json.as_array().unwrap().contains(&crate_1));
    assert!(response_json.as_array().unwrap().contains(&crate_2));

    clean_test_crates(&client, crate_1);
    clean_test_crates(&client, crate_2);
}
