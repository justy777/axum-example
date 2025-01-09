use axum::http::{header::CONTENT_TYPE, StatusCode};
use axum_test_helper::TestClient;
use serde_json::{json, Value};
use crate::http::api_routes;
use self::context::TestContext;

mod context;

#[tokio::test]
async fn fallback_not_found() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let response = client.get("/does-not-exist").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_tags_ok() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let response = client.get("/api/v1/tags").await;
    assert_eq!(response.status(), StatusCode::OK);
    let text = response.text().await;
    let value: Value = serde_json::from_str(&text).unwrap();

    assert_eq!(value.as_array().unwrap().len(), 2);
    assert_eq!(value[0]["label"], json!("star"));
}

#[tokio::test]
async fn get_tag_ok() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let response = client.get("/api/v1/tags/1").await;
    assert_eq!(response.status(), StatusCode::OK);
    let text = response.text().await;
    let value: Value = serde_json::from_str(&text).unwrap();

    assert_eq!(value["label"], json!("star"));
}

#[tokio::test]
async fn get_tag_not_found_id() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let response = client.get("/api/v1/tags/3").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_tag_malformed_id() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let response = client.get("/api/v1/tags/x").await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_tag_ok() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let create_tag = json!({"label": "popular"});

    let response = client.post("/api/v1/tags").json(&create_tag).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    let text = response.text().await;
    let value: Value = serde_json::from_str(&text).unwrap();

    assert_eq!(value["label"], json!("popular"));
}

#[tokio::test]
async fn create_tag_already_exists() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let create_tag = json!({"label": "star"});

    let response = client.post("/api/v1/tags").json(&create_tag).await;
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn create_tag_missing_content_type() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let response = client.post("/api/v1/tags").body("").await;
    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

#[tokio::test]
async fn create_tag_empty_body() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let create_movie = json!({});

    let response = client.post("/api/v1/tags").json(&create_movie).await;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn create_tag_invalid_syntax() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let response = client
        .post("/api/v1/tags")
        .header(CONTENT_TYPE, "application/json")
        .body("{")
        .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_tag_invalid_field_type() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let create_tag = json!({"label": 5});

    let response = client.post("/api/v1/tags").json(&create_tag).await;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn create_tag_missing_required_field() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let create_movie = json!({
        "originally_available_at": "2022-03-18",
    });

    let response = client.post("/api/v1/tags").json(&create_movie).await;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn delete_tag_ok() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let response = client.get("/api/v1/tags/1").await;
    assert_eq!(response.status(), StatusCode::OK);

    let response = client.delete("/api/v1/tags/1").await;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = client.get("/api/v1/tags/1").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_tag_not_found_id() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let response = client.delete("/api/v1/tags/3").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_movie_malformed_id() {
    let ctx = TestContext::new();
    let router = api_routes().with_state(ctx.pool().await);
    let client = TestClient::new(router);

    let response = client.delete("/api/v1/tags/x").await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
