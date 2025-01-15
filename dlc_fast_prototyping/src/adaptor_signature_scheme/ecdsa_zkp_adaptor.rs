// src/adaptor_signature_scheme/ecdsa_zkp_adaptor.rs
use crate::adaptor_signature_scheme::AdaptorSignatureScheme;

use secp256k1_zkp::{rand::thread_rng, EcdsaAdaptorSignature, SECP256K1};

pub struct EcdsaAdaptorSignatureScheme;

// We are using SECP global context, but it might be created as a static variable if needed.

impl AdaptorSignatureScheme for EcdsaAdaptorSignatureScheme {
    type AdaptorSignature = EcdsaAdaptorSignature;
    type Signature = secp256k1_zkp::ecdsa::Signature;

    fn pre_sign(
        signing_key: &secp256k1_zkp::SecretKey,
        message: &secp256k1_zkp::Message,
        anticipation_point: &secp256k1_zkp::PublicKey,
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
        verification_key: &secp256k1_zkp::PublicKey,
        message: &secp256k1_zkp::Message,
        anticipation_point: &secp256k1_zkp::PublicKey,
        adaptor_signature: &Self::AdaptorSignature,
    ) -> bool {
        adaptor_signature
            .verify(SECP256K1, message, verification_key, anticipation_point)
            .is_ok()
    }

    fn adapt(
        adaptor_signature: &Self::AdaptorSignature,
        attestation: &secp256k1_zkp::SecretKey,
    ) -> Self::Signature {
        adaptor_signature
            .decrypt(attestation)
            .expect("Failed to decrypt signature")
    }

    fn extract(
        signature: &Self::Signature,
        adaptor_signature: &Self::AdaptorSignature,
        anticipation_point: &secp256k1_zkp::PublicKey,
    ) -> secp256k1_zkp::SecretKey {
        adaptor_signature
            .recover(SECP256K1, signature, anticipation_point)
            .expect("Failed to recover attestation")
    }
}
