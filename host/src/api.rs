use anyhow::Result;
use contract::Meetup;
use contract::MeetupAction;
use client_sdk::rest_client::NodeApiHttpClient;
use sdk::{ContractInput, HyleContract};
use sdk::Digestable;

pub async fn register_contract(
    host: &str,
    contract_name: &str,
) -> Result<String> {
    // Initialize the client, converting host to a String.
    let client = NodeApiHttpClient::new(host.to_string())?;
    
    // Build initial state of the contract
    let initial_state = Meetup { merkle_roots: Vec::new() };

    // Register the contract (adjust fields as needed)
    let res = client.register_contract(
        &sdk::api::APIRegisterContract {
            verifier: "risc0".into(),
            program_id: sdk::ProgramId(vec![]), // Replace with proper value
            state_digest: initial_state.as_digest(),
            contract_name: contract_name.into(),
        }
    ).await?;
    
    Ok(res.to_string())
}

pub async fn post_root(
    host: &str,
    contract_name: &str,
) -> Result<String> {
    // Initialize the client
    let client = NodeApiHttpClient::new(host.to_string())?;
    let identity = format!("none.{}", contract_name);

    // Fetch initial state from the node
    let mut initial_state: Meetup = client
        .get_contract(&contract_name.into())
        .await?
        .state
        .into();

    // Build the blob transaction
    let action = MeetupAction::PostRoot {};
    let blobs = vec![action.as_blob(contract_name)];
    let blob_tx = sdk::BlobTransaction::new(identity.clone(), blobs.clone());

    // Send the blob transaction
    let blob_tx_hash = client.send_tx_blob(&blob_tx).await?;
    
    // Execute contract logic with the provided state
    let inputs = ContractInput {
        state: initial_state.as_bytes()?,
        identity: identity.clone().into(),
        tx_hash: blob_tx_hash.clone(),
        private_input: vec![],
        tx_ctx: None,
        blobs: blobs.clone(),
        index: sdk::BlobIndex(0),
    };

    // Convert the error (if any) from String to anyhow::Error.
    let (program_outputs, _, _) = initial_state
        .execute(&inputs)
        .map_err(|e| anyhow::anyhow!(e))?;
    println!("Executed: {}", program_outputs);

    Ok(blob_tx_hash.to_string())
}