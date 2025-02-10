// src/adaptor_signature_scheme/mod.rs

use secp256k1_zkp::{Message, PublicKey, SecretKey};

pub trait AdaptorSignatureScheme {
    type AdaptorSignature: Clone;
    type Signature;

    fn pre_sign(
        signing_key: &SecretKey,
        message: &Message,
        anticipation_point: &PublicKey,
    ) -> Self::AdaptorSignature;

    fn pre_verify(
        verification_key: &PublicKey,
        message: &Message,
        anticipation_point: &PublicKey,
        adaptor_signature: &Self::AdaptorSignature,
    ) -> bool;

    fn adapt(
        adaptor_signature: &Self::AdaptorSignature,
        attestation: &SecretKey,
    ) -> Self::Signature;

    fn extract(
        signature: &Self::Signature,
        adaptor_signature: &Self::AdaptorSignature,
        anticipation_point: &PublicKey,
    ) -> types::Attestation;
}

mod ecdsa_zkp_adaptor;
pub use ecdsa_zkp_adaptor::EcdsaAdaptorSignatureScheme;

use crate::common::types;
