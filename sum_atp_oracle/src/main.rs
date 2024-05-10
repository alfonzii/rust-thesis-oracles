extern crate secp256k1_zkp;

use secp256k1_zkp::{rand, KeyPair, Secp256k1};

// Demonstrate the use of the adaptor signature scheme
// Alice creates pre-signature based on Oracle and sends it to Bob
// Bob verifies the pre-signature based on Oracle and decrypts it using Oracle attestation
fn main() {
    let secp = Secp256k1::new();
    // Alice generates a key pair
    let (secret_key, verification_key) = secp.generate_keypair(&mut rand::thread_rng());

    // TODO: NEEDS REWORK FOR SUM METHOD - Oracle generates attestation and anticipation point
    let oracle_attestation = KeyPair::new(&secp, &mut rand::thread_rng()).secret_key();
    let anticipation_point = oracle_attestation.public_key(&secp);
    println!("Oracle attestation: {:?}", oracle_attestation);

    // Message to be pre-signed
    let message = secp256k1_zkp::Message::from_slice(&[0xab; 32]).unwrap();

    // Alice knows: secret_key, verification_key, anticipation_point
    // Bob knows: verification_key, anticipation_point
    // Oracle knows: oracle_attestation, anticipation_point

    // Alice creates an adaptor signature and sends it to Bob
    let adaptor_signature = secp256k1_zkp::EcdsaAdaptorSignature::encrypt(&secp, &message, &secret_key, &anticipation_point);

    // Bob verifies the adaptor signature
    if let Err(err) = adaptor_signature.verify(&secp, &message, &verification_key, &anticipation_point) {
        panic!("Adaptor signature verification failed: {:?}", err);
    }

    // Oracle attests the outcome and Bob decrypts the adaptor signature
    let signature = match adaptor_signature.decrypt(&oracle_attestation) {
        Ok(signature) => signature,
        Err(err) => {
            eprintln!("Error decrypting signature: {:?}", err);
            return;
        }
    };

    // Bob then broadcasts the signature to the public.
    // Once Alice sees it she can recover Bob's secret decryption key
    match adaptor_signature.recover(&secp, &signature, &anticipation_point) {
        Ok(decryption_key) => {
            println!("Alice got the decryption key {:?}", decryption_key);
            assert_eq!(decryption_key, oracle_attestation, "Decryption key does not match oracle attestation");
        }
        Err(err) => eprintln!("signature is not the decryption of our original encrypted signature: {:?}", err),
    }
}

// signing key - client    ---    secp256k1::key::SecretKey
// verification key - client    ---    secp256k1::key::PublicKey

// attestation - oracle    ---    sum_compute_sig_point->prerobene na secretkey
// secret key - oracle    ---    KeyPair::new(SECP256K1, &mut thread_rng())
// anticipation point - oracle    ---    sum_compute_sig_point()
// public key - oracle    ---    KeyPair::new(SECP256K1, &mut thread_rng()).x_only_public_key()

// message    ---    secp256k1::Message

// pre-sign(signing key, anticipation point, message) - client    ---    EcdsaAdaptorSignature::encrypt()
// verify(verification key, anticipation point, message, pre-signature) - client    ---    funkcia verify() na premennej(adapt. sig) typu EcdsaAdaptorSignature
// adapt(pre-signature, attestation) - client    ---    funkcia decrypt() na premennej(adapt. sig) typu EcdsaAdaptorSignature
