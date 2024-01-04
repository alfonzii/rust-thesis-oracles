use blsful::inner_types;
use blsful::inner_types::elliptic_curve::ops::Add;
use blsful::inner_types::elliptic_curve::ops::AddAssign;
use blsful::inner_types::elliptic_curve::ops::Mul;
use blsful::inner_types::elliptic_curve::ops::Neg;

use blsful::inner_types::Curve;
use blsful::inner_types::ExpandMsgXmd;
use blsful::inner_types::G1Projective;
use blsful::inner_types::G2Projective;
use blsful::inner_types::Group;
use blsful::inner_types::GroupEncoding;
use blsful::inner_types::Gt;
use blsful::inner_types::Scalar;
use blsful::vsss_rs::elliptic_curve::ops::MulByGenerator;
use blsful::vsss_rs::subtle::Choice;
use blsful::vsss_rs::subtle::CtOption;
use blsful::Bls12381G2Impl;
use blsful::PublicKey;
use blsful::SecretKey;
use blsful::Signature;
use blsful::SignatureSchemes;
use rand::Rng;

use sha2::{Digest, Sha256};

// Witness encryption based on BLS signatures

fn main() {
    test_pairing();
    test_random();

    test_debug();
}

fn enc((vk_, m_): (&PublicKey<Bls12381G2Impl>, &[u8]), m: &[u8; 32]) -> (G1Projective, Gt, [u8; 32]) {
    // Prepare valid random scalar for r1
    let mut rng = rand::thread_rng();
    let mut montgomery = [0u64; 4];
    let mut scal: CtOption<Scalar> = CtOption::new(Scalar::default(), Choice::from(0));
    while scal.is_none().into() {
        rng.fill(&mut montgomery[..]);
        scal = inner_types::Scalar::from_raw(&montgomery);
    }

    // Sample r1 <- Z_q and r2 <- G_t
    let r1 = scal.expect("Failed to generate a valid scalar from raw");
    let r2 = Gt::random(rng.clone());

    // Set c1 := g1^r1
    let c1 = G1Projective::mul_by_generator(&r1);

    // Compute h := H_1(r2)
    let r2_bytes = r2.to_bytes();
    let mut hasher = Sha256::new();
    hasher.update(r2_bytes);
    let h = hasher.finalize();

    // Compute c2 := ( e(vk_, H_0(m_))^r1 * r2 )
    let h0 = G2Projective::hash::<ExpandMsgXmd<sha2::Sha256>>(m_, b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_");
    let mut pairing_result = inner_types::pairing(&vk_.0.to_affine(), &h0.to_affine());
    pairing_result = pairing_result.mul(&r1);
    pairing_result.add_assign(&r2);
    let c2 = pairing_result;

    // Compute c3 := (h + m) -- xor
    let mut c3 = [0u8; 32];
    for i in 0..32 {
        c3[i] = m[i] ^ h[i];
    }
    let c3 = c3;

    // Return c := (c1, c2, c3)
    (c1, c2, c3)
}

fn dec(sig_: Signature<Bls12381G2Impl>, (c1, c2, c3): (G1Projective, Gt, [u8; 32])) -> [u8; 32] {
    // Parse c := (c1, c2, c3)
    // trivially done

    // Compute r := c2 * e(c1, sig_)^-1
    let pairing_result = inner_types::pairing(&c1.to_affine(), &sig_.as_raw_value().to_affine());
    let neg_p_result = pairing_result.neg();
    let r = neg_p_result.add(&c2);

    // Compute h := H_1(r)
    let r_bytes = r.to_bytes();
    let mut hasher = Sha256::new();
    hasher.update(r_bytes);
    let h = hasher.finalize();

    // Compute m := c3 ^ h
    let mut m = [0u8; 32];
    for i in 0..32 {
        m[i] = c3[i] ^ h[i];
    }
    let m = m;

    m
}

fn enc_debug((vk_, m_): (&PublicKey<Bls12381G2Impl>, &[u8]), m: &[u8; 32], sig_: &Signature<Bls12381G2Impl>) -> (G1Projective, Gt, [u8; 32]) {
    println!("STARTING ENC_DEBUG FUNCTION");

    // Prepare scalar for r1
    let montgomery: [u64; 4] = [500, 1000, 100123124, 45423123411];
    let scal = inner_types::Scalar::from_raw(&montgomery);

    let r2seed: [u8; 576] = [
        9, 158, 240, 113, 249, 243, 23, 217, 164, 70, 246, 171, 49, 21, 64, 141, 200, 171, 149, 147, 102, 40, 205, 40, 124, 103, 15, 35, 185, 167, 48, 243, 26, 156, 210, 189, 114, 107, 115, 21,
        213, 188, 211, 127, 97, 35, 79, 148, 11, 100, 63, 188, 213, 170, 90, 153, 21, 170, 99, 208, 30, 173, 138, 228, 157, 149, 102, 81, 255, 37, 87, 82, 55, 189, 39, 43, 139, 52, 106, 165, 9,
        12, 173, 158, 7, 252, 201, 184, 134, 161, 89, 106, 60, 220, 160, 187, 0, 182, 49, 43, 1, 94, 217, 8, 248, 127, 21, 168, 13, 134, 39, 26, 90, 197, 194, 221, 63, 203, 52, 184, 211, 108,
        249, 109, 163, 121, 175, 114, 166, 101, 235, 154, 170, 109, 122, 209, 103, 81, 86, 224, 58, 239, 169, 7, 13, 103, 36, 49, 246, 37, 246, 178, 116, 90, 220, 134, 22, 113, 89, 209, 132, 56,
        95, 180, 31, 229, 63, 31, 176, 184, 233, 65, 160, 241, 184, 76, 14, 97, 229, 188, 16, 33, 248, 154, 24, 104, 227, 214, 210, 187, 103, 223, 14, 164, 124, 34, 151, 97, 22, 175, 169, 110,
        183, 179, 52, 182, 172, 234, 153, 38, 12, 138, 54, 171, 243, 116, 87, 149, 15, 30, 5, 50, 48, 182, 126, 133, 249, 216, 27, 5, 105, 80, 33, 112, 52, 226, 38, 188, 14, 104, 3, 193, 136,
        102, 4, 69, 218, 104, 55, 231, 124, 94, 97, 11, 147, 35, 20, 17, 193, 3, 249, 135, 194, 9, 215, 216, 148, 197, 125, 25, 18, 32, 113, 32, 233, 21, 87, 243, 37, 66, 69, 237, 141, 97, 245,
        235, 124, 119, 15, 107, 177, 166, 16, 85, 175, 100, 172, 92, 30, 62, 10, 94, 47, 141, 132, 231, 122, 19, 54, 230, 253, 151, 155, 131, 67, 36, 75, 173, 94, 15, 199, 157, 254, 151, 91,
        177, 156, 188, 121, 113, 46, 85, 196, 244, 196, 233, 1, 163, 96, 100, 123, 132, 36, 34, 181, 36, 21, 198, 50, 173, 91, 158, 158, 231, 57, 100, 2, 138, 202, 48, 181, 125, 32, 251, 37,
        122, 87, 116, 17, 78, 187, 143, 190, 81, 240, 209, 144, 143, 164, 118, 9, 145, 55, 210, 11, 21, 38, 9, 11, 167, 71, 15, 160, 161, 126, 114, 178, 121, 59, 79, 81, 159, 47, 99, 238, 176,
        102, 22, 142, 251, 238, 230, 63, 251, 250, 209, 157, 132, 147, 68, 226, 5, 9, 199, 81, 248, 200, 20, 46, 134, 113, 161, 10, 76, 153, 101, 92, 58, 255, 62, 138, 212, 74, 204, 249, 255,
        66, 138, 180, 2, 33, 150, 105, 101, 129, 186, 150, 198, 201, 230, 166, 227, 97, 60, 159, 155, 4, 186, 26, 75, 34, 98, 184, 162, 40, 125, 180, 136, 208, 250, 7, 143, 114, 79, 185, 26,
        206, 183, 193, 165, 59, 70, 202, 230, 151, 115, 197, 69, 38, 116, 92, 180, 7, 242, 88, 198, 244, 64, 89, 134, 124, 202, 56, 8, 75, 64, 60, 161, 173, 116, 13, 170, 213, 177, 94, 79, 255,
        65, 5, 102, 9, 134, 239, 112, 35, 121, 73, 149, 192, 71, 27, 44, 5, 255, 72, 82, 120, 138, 236, 152, 137, 34, 174, 17, 30, 67, 217, 138, 170, 194, 174, 108, 17, 62, 172, 32, 80, 35, 101,
        198, 165, 144, 58, 226, 31, 100,
    ];

    // Fixed r1 <- Z_q and r2 <- G_t
    let r1 = scal.expect("Failed to generate a valid scalar from raw");
    let mut r2_bytes = <Gt as GroupEncoding>::Repr::default();
    r2_bytes.as_mut().copy_from_slice(&r2seed);
    let r2 = Gt::from_bytes(&r2_bytes).expect("Failed to generate a valid scalar from raw");

    // Set c1 := g1^r1
    let c1 = G1Projective::mul_by_generator(&r1);

    // Compute h := H_1(r2)
    let mut hasher = Sha256::new();
    hasher.update(r2_bytes);
    let h = hasher.finalize();

    println!("ENC_DEBUG h: {:?}", h);

    // Compute c2 := ( e(vk_, H_0(m_))^r1 * r2 )
    let h0 = G2Projective::hash::<ExpandMsgXmd<sha2::Sha256>>(m_, b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_");
    let mut pairing_result = inner_types::pairing(&vk_.0.to_affine(), &h0.to_affine());
    pairing_result = pairing_result.mul(&r1);
    let beta = pairing_result;
    pairing_result.add_assign(&r2);
    let c2 = pairing_result;

    // TEST
    let beta2 = inner_types::pairing(&c1.to_affine(), &sig_.as_raw_value().to_affine());
    // assert that beta and beta2 are equal
    assert_eq!(beta, beta2);
    println!("beta == beta2: {:?}", beta == beta2);
    //------------------------------------------------------------------------------------------

    // Compute c3 := (h + m) -- xor
    let mut c3 = [0u8; 32];
    for i in 0..32 {
        c3[i] = m[i] ^ h[i];
    }
    let c3 = c3;

    println!("FINISHED ENC_DEBUG FUNCTION\n");

    // Return c := (c1, c2, c3)
    (c1, c2, c3)
}

fn dec_debug(sig_: Signature<Bls12381G2Impl>, (c1, c2, c3): (G1Projective, Gt, [u8; 32])) -> [u8; 32] {
    println!("STARTING DEC FUNCTION");

    // Parse c := (c1, c2, c3)
    // trivially done

    // Compute r := c2 * e(c1, sig_)^-1
    let pairing_result = inner_types::pairing(&c1.to_affine(), &sig_.as_raw_value().to_affine());
    let neg_p_result = pairing_result.neg();
    let r = neg_p_result.add(&c2);

    // Compute h := H_1(r)
    let r_bytes = r.to_bytes();
    let mut hasher = Sha256::new();
    hasher.update(r_bytes);
    let h = hasher.finalize();

    println!("DEC h: {:?}", h);

    // Compute m := c3 ^ h
    let mut m = [0u8; 32];
    for i in 0..32 {
        m[i] = c3[i] ^ h[i];
    }
    let m = m;

    // Return m
    println!("Decrypted message: {:?}", m);

    println!("FINISHED DEC FUNCTION");
    m
}

// ------------------------------------ TESTS --------------------------------------

fn test_debug() {
    let seed: [u8; 32] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    let sk = SecretKey::<Bls12381G2Impl>::from_be_bytes(&seed).expect("a valid secret key");
    let pk = PublicKey::from(&sk);
    let m_ = b"00000000-0000-0000-0000-00000000";
    let sig = SecretKey::sign(&sk, SignatureSchemes::Basic, m_).expect("a valid signature");

    match sig.verify(&pk, m_) {
        Ok(()) => println!("[test_debug]: Correct - Signature is valid"),
        Err(err) => println!("Error - Invalid signature: {:?}", err),
    }

    let m = [2u8; 32];

    println!("[test_debug]: sk: {:?}\n", sk);
    println!("[test_debug]: pk: {:?}\n", pk);
    println!("[test_debug]: m_: {:?}\n", m_);
    println!("[test_debug]: sig: {:?}\n", sig);
    println!("[test_debug]: m: {:?}\n", m);

    let (c1, c2, c3) = enc_debug((&pk, m_), &m, &sig);
    let ret_m = dec_debug(sig, (c1, c2, c3));

    // assert that m (as bytes) and ret_m are equal
    assert_eq!(m, ret_m);
    println!("[test_debug]: m == ret_m: {:?}\n\n", m == ret_m);
}

fn test_random() {
    let sk = SecretKey::<Bls12381G2Impl>::new();
    let pk = PublicKey::from(&sk);
    let msg = b"00000000-0000-0000-0000-000000000000";

    let sig = SecretKey::sign(&sk, SignatureSchemes::Basic, msg).expect("a valid signature");
    let m = [1u8; 32];

    let (c1, c2, c3) = enc((&pk, msg), &m);
    let ret_m = dec(sig, (c1, c2, c3));

    assert_eq!(m, ret_m);
    println!("[test_random]: m == ret_m: {:?}\n\n", m == ret_m);
}

fn test_pairing() {
    let seed: [u8; 32] = [1, 2, 3, 4, 5, 6, 7, 80, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    let sk = SecretKey::<Bls12381G2Impl>::from_be_bytes(&seed).expect("a valid secret key");
    let pk = PublicKey::from(&sk);
    let msg = b"00000000-0000-0000-0000-000000000000";
    let sig = SecretKey::sign(&sk, SignatureSchemes::Basic, msg).expect("a valid signature");

    match sig.verify(&pk, msg) {
        Ok(()) => println!("[test_pairing]: Correct - Signature is valid"),
        Err(err) => println!("[test_pairing]: Error - Invalid signature: {:?}", err),
    }

    //BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_NUL_
    //BLS12381G1_XMD:SHA-256_SSWU_RO_
    let h = G2Projective::hash::<ExpandMsgXmd<sha2::Sha256>>(msg, b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_");

    let pair1 = inner_types::pairing(&inner_types::G1Projective::GENERATOR.to_affine(), &sig.as_raw_value().to_affine());
    let pair2 = inner_types::pairing(&pk.0.to_affine(), &h.to_affine());

    assert_eq!(pair1, pair2);
    println!("[test_pairing]: pair1 == pair2: {:?}\n\n", pair1 == pair2);
}
