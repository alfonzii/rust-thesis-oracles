use bitcoin::hashes::{sha256t_hash_newtype, Hash};
use secp256k1_zkp::{PublicKey, Scalar, Secp256k1, SecretKey, Signing, Verification};

use crate::common::types;

const BIP340_MIDSTATE: [u8; 32] = [
    0x9c, 0xec, 0xba, 0x11, 0x23, 0x92, 0x53, 0x81, 0x11, 0x67, 0x91, 0x12, 0xd1, 0x62, 0x7e, 0x0f,
    0x97, 0xc8, 0x75, 0x50, 0x00, 0x3c, 0xc7, 0x65, 0x90, 0xf6, 0x11, 0x64, 0x33, 0xe9, 0xb6, 0x6a,
];

sha256t_hash_newtype! {
    /// BIP340 Hash Tag
    pub struct BIP340HashTag = raw(BIP340_MIDSTATE, 64);

    /// BIP340 Hash
    #[hash_newtype(backward)]
    pub struct BIP340Hash(_);
}

/// Compute an anticipation point for the given public key, nonce and message.
pub(in crate::crypto_utils) fn schnorrsig_compute_anticipation_point<C: Verification>(
    secp: &Secp256k1<C>,
    pub_key: &PublicKey,
    pub_nonce: &PublicKey,
    outcome: &impl types::Outcome,
) -> Result<PublicKey, secp256k1_zkp::Error> {
    let hash = create_schnorr_hash(pub_key, pub_nonce, outcome);
    let scalar = Scalar::from_be_bytes(hash).unwrap();
    let tweaked = pub_key.mul_tweak(secp, &scalar)?;
    Ok(pub_nonce.combine(&tweaked)?)
}

/// Compute an oracle attestation for the given private key, private nonce and digit index. (JUST FOR TESTING! IN REAL SCENARIO, ORACLE SHOULD DO THIS)
pub(in crate::crypto_utils) fn schnorrsig_compute_oracle_attestation<C: Verification + Signing>(
    secp: &Secp256k1<C>,
    priv_key: &SecretKey,
    priv_nonce: &SecretKey,
    outcome: &impl types::Outcome,
) -> Result<SecretKey, secp256k1_zkp::Error> {
    let hash = create_schnorr_hash(
        &PublicKey::from_secret_key(secp, priv_key),
        &PublicKey::from_secret_key(secp, priv_nonce),
        outcome,
    );
    let scalar = Scalar::from_be_bytes(hash).unwrap();
    let tweaked = Scalar::from(priv_key.mul_tweak(&scalar)?);
    Ok(priv_nonce.add_tweak(&tweaked)?)
}

/// Create a BIP340 hash for the given digit index (which is 1), nonce and public key.
fn create_schnorr_hash(
    pub_key: &PublicKey,
    pub_nonce: &PublicKey,
    outcome: &impl types::Outcome,
) -> [u8; 32] {
    let mut buf = Vec::<u8>::new();
    buf.extend(pub_nonce.serialize());
    buf.extend(pub_key.serialize());
    buf.extend(outcome.serialize()); // TODO: mozno nejaka optimalizacia
    BIP340Hash::hash(&buf).to_byte_array()
}
