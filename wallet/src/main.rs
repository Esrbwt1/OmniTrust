use ed25519_dalek::{Keypair, Signer, Verifier, PublicKey};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use serde_json;
use hex;

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
    signature: Vec<u8>,
}

fn main() {
    // Generate keypair
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public;

    // Create transaction
    let tx = Transaction {
        sender: hex::encode(public_key.to_bytes()),
        receiver: String::from("Bob"),
        amount: 0.01,
        signature: vec![],
    };

    // Sign transaction
    let tx_string = serde_json::to_string(&tx).unwrap();
    let tx_bytes = tx_string.as_bytes();
    let signature = keypair.sign(tx_bytes).to_bytes().to_vec();
    let tx_signed = Transaction {
        signature,
        ..tx
    };

    // Verify signature
    let public_key_bytes = hex::decode(&tx_signed.sender).unwrap();
    let public_key = PublicKey::from_bytes(&public_key_bytes).unwrap();
    let signature_bytes = tx_signed.signature.clone();
    let verified = public_key.verify(tx_bytes, &ed25519_dalek::Signature::from_bytes(&signature_bytes).unwrap()).is_ok();

    println!("Transaction: {:?}", tx_signed);
    println!("Signature verified: {}", verified);
}