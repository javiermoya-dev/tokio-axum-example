use axum::{
    routing::{get, post},
    http::StatusCode,
    Extension,
    Json, Router, response::Response, body::Body,
};
use axum::response::IntoResponse;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use models::Item;
use r2d2::{self, PooledConnection};
use dotenv::dotenv;
use std::env;
use serde::Serialize;
use lazy_static::lazy_static;

mod db;
mod models;
mod schema;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Serialize)]
struct Data {
    message: &'static str,
}

async fn index() -> impl IntoResponse {
    axum::Json(Data {
        message: "Hello, Axum!",
    })
}

async fn get_items() -> (StatusCode, Json<Vec<Item>>) {
    let conn = &mut db::connection();
    
    (
        StatusCode::OK,
        Json(models::Item::all(conn))
    )
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Build the Axum app
    let app = Router::new()
        .route("/", get(index))
        .route("/items", get(get_items))
        .layer(Extension(db::init().clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    
    // Start the Axum server
    axum::serve(listener, app).await.unwrap();
}