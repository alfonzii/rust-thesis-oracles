use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::thread_rng;
use secp256k1_zkp::Secp256k1;

// Import necessary types and functions
use dlc_fast_prototyping::adaptor_signature_scheme::{
    AdaptorSignatureScheme, EcdsaAdaptorSignatureScheme,
};
use dlc_fast_prototyping::common::fun; // contains create_cet and create_message
use dlc_fast_prototyping::common::types::OutcomeU32;
use dlc_fast_prototyping::crypto_utils::simple_crypto_utils::SimpleCryptoUtils;
use dlc_fast_prototyping::crypto_utils::CryptoUtils;

fn bench_create_cet(c: &mut Criterion) {
    let total_collateral = 1000;
    let payout = 400;
    c.bench_function("create_cet", |b| {
        b.iter(|| {
            let cet = black_box(fun::create_cet(payout, total_collateral));
            black_box(cet)
        })
    });
}

fn bench_create_message(c: &mut Criterion) {
    let cet_str = "Alice gets 600 sats and Bob gets 400 sats".to_string();
    c.bench_function("create_message", |b| {
        b.iter(|| {
            let msg = black_box(fun::create_message(&cet_str)).unwrap();
            black_box(msg)
        })
    });
}

fn bench_compute_anticipation_point(c: &mut Criterion) {
    let secp = Secp256k1::new();
    let (_, oracle_pub) = secp.generate_keypair(&mut thread_rng());
    let (_, oracle_nonce) = secp.generate_keypair(&mut thread_rng());
    let outcome = OutcomeU32::from(5);
    c.bench_function("compute_anticipation_point", |b| {
        b.iter(|| {
            let atp = black_box(SimpleCryptoUtils::compute_anticipation_point(
                &oracle_pub,
                &oracle_nonce,
                &outcome,
            ))
            .unwrap();
            black_box(atp)
        })
    });
}

fn bench_pre_sign(c: &mut Criterion) {
    let secp = Secp256k1::new();
    let (signing_sk, _signing_pk) = secp.generate_keypair(&mut thread_rng());
    let cet_str = "Alice gets 600 sats and Bob gets 400 sats".to_string();
    let msg = fun::create_message(&cet_str).unwrap();
    // For anticipation point, generate dummy keys:
    let (_, oracle_pub) = secp.generate_keypair(&mut thread_rng());
    let (_, oracle_nonce) = secp.generate_keypair(&mut thread_rng());
    let outcome = OutcomeU32::from(5);
    let atp_point =
        SimpleCryptoUtils::compute_anticipation_point(&oracle_pub, &oracle_nonce, &outcome)
            .unwrap();
    c.bench_function("pre_sign", |b| {
        b.iter(|| {
            let _ = black_box(EcdsaAdaptorSignatureScheme::pre_sign(
                &signing_sk,
                &msg,
                &atp_point,
            ));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10000);
    targets = bench_create_cet, bench_create_message, bench_compute_anticipation_point, bench_pre_sign
}
criterion_main!(benches);
