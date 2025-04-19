use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dlc_fast_prototyping::config::runparams::{MyAdaptorSignatureScheme, MyCryptoUtils};
use rand::thread_rng;
use secp256k1_zkp::{Keypair, Secp256k1};

// Import necessary types and functions
use dlc_fast_prototyping::adaptor_signature_scheme::AdaptorSignatureScheme;
use dlc_fast_prototyping::common::fun; // contains create_cet and create_message
use dlc_fast_prototyping::common::types::OutcomeU32;
use dlc_fast_prototyping::crypto_utils::CryptoUtils;

const POW2_20SUB1: u32 = 1_048_575; // twenty bits set to 1 in binary
const POW2_10SUB1: u32 = 1023; // ten bits set to 1 in binary

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
    let crypto_utils_engine = MyCryptoUtils::new(&oracle_pub, &oracle_nonce);
    let outcome = OutcomeU32::from(POW2_10SUB1);
    c.bench_function("compute_anticipation_point", |b| {
        b.iter(|| {
            let atp = black_box(crypto_utils_engine.compute_anticipation_point(&outcome)).unwrap();
            black_box(atp)
        })
    });
}

fn bench_pre_sign(c: &mut Criterion) {
    let secp = Secp256k1::new();
    let keypair = Keypair::new(&secp, &mut thread_rng());
    let cet_str = "Alice gets 600 sats and Bob gets 400 sats".to_string();
    let msg = fun::create_message(&cet_str).unwrap();
    // For anticipation point, generate dummy keys:
    let (_, oracle_pub) = secp.generate_keypair(&mut thread_rng());
    let (_, oracle_nonce) = secp.generate_keypair(&mut thread_rng());
    let crypto_utils_engine = MyCryptoUtils::new(&oracle_pub, &oracle_nonce);
    let outcome = OutcomeU32::from(POW2_10SUB1);
    let atp_point = crypto_utils_engine
        .compute_anticipation_point(&outcome)
        .unwrap();
    c.bench_function("pre_sign", |b| {
        b.iter(|| {
            let _ = MyAdaptorSignatureScheme::pre_sign(&keypair, &msg, &atp_point);
        })
    });
}

fn bench_verify_adaptor(c: &mut Criterion) {
    use dlc_fast_prototyping::adaptor_signature_scheme::AdaptorSignatureScheme;
    use rand::thread_rng;
    use secp256k1_zkp::Secp256k1;

    let secp = Secp256k1::new();
    let keypair = Keypair::new(&secp, &mut thread_rng());
    let (_, oracle_pk) = secp.generate_keypair(&mut thread_rng());
    let (_, oracle_nonce) = secp.generate_keypair(&mut thread_rng());

    let crypto_utils_engine = MyCryptoUtils::new(&oracle_pk, &oracle_nonce);

    let outcome = OutcomeU32::from(POW2_10SUB1);
    let cet_str = fun::create_cet(400, 1000);
    let msg = fun::create_message(&cet_str).unwrap();
    let atp_point = crypto_utils_engine
        .compute_anticipation_point(&outcome)
        .unwrap();

    let adaptor_sig = MyAdaptorSignatureScheme::pre_sign(&keypair, &msg, &atp_point);
    c.bench_function("verify_adaptor_sig", |b| {
        b.iter(|| {
            let _check = black_box(MyAdaptorSignatureScheme::pre_verify(
                &keypair.public_key(),
                &msg,
                &atp_point,
                &adaptor_sig,
            ));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10000);
    targets = bench_create_cet, bench_create_message, bench_compute_anticipation_point, bench_pre_sign, bench_verify_adaptor
    // targets = bench_compute_anticipation_point
}
criterion_main!(benches);
