use serde::{Serialize, Deserialize};
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

fn main() {
    let tx = Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 0.01,
    };
    println!("Transaction: {:?}", tx);
}