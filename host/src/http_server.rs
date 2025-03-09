// host/src/http_server.rs

use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;
use crate::api;
use awc::Client;

const HYLE_BLOCKCHAIN_SERVER: &str = "http://localhost:4321";
const HYLE_BLOCKCHAIN_URL: &str = "http://localhost:4321/v1";

#[derive(Debug, Deserialize)]
struct RegisterContractRequest {
    contract_name: String,
}

#[derive(Debug, Deserialize)]
struct PostRootRequest {
    host: String,
    contract_name: String,
    interests: String,
}

#[derive(Debug, Deserialize)]
struct HashedInterestsRequest {
    address: String,
    m1: String,
    m2: String,
    m3: String,
    m4: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HashedInterestsData {
    m1: String,
    m2: String,
    m3: String,
    m4: String,
}

#[post("/register-contract")]
async fn register_contract(req: web::Json<RegisterContractRequest>) -> impl Responder {
    println!("Received data: {:?}", req);
    
    match api::register_contract(HYLE_BLOCKCHAIN_SERVER, &req.contract_name).await {
        Ok(tx_hash) => {
            // Return original tx_hash regardless of POST result
            HttpResponse::Ok().json(serde_json::json!({ "tx_hash": tx_hash }))
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/post-root")]
async fn post_root(req: web::Json<PostRootRequest>) -> impl Responder {
    println!("Received root data: {:?}", req);
    
    match api::post_root(&req.host, &req.contract_name, req.interests.clone()).await {
        Ok(tx_hash) => HttpResponse::Ok().json(serde_json::json!({ "tx_hash": tx_hash })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/receive-hashed-interests")]
async fn receive_hashed_interests(req: web::Json<HashedInterestsRequest>) -> impl Responder {
    println!("Received hashed interests: {:?}", req);

    // Read existing data or create new HashMap if file doesn't exist
    let mut hashed_interests: HashMap<String, HashedInterestsData> = match fs::read_to_string("hashed-interests.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => HashMap::new(),
    };

    // Update the hashed interests for this address
    hashed_interests.insert(req.address.clone(), HashedInterestsData {
        m1: req.m1.clone(),
        m2: req.m2.clone(),
        m3: req.m3.clone(),
        m4: req.m4.clone(),
    });

    // Save back to file
    match serde_json::to_string_pretty(&hashed_interests) {
        Ok(json_str) => {
            if let Err(e) = fs::write("hashed-interests.json", json_str) {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Failed to write to file: {}", e)
                }));
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to serialize data: {}", e)
            }));
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hashed interests saved successfully"
    }))
}

pub async fn run_server() -> std::io::Result<()> {
    println!("Starting HTTP server on 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(register_contract)
            .service(post_root)
            .service(receive_hashed_interests)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}