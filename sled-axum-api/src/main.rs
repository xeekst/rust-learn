use anyhow::{anyhow, Result};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {
    pub state: String,
}

#[tokio::main]
async fn main() {
    let db = init_sled().unwrap();
    let state = AppState {
        state: "".to_string(),
    };

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        // .route("/users", post(create_user))
        // .route("/test_state", get(test_state))
        .route("/get_key", get(get_key))
        .route("/put_key", get(put_key))
        .with_state(db);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn init_sled() -> Result<Db> {
    let tree = sled::open("shard-database.sled")?;

    Ok(tree)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn test_state(State(s):  State<AppState>) -> (StatusCode, String) {
    //s.state = "2".to_string();

    (StatusCode::OK, "".to_string())
}

async fn get_key(State(db): State<Db>) -> (StatusCode, String) {
    let q_r = db.get("KEY1").unwrap();

    match q_r {
        Some(iv) => (
            StatusCode::OK,
            std::str::from_utf8(&iv).unwrap().to_string(),
        ),
        None => (StatusCode::OK, "".to_string()),
    }
}

async fn put_key(Extension(db): Extension<Db>) -> (StatusCode, Json<bool>) {
    let q_r = db.insert("KEY1", "v1").unwrap();

    (StatusCode::OK, Json(true))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
// use axum::extract::{Path, Query, Json};
// use std::collections::HashMap;

// // `Path` gives you the path parameters and deserializes them.
// async fn path(Path(user_id): Path<u32>) {}

// // `Query` gives you the query parameters and deserializes them.
// async fn query(Query(params): Query<HashMap<String, String>>) {}

// // Buffer the request body and deserialize it as JSON into a
// // `serde_json::Value`. `Json` supports any type that implements
// // `serde::Deserialize`.
// async fn json(Json(payload): Json<serde_json::Value>) {}