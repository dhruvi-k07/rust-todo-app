// tests/integration_tests.rs

use rocket::local::asynchronous::Client;
use rocket::http::Status;
use serde_json::json;
use reqwest::Client as ReqwestClient;

#[rocket::async_test]
async fn test_create_todo() {
    let client = Client::tracked(rocket()).await.expect("valid rocket instance");
    let reqwest_client = ReqwestClient::new();

    let new_todo = json!({
        "title": "Test Todo",
        "description": "This is a test todo",
        "done": false
    });

    let response = reqwest_client
        .post("http://localhost:8000/todo")
        .json(&new_todo)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), Status::Ok);

    let created_todo: serde_json::Value = response.json().await.expect("Failed to parse response");

    assert_eq!(created_todo["title"], "Test Todo");
    assert_eq!(created_todo["description"], "This is a test todo");
    assert_eq!(created_todo["done"], false);
}

#[rocket::async_test]
async fn test_get_todos() {
    let client = Client::tracked(rocket()).await.expect("valid rocket instance");
    let reqwest_client = ReqwestClient::new();

    let response = reqwest_client
        .get("http://localhost:8000/todos")
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), Status::Ok);

    let todos: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert!(todos.as_array().expect("Response is not an array").len() > 0);
}

#[rocket::async_test]
async fn test_update_todo() {
    let client = Client::tracked(rocket()).await.expect("valid rocket instance");
    let reqwest_client = ReqwestClient::new();

    let updated_todo = json!({
        "title": "Updated Test Todo",
        "description": "This is an updated test todo",
        "done": true
    });

    let response = reqwest_client
        .put("http://localhost:8000/todo/1")
        .json(&updated_todo)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), Status::Ok);

    let updated_todo_response: serde_json::Value = response.json().await.expect("Failed to parse response");

    assert_eq!(updated_todo_response["title"], "Updated Test Todo");
    assert_eq!(updated_todo_response["description"], "This is an updated test todo");
    assert_eq!(updated_todo_response["done"], true);
}

#[rocket::async_test]
async fn test_delete_todo() {
    let client = Client::tracked(rocket()).await.expect("valid rocket instance");
    let reqwest_client = ReqwestClient::new();

    let response = reqwest_client
        .delete("http://localhost:8000/todo/1")
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), Status::Ok);

    let delete_response: String = response.text().await.expect("Failed to parse response");
    assert_eq!(delete_response, "Todo deleted");
}
