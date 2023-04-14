#![allow(non_snake_case)]
mod extract;
use extract::Data;

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> String {
    let res = Data::netcdf("data.nc", "pr", 9_i32, 3_i32).unwrap();
    res.to_string()
}
