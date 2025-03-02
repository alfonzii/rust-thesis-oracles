// src/adaptor_signature_scheme/schnorr_zkp_adaptor.rs

use crate::{adaptor_signature_scheme::AdaptorSignatureScheme, common::types};
use secp256k1_zkp::{
    schnorr, Message, PublicKey, SchnorrAdaptorPreSignature, SecretKey, SECP256K1,
};

pub struct SchnorrAdaptorSignatureScheme;

impl AdaptorSignatureScheme for SchnorrAdaptorSignatureScheme {
    type AdaptorSignature = SchnorrAdaptorPreSignature;
    type Signature = schnorr::Signature;

    fn pre_sign(
        signing_key: &SecretKey,
        message: &Message,
        anticipation_point: &PublicKey,
    ) -> Self::AdaptorSignature {
        SchnorrAdaptorPreSignature::presign(
            SECP256K1,
            message,
            &signing_key.keypair(SECP256K1),
            anticipation_point,
        )
    }

    fn pre_verify(
        verification_key: &PublicKey,
        message: &Message,
        anticipation_point: &PublicKey,
        adaptor_signature: &Self::AdaptorSignature,
    ) -> bool {
        // Convert `verification_key` into an x-only public key (for BIP-340).
        let xonly_pk = verification_key.x_only_public_key().0;

        // Extract the public key of the "anticipation point" from the presignature.
        let extracted_ap = match adaptor_signature.extract_adaptor(message, &xonly_pk) {
            Ok(pk) => pk,
            Err(_) => return false,
        };

        // Compare it to the `anticipation_point` we *expected*.
        // If they match, we've confirmed that the presignature commits
        // to the same anticipation point we provided.
        if extracted_ap != *anticipation_point {
            return false;
        }

        // If we get here, the presignature is "valid" in the sense that
        // it corresponds to the claimed anticipation point for this pubkey/message.
        true
    }

    fn adapt(
        adaptor_signature: &Self::AdaptorSignature,
        attestation: &SecretKey,
    ) -> Self::Signature {
        adaptor_signature
            .adapt(attestation)
            .expect("Failed to adapt schnorr signature")
    }

    fn extract(
        signature: &Self::Signature,
        adaptor_signature: &Self::AdaptorSignature,
        _anticipation_point: &PublicKey,
    ) -> types::Attestation {
        adaptor_signature
            .extract_secadaptor(signature)
            .expect("Failed to extract secret adaptor")
    }
}
