use criterion::{black_box, criterion_group, criterion_main, Criterion};

use secp256k1_zkp::{global::SECP256K1, rand::thread_rng, Keypair, Message};

use k256::schnorr::{
    signature::{Signer, Verifier},
    SigningKey, VerifyingKey,
};

use rand_core::{OsRng, RngCore};

fn bench_secp256k1_zkp_sign(c: &mut Criterion) {
    let mut rng = thread_rng();
    let keypair = Keypair::new(SECP256K1, &mut rng);

    let mut buf = [0u8; 32];
    thread_rng().fill_bytes(&mut buf);
    let msg = Message::from_digest_slice(&buf).unwrap();

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
    let msg = Message::from_digest_slice(&buf).unwrap();
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

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::new(10, 0)).sample_size(10000);
    targets = bench_secp256k1_zkp_sign, bench_secp256k1_zkp_verify, bench_k256_sign, bench_k256_verify
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
