use std::net::SocketAddr;
extern crate rsa;

// our router
use axum::{
    routing::{get, post},
    Json, Router,
};

use serde::Deserialize;
mod digital_signature;

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
        .route("/", get(root))
        .route("/getusers", get(get_voters).post(create_voters))
        .route("/createusers", post(create_voters));

    // which calls one of these handlers
    async fn root() {}
    async fn get_voters() {
        // get ussers
    }
    async fn create_voters(Json(payload): Json<CreateUser>) {
        println!("Received payload {:#?}", payload);
        //hash user then store them in the database

        digital_signature::digital_signature(payload)
    }

    // which calls one of these handlers
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("--> listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
