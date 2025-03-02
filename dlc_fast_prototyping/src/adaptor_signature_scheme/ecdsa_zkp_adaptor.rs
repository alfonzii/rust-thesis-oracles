// src/adaptor_signature_scheme/ecdsa_zkp_adaptor.rs
use crate::{adaptor_signature_scheme::AdaptorSignatureScheme, common::types};

use secp256k1_zkp::{
    ecdsa, rand::thread_rng, EcdsaAdaptorSignature, Message, PublicKey, SecretKey, SECP256K1,
};

pub struct EcdsaAdaptorSignatureScheme;

// We are using SECP global context, but it might be created as a static variable if needed.

impl AdaptorSignatureScheme for EcdsaAdaptorSignatureScheme {
    type AdaptorSignature = EcdsaAdaptorSignature;
    type Signature = ecdsa::Signature;

    fn pre_sign(
        signing_key: &SecretKey,
        message: &Message,
        anticipation_point: &PublicKey,
    ) -> Self::AdaptorSignature {
        let mut rng = thread_rng();
        EcdsaAdaptorSignature::encrypt_with_rng(
            SECP256K1,
            message,
            signing_key,
            anticipation_point,
            &mut rng,
        )
    }

    fn pre_verify(
        verification_key: &PublicKey,
        message: &Message,
        anticipation_point: &PublicKey,
        adaptor_signature: &Self::AdaptorSignature,
    ) -> bool {
        adaptor_signature
            .verify(SECP256K1, message, verification_key, anticipation_point)
            .is_ok()
    }

    fn adapt(
        adaptor_signature: &Self::AdaptorSignature,
        attestation: &SecretKey,
    ) -> Self::Signature {
        adaptor_signature
            .decrypt(attestation)
            .expect("Failed to decrypt signature")
    }

    fn extract(
        signature: &Self::Signature,
        adaptor_signature: &Self::AdaptorSignature,
        anticipation_point: &PublicKey,
    ) -> types::Attestation {
        adaptor_signature
            .recover(SECP256K1, signature, anticipation_point)
            .expect("Failed to recover attestation")
    }
}
