use anyhow::Result;
use clap::{Parser, Subcommand};
use client_sdk::helpers::risc0::Risc0Prover;
use contract::Meetup;
use contract::MeetupAction;
use sdk::api::APIRegisterContract;
use sdk::BlobTransaction;
use sdk::ProofTransaction;
use sdk::{ContractInput, Digestable, HyleContract};

// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{GUEST_ELF, GUEST_ID};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, default_value = "http://localhost:4321")]
    pub host: String,

    #[arg(long, default_value = "counter")]
    pub contract_name: String,
}

#[derive(Subcommand)]
enum Commands {
    RegisterContract {},
    PostRoot {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Client to send requests to the node
    let client = client_sdk::rest_client::NodeApiHttpClient::new(cli.host)?;
    let contract_name = &cli.contract_name;

    // Will be used to generate zkProof of the execution.
    let prover = Risc0Prover::new(GUEST_ELF);

    // This dummy example doesn't uses identities. But there are required fields & validation.
    let identity = format!("none.{}", contract_name);

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
        Commands::PostRoot {} => {
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
                private_input: vec![],
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
    }
    Ok(())
}
