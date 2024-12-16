use crate::adaptor_signatures::AdaptorSignatureScheme;

use secp256k1_zkp::{
    rand::thread_rng, EcdsaAdaptorSignature, Message, PublicKey, Secp256k1, SecretKey,
};

pub struct EcdsaZkpAdaptorSignatureScheme {
    secp: Secp256k1<secp256k1_zkp::All>,
}

impl EcdsaZkpAdaptorSignatureScheme {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        Self { secp }
    }
}

impl AdaptorSignatureScheme for EcdsaZkpAdaptorSignatureScheme {
    type SigningKey = SecretKey;
    type VerificationKey = PublicKey;
    type AnticipationPoint = PublicKey; // Encryption key
    type Attestation = SecretKey; // Decryption key
    type PreSignature = EcdsaAdaptorSignature;
    type Signature = secp256k1_zkp::ecdsa::Signature;
    type Message = Message;

    fn pre_sign(
        &self,
        signing_key: &Self::SigningKey,
        message: &Self::Message,
        atp_point: &Self::AnticipationPoint,
    ) -> Self::PreSignature {
        let mut rng = thread_rng();
        EcdsaAdaptorSignature::encrypt_with_rng(
            &self.secp,
            message,
            signing_key,
            atp_point,
            &mut rng,
        )
    }

    fn pre_verify(
        &self,
        verification_key: &Self::VerificationKey,
        message: &Self::Message,
        atp_point: &Self::AnticipationPoint,
        pre_signature: &Self::PreSignature,
    ) -> bool {
        pre_signature
            .verify(&self.secp, message, verification_key, atp_point)
            .is_ok()
    }

    fn adapt(
        &self,
        pre_signature: &Self::PreSignature,
        attestation: &Self::Attestation,
    ) -> Self::Signature {
        pre_signature
            .decrypt(attestation)
            .expect("Failed to decrypt signature")
    }

    fn extract(
        &self,
        signature: &Self::Signature,
        pre_signature: &Self::PreSignature,
        atp_point: &Self::AnticipationPoint,
    ) -> Self::Attestation {
        pre_signature
            .recover(&self.secp, signature, atp_point)
            .expect("Failed to recover attestation")
    }
}
