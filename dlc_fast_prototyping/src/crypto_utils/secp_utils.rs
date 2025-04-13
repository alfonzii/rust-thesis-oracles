/*
MIT License

Copyright (c) 2020 p2pderivatives

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

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
    public_key: &PublicKey,
    public_nonce: &PublicKey,
    outcome: &impl types::Outcome,
) -> Result<types::AnticipationPoint, secp256k1_zkp::Error> {
    let hash = create_schnorr_hash(public_key, public_nonce, outcome);
    let scalar = Scalar::from_be_bytes(hash).unwrap();
    let tweaked = public_key.mul_tweak(secp, &scalar)?;
    Ok(public_nonce.combine(&tweaked)?)
}

/// Compute an oracle attestation for the given private key, private nonce and digit index. (JUST FOR TESTING! IN REAL SCENARIO, ORACLE SHOULD DO THIS)
pub(in crate::crypto_utils) fn schnorrsig_compute_oracle_attestation<C: Verification + Signing>(
    secp: &Secp256k1<C>,
    private_key: &SecretKey,
    private_nonce: &SecretKey,
    outcome: &impl types::Outcome,
) -> Result<types::Attestation, secp256k1_zkp::Error> {
    let hash = create_schnorr_hash(
        &PublicKey::from_secret_key(secp, private_key),
        &PublicKey::from_secret_key(secp, private_nonce),
        outcome,
    );
    let scalar = Scalar::from_be_bytes(hash).unwrap();
    let tweaked = Scalar::from(private_key.mul_tweak(&scalar)?);
    Ok(private_nonce.add_tweak(&tweaked)?)
}

/// Create a BIP340 hash for the given digit index (which is 1), nonce and public key.
fn create_schnorr_hash(
    public_key: &PublicKey,
    public_nonce: &PublicKey,
    outcome: &impl types::Outcome,
) -> [u8; 32] {
    let mut buf = Vec::<u8>::new();
    buf.extend(public_nonce.serialize());
    buf.extend(public_key.serialize());
    buf.extend(outcome.serialize());
    BIP340Hash::hash(&buf).to_byte_array()
}
