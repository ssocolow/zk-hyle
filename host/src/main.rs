use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::Deserialize;

mod api; // This module contains your core functions
mod http_server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  /// Run in CLI mode instead of HTTP mode.
  #[arg(long)]
  cli: bool,

  #[command(subcommand)]
  command: Option<Commands>,

  /// Host URL for the node (default: http://localhost:4321)
  #[arg(long, default_value = "http://localhost:4321")]
  host: String,

  /// Contract name (default: counter)
  #[arg(long, default_value = "counter")]
  contract_name: String,
}

#[derive(Subcommand)]
enum Commands {
  RegisterContract {},
  PostRoot {},
}

#[derive(Deserialize)]
struct RegisterContractRequest {
  host: String,
  contract_name: String,
}

// #[derive(Deserialize)]
// struct PostRootRequest {
//   host: String,
//   contract_name: String,
// }

// async fn register_contract_endpoint(
//   req: web::Json<RegisterContractRequest>,
// ) -> impl Responder {
//   match api::register_contract(&req.host, &req.contract_name).await {
//     Ok(tx_hash) => HttpResponse::Ok().json(serde_json::json!({ "tx_hash": tx_hash })),
//     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//   }
// }

// async fn post_root_endpoint(
//   req: web::Json<PostRootRequest>,
// ) -> impl Responder {
//   match api::post_root(&req.host, &req.contract_name).await {
//     Ok(tx_hash) => HttpResponse::Ok().json(serde_json::json!({ "tx_hash": tx_hash })),
//     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//   }
// }


#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();

  if cli.cli {
    // Run CLI mode.
    match cli.command {
      Some(Commands::RegisterContract {}) => {
        let tx_hash = api::register_contract(&cli.host, &cli.contract_name).await?;
        println!("✅ Register contract tx sent. Tx hash: {}", tx_hash);
      }
      Some(Commands::PostRoot {}) => {
        let tx_hash = api::post_root(&cli.host, &cli.contract_name).await?;
        println!("✅ Blob tx sent. Tx hash: {}", tx_hash);
      }
      None => {
        println!("No CLI command provided.");
      }
    }
  } else {
    // Default: Run HTTP server mode.
    http_server::run_server().await?;
  }

  Ok(())
}