// host/src/http_server.rs

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use serde::Deserialize;
use crate::api;


#[derive(Deserialize)]
struct RegisterContractRequest {
    host: String,
    contract_name: String,
}

#[derive(Deserialize)]
struct PostRootRequest {
    host: String,
    contract_name: String,
}

#[post("/register-contract")]
async fn register_contract(req: web::Json<RegisterContractRequest>) -> impl Responder {
    match api::register_contract(&req.host, &req.contract_name).await {
        Ok(tx_hash) => HttpResponse::Ok().json(serde_json::json!({ "tx_hash": tx_hash })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/post-root")]
async fn post_root(req: web::Json<PostRootRequest>) -> impl Responder {
    match api::post_root(&req.host, &req.contract_name).await {
        Ok(tx_hash) => HttpResponse::Ok().json(serde_json::json!({ "tx_hash": tx_hash })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register_contract)
            .service(post_root)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}