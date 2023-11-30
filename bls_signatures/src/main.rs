use blsful::{ SecretKey, PublicKey };
use blsful::Bls12381G2Impl;
use blsful::SignatureSchemes;

fn main() {
    let sk = SecretKey::<Bls12381G2Impl>::new();
    let pk = PublicKey::from(&sk);
    let msg = b"00000000-0000-0000-0000-000000000000";
    let msg2 = b"00000000-0000-0000-0000-000000000001";

    let sig = SecretKey::sign(&sk, SignatureSchemes::Basic, msg).expect("a valid signature");

    match sig.verify(&pk, msg2) {
        Ok(()) => println!("Correct - Signature is valid"),
        Err(err) => println!("Error - Invalid signature: {:?}", err),
    }
}