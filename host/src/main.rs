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
    PostRoot {
        interests: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();

  if cli.cli {
    // Run CLI mode.
    match cli.command {
        Commands::RegisterContract {} => {
            // Build initial state of contract
            let initial_state = Meetup { merkle_roots: Vec::new() };

            // Send the transaction to register the contract
            let res = client
                .register_contract(&APIRegisterContract {
                    verifier: "risc0".into(),
                    program_id: sdk::ProgramId(sdk::to_u8_array(&GUEST_ID).to_vec()),
                    state_digest: initial_state.as_digest(),
                    contract_name: contract_name.clone().into(),
                })
                .await?;
            println!("✅ Register contract tx sent. Tx hash: {}", res);
        }
        Commands::PostRoot { interests } => {
            // Fetch the initial state from the node
            let mut initial_state: Meetup = client
                .get_contract(&contract_name.clone().into())
                .await
                .unwrap()
                .state
                .into();

            // ----
            // Build the blob transaction
            // ----
            let action = MeetupAction::PostRoot {};
            let blobs = vec![action.as_blob(contract_name)];
            let blob_tx = BlobTransaction::new(identity.clone(), blobs.clone());

            // Send the blob transaction
            let blob_tx_hash = client.send_tx_blob(&blob_tx).await.unwrap();
            println!("✅ Blob tx sent. Tx hash: {}", blob_tx_hash);

            // ----
            // Prove the state transition
            // ----

            // Build the contract input
            let inputs = ContractInput {
                state: initial_state.as_bytes().unwrap(),
                identity: identity.clone().into(),
                tx_hash: blob_tx_hash,
                private_input: interests.as_bytes().to_vec(),
                tx_ctx: None,
                blobs: blobs.clone(),
                index: sdk::BlobIndex(0),
            };

            let (program_outputs, _, _) = initial_state.execute(&inputs).unwrap();
            println!("🚀 Executed: {}", program_outputs);

            // Generate the zk proof
            let proof = prover.prove(inputs).await.unwrap();

            // Build the Proof transaction
            let proof_tx = ProofTransaction {
                proof,
                contract_name: contract_name.clone().into(),
            };

            // Send the proof transaction
            let proof_tx_hash = client.send_tx_proof(&proof_tx).await.unwrap();
            println!("✅ Proof tx sent. Tx hash: {}", proof_tx_hash);
        }
>>>>>>> 06831a8fea2fe146b3d7f53c28e26b85bacb8faf
    }
  } else {
    // Default: Run HTTP server mode.
    http_server::run_server().await?;
  }

  Ok(())
}