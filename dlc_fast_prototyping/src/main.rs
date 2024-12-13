// main.rs

use oracle::Oracle;

mod oracle;
mod secp_utils;

fn main() {
    let oracle = oracle::LocalOracle::new();
    println!("Public key: {:?}", oracle.get_public_key());
    println!("Public nonce: {:?}", oracle.get_announcement(0).1);
    println!("Outcome: {:?}", oracle.get_outcome());
    println!("Attestation: {:?}", oracle.get_attestation(0).1);
}
