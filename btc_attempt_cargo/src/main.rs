use bitcoin::{Address, PublicKey, Network};
use bitcoin::secp256k1::{rand, Secp256k1};

use rand::rngs::ThreadRng;
use schnorr_fun::{
adaptor::{Adaptor, EncryptedSign},
fun::{marker::*, nonce, Scalar},
Message, Schnorr,
};
use sha2::Sha256;

fn main() {
let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);
let signing_keypair = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));
let verification_key = signing_keypair.public_key();
// Oracle Y_i, and y_i
let decryption_key = Scalar::random(&mut rand::thread_rng());
let encryption_key = schnorr.encryption_key_for(&decryption_key);
let message = Message::<Public>::plain("text-bitcoin", b"send 1 BTC to Bob");

// Alice knows: signing_keypair, encryption_key
// Bob knows: decryption_key, verification_key

// Alice creates an encrypted signature and sends it to Bob
let encrypted_signature = schnorr.encrypted_sign(&signing_keypair, &encryption_key, message);

// Bob verifies it and decrypts it
assert!(schnorr.verify_encrypted_signature(
    &verification_key,
    &encryption_key,
    message,
    &encrypted_signature
));
let signature = schnorr.decrypt_signature(decryption_key, encrypted_signature.clone());

// Bob then broadcasts the signature to the public.
// Once Alice sees it she can recover Bob's secret decryption key
match schnorr.recover_decryption_key(&encryption_key, &encrypted_signature, &signature) {
    Some(decryption_key) => println!("Alice got the decryption key {}", decryption_key),
    None => eprintln!("signature is not the decryption of our original encrypted signature"),
}
}

// Function to generate a random key pair, compressed public key, and Bitcoin address
fn rand_pair_gen() {
    // Generate random key pair.
    let s = Secp256k1::new();
    let public_key = s.generate_keypair(&mut rand::thread_rng()).1;
    let compressed_public_key = PublicKey::new(public_key);

    // Generate pay-to-pubkey-hash address.
    let address = Address::p2pkh(&compressed_public_key, Network::Bitcoin);

    // Print the uncompressed public key, compressed public key, and Bitcoin address.
    println!("This is (uncompressed) public key: {:?}\n", public_key.serialize_uncompressed());
    println!("This is compressed public key: {}", compressed_public_key);
    println!("This is bitcoin address: {}", address);
}