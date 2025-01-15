use criterion::{black_box, criterion_group, criterion_main, Criterion};

use secp256k1_zkp::{
    global::SECP256K1, rand::thread_rng, EcdsaAdaptorSignature, Keypair, SecretKey,
};

use k256::schnorr::{
    signature::{Signer, Verifier},
    SigningKey, VerifyingKey,
};
use schnorr_fun::{
    adaptor::{Adaptor, EncryptedSign},
    fun::{marker::*, nonce, Scalar},
    Schnorr,
};
use sha2::Sha256;

use rand::rngs::ThreadRng;
use rand_core::{OsRng, RngCore};

// -------------------------------------------  Signing and verification -------------------------------------------

fn bench_secp256k1_zkp_sign(c: &mut Criterion) {
    let mut rng = thread_rng();
    let keypair = Keypair::new(SECP256K1, &mut rng);

    let mut buf = [0u8; 32];
    thread_rng().fill_bytes(&mut buf);
    let msg = secp256k1_zkp::Message::from_digest_slice(&buf).unwrap();

    c.bench_function("secp256k1_zkp_sign", |b| {
        b.iter(|| {
            black_box(keypair.sign_schnorr(msg));
        })
    });
}

fn bench_secp256k1_zkp_verify(c: &mut Criterion) {
    let mut rng = thread_rng();
    let keypair = Keypair::new(SECP256K1, &mut rng);
    let mut buf = [0u8; 32];
    thread_rng().fill_bytes(&mut buf);
    let msg = secp256k1_zkp::Message::from_digest_slice(&buf).unwrap();
    let sig = keypair.sign_schnorr(msg);
    let xpubkey = keypair.x_only_public_key().0;

    c.bench_function("secp256k1_zkp_verify", |b| {
        b.iter(|| {
            let _ = black_box(SECP256K1.verify_schnorr(&sig, &msg, &xpubkey));
        })
    });
}

fn bench_k256_sign(c: &mut Criterion) {
    let signing_key = SigningKey::random(&mut OsRng);
    let mut msg = [0u8; 32];
    thread_rng().fill_bytes(&mut msg);

    // Measure signing time
    c.bench_function("k256_sign", |b| {
        b.iter(|| {
            black_box(signing_key.sign(&msg)); // returns `k256::schnorr::Signature`
        })
    });
}

fn bench_k256_verify(c: &mut Criterion) {
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key_bytes = signing_key.verifying_key().to_bytes(); // 32-bytes

    let message = b"this is message im about to sign";
    let signature = signing_key.sign(message); // returns `k256::schnorr::Signature`

    let verifying_key = VerifyingKey::from_bytes(verifying_key_bytes.as_slice()).unwrap();

    // Measure verification time
    c.bench_function("k256_verify", |b| {
        b.iter(|| {
            black_box(verifying_key.verify(message, &signature).unwrap());
        })
    });
}

fn bench_schnorr_fun_sign(c: &mut Criterion) {
    // Use synthetic nonces
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen.clone());
    // Generate your public/private key-pair
    let keypair = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));
    // Sign a variable length message
    let message = schnorr_fun::Message::<Public>::plain(
        "the-times-of-london",
        b"Chancellor on brink of second bailout for banks",
    );
    // Sign the message with our keypair
    c.bench_function("schnorr_fun_sign", |b| {
        b.iter(|| {
            let _ = black_box(schnorr.sign(&keypair, message));
        })
    });
}

fn bench_schnorr_fun_verify(c: &mut Criterion) {
    // Use synthetic nonces
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen.clone());
    // Generate your public/private key-pair
    let keypair = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));
    // Sign a variable length message
    let message = schnorr_fun::Message::<Public>::plain(
        "the-times-of-london",
        b"Chancellor on brink of second bailout for banks",
    );
    // Sign the message with our keypair
    let signature = schnorr.sign(&keypair, message);
    // Get the verifier's key
    let verification_key = keypair.public_key();

    c.bench_function("schnorr_fun_verify", |b| {
        b.iter(|| {
            let _ = black_box(schnorr.verify(&verification_key, message, &signature));
        })
    });
}

// ------------------------------------------- Adaptor signature computation -------------------------------------------

fn bench_schnorr_fun_presign(c: &mut Criterion) {
    // Use synthetic nonces
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

    // Generate your public/private key-pair
    let signing_keypair = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));

    // Generate verification, decryption, encryption keys and message
    let verification_key = signing_keypair.public_key();
    let attestation = Scalar::random(&mut rand::thread_rng());
    let anticipation_point = schnorr.encryption_key_for(&attestation);
    let message = schnorr_fun::Message::<Public>::plain("text-bitcoin", b"send 1 BTC to Bob");

    // Alice knows: signing_keypair, anticipation_point
    // Bob knows: attestation, verification_key

    // Alice creates an encrypted signature and sends it to Bob
    c.bench_function("schnorr_fun_presign", |b| {
        b.iter(|| {
            let pre_signature =
                black_box(schnorr.encrypted_sign(&signing_keypair, &anticipation_point, message));
        })
    });
}

fn bench_secp256k1_zkp_ecdsa_presign(c: &mut Criterion) {
    let mut rng = thread_rng();
    let signing_keypair = Keypair::new(SECP256K1, &mut rng);
    let secret_key = signing_keypair.secret_key();
    let attestation = SecretKey::new(&mut rand::thread_rng());
    let anticipation_point = attestation.public_key(&SECP256K1);

    let mut buf = [0u8; 32];
    thread_rng().fill_bytes(&mut buf);
    let msg = secp256k1_zkp::Message::from_digest_slice(&buf).unwrap();

    c.bench_function("secp256k1_zkp_ecdsa_presign", |b| {
        b.iter(|| {
            let ecdsa_pre_signature = black_box(EcdsaAdaptorSignature::encrypt(
                SECP256K1,
                &msg,
                &secret_key,
                &anticipation_point,
            ));
        })
    });
}

// -------------------------------------------  Key serialization and deserialization -------------------------------------------

fn bench_secp256k1_zkp_key_serialize(c: &mut Criterion) {
    let keypair = Keypair::new(SECP256K1, &mut thread_rng());
    let pubkey = keypair.public_key();
    c.bench_function("secp256k1_zkp_key_serialize", |b| {
        b.iter(|| {
            let _ = black_box(pubkey.serialize());
        })
    });
}

fn bench_secp256k1_zkp_key_deserialize(c: &mut Criterion) {
    let keypair = Keypair::new(SECP256K1, &mut thread_rng());
    let serialized = keypair.public_key().serialize();
    c.bench_function("secp256k1_zkp_key_deserialize", |b| {
        b.iter(|| {
            black_box(secp256k1_zkp::PublicKey::from_slice(&serialized).unwrap());
        })
    });
}

fn bench_k256_key_serialize(c: &mut Criterion) {
    let signing_key = k256::schnorr::SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    c.bench_function("k256_key_serialize", |b| {
        b.iter(|| {
            black_box(verifying_key.to_bytes());
        })
    });
}

fn bench_k256_key_deserialize(c: &mut Criterion) {
    let signing_key = k256::schnorr::SigningKey::random(&mut OsRng);
    let verifying_key_bytes = signing_key.verifying_key().to_bytes();
    c.bench_function("k256_key_deserialize", |b| {
        b.iter(|| {
            black_box(k256::schnorr::VerifyingKey::from_bytes(&verifying_key_bytes).unwrap());
        })
    });
}

fn bench_schnorr_fun_key_serialize(c: &mut Criterion) {
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);
    let kp = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));
    let pubkey = kp.public_key();
    c.bench_function("schnorr_fun_key_serialize", |b| {
        b.iter(|| {
            black_box(pubkey.to_bytes());
        })
    });
}

fn bench_schnorr_fun_key_deserialize(c: &mut Criterion) {
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);
    let kp = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));
    let serialized = kp.public_key().to_bytes();
    c.bench_function("schnorr_fun_key_deserialize", |b| {
        b.iter(|| {
            black_box(
                schnorr_fun::fun::Point::<_, schnorr_fun::fun::marker::NonZero>::from_bytes(
                    serialized,
                )
                .unwrap(),
            );
        })
    });
}

// -------------------------------------------  Adaptor signature cloning -------------------------------------------

fn bench_ecdsa_adaptor_signature_clone(c: &mut Criterion) {
    let mut rng = thread_rng();
    let signing_keypair = Keypair::new(SECP256K1, &mut rng);
    let secret_key = signing_keypair.secret_key();
    let attestation = SecretKey::new(&mut rand::thread_rng());
    let anticipation_point = attestation.public_key(&SECP256K1);

    let mut buf = [0u8; 32];
    thread_rng().fill_bytes(&mut buf);
    let msg = secp256k1_zkp::Message::from_digest_slice(&buf).unwrap();

    let ecdsa_adaptor =
        EcdsaAdaptorSignature::encrypt(SECP256K1, &msg, &secret_key, &anticipation_point);

    c.bench_function("ecdsa_adaptor_signature_clone", |b| {
        b.iter(|| {
            let _ = black_box(ecdsa_adaptor.clone());
        })
    });
}

fn bench_schnorr_adaptor_signature_clone(c: &mut Criterion) {
    // Use synthetic nonces
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

    // Generate your public/private key-pair
    let signing_keypair = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));

    // Generate verification, decryption, encryption keys and message
    let attestation = Scalar::random(&mut rand::thread_rng());
    let anticipation_point = schnorr.encryption_key_for(&attestation);
    let message = schnorr_fun::Message::<Public>::plain("text-bitcoin", b"send 1 BTC to Bob");

    let schnorr_adaptor = schnorr.encrypted_sign(&signing_keypair, &anticipation_point, message);

    c.bench_function("schnorr_adaptor_signature_clone", |b| {
        b.iter(|| {
            let _ = black_box(schnorr_adaptor.clone());
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10000);
    // targets = bench_secp256k1_zkp_sign, bench_secp256k1_zkp_verify, bench_k256_sign, bench_k256_verify, bench_schnorr_fun_sign, bench_schnorr_fun_verify
    // targets = bench_schnorr_fun_presign, bench_secp256k1_zkp_ecdsa_presign
    // targets = bench_secp256k1_zkp_key_serialize, bench_secp256k1_zkp_key_deserialize, bench_k256_key_serialize, bench_k256_key_deserialize, bench_schnorr_fun_key_serialize, bench_schnorr_fun_key_deserialize
    targets = bench_ecdsa_adaptor_signature_clone, bench_schnorr_adaptor_signature_clone
}
criterion_main!(benches);

/*
Porovnavali sme crate K256 a secp256k1-zkp
k256 je napisana plne v Ruste, cize je asi viac memory safe a prenositelna.
secp256k1 je wrapnuta na Cckovske kniznice.

Z toho nam vyslo po benchmarkovani, ze secp256k1 je viac nez 4x rychlejsia nez k256, takze budeme pouzivat tuto.
chceli sme vsak preverit moznost, ci nahodou k256 nebude iba o velmi malicko pomalsia a rozhodli by sme sa pre nu.
avsak, je pomalsia pomerne o dost.
 */
