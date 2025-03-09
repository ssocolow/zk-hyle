use borsh::{io::Error, BorshDeserialize, BorshSerialize};
use serde::{forward_to_deserialize_any, Deserialize, Serialize};

use sdk::{Digestable, HyleContract, RunResult};
use sha2::{Digest, Sha256};


impl HyleContract for Meetup {
    /// Entry point of the contract's logic
    fn execute(&mut self, contract_input: &sdk::ContractInput) -> RunResult {
        // Parse contract inputs
        let (action, ctx) = sdk::utils::parse_raw_contract_input::<MeetupAction>(contract_input)?;

        // Execute the contract logic
        match action {
            MeetupAction::PostRoot => {
                // Hash the private input
                let mut hasher = Sha256::new();
                hasher.update(&contract_input.private_input);
                let result = hasher.finalize();

                // convert to string
                let hash = format!("{:x}", result);
                self.merkle_roots.push(hash);
            }
            MeetupAction::AddEncryption => {
                let mut hasher = Sha256::new();
                hasher.update(&contract_input.private_input);

                // p, q, interest1, interest2, interest3, ...
                let data = core::str::from_utf8(&contract_input.private_input).unwrap();
                let numbers: Vec<u128> = data.split(" ").map(|x| x.parse().unwrap()).collect();

                let p: u128 = numbers[0];
                let q: u128 = numbers[1];
                let encrypted_messages: Vec<u128> = numbers.iter().map(|msg| Meetup::encrypt(p, q, *msg)).collect();
                let encrypted_messages_str = encrypted_messages.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
                hasher.update(encrypted_messages_str.as_bytes());
                let result = hasher.finalize();
                let hash = format!("{:x}", result);
                self.encrypted_message_hash = hash;
            }
        }

        // program_output might be used to give feedback to the user
        let program_output = format!("new value: {}", self.merkle_roots.join(" "));
        Ok((program_output, ctx, vec![]))
    }
}

impl Meetup {
    fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
        if modulus == 1 { return 0 }
        let mut result = 1;
        base = base % modulus;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % modulus;
            }
            exp = exp >> 1;
            base = base * base % modulus
        }
        result
    }
    
    fn encrypt(p: u128, q: u128, msg: u128) -> u128 {
        // paillier encryption
        // let p: u128 = 13;
        // let q: u128 = 23;

        let n: u128 = p * q;
        // let lambda: u128 = (p - 1) * (q - 1);
        let g: u128 = n + 1;
        // let mu = Meetup::mod_inverse(n, lambda);

        // get message
        let r: u128 = 5;
        let mut c = Self::mod_pow(g, msg, n.pow(2));
        c *= Self::mod_pow(r, n, n.pow(2));
        c % n.pow(2)
    }

    /*
    fn mod_inverse (n: u128, p: u128) -> u128{
        // Checks numbers from 1 to p-1
        for x in 1..p {
            if (n * x) % p == 1 {
                return x;
            }
        }

        // Returns 0 if no Modular Multiplicative Inverse exist
        return 0;
    }
    */
}

/// The action represents the different operations that can be done on the contract
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum MeetupAction {
    PostRoot,
    AddEncryption,
}

/// The state of the contract, in this example it is fully serialized on-chain
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct Meetup {
    pub merkle_roots: Vec<String>,
    pub encrypted_message_hash: String,
}

/// Utils function for the host
impl Meetup {
    pub fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        borsh::to_vec(self)
    }
}

/// Utils function for the host
impl MeetupAction {
    pub fn as_blob(&self, contract_name: &str) -> sdk::Blob {
        sdk::Blob {
            contract_name: contract_name.into(),
            data: sdk::BlobData(borsh::to_vec(self).expect("failed to encode BlobData")),
        }
    }
}

/// Helpers to transform the contrat's state in its on-chain state digest version.
/// In an optimal version, you would here only returns a hash of the state,
/// while storing the full-state off-chain
impl Digestable for Meetup {
    fn as_digest(&self) -> sdk::StateDigest {
        sdk::StateDigest(borsh::to_vec(self).expect("Failed to encode Balances"))
    }
}
impl From<sdk::StateDigest> for Meetup {
    fn from(state: sdk::StateDigest) -> Self {
        borsh::from_slice(&state.0)
            .map_err(|_| "Could not decode hyllar state".to_string())
            .unwrap()
    }
}
