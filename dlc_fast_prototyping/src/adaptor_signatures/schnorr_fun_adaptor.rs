use crate::adaptor_signatures::AdaptorSignatureScheme;

use rand::rngs::ThreadRng;

use schnorr_fun::{
    adaptor::{Adaptor, EncryptedSign},
    fun::{marker::*, nonce},
    Message, Schnorr,
};
use sha2::Sha256;

pub struct SchnorrFunAdaptorSignatureScheme {
    schnorr: Schnorr<Sha256, nonce::Synthetic<Sha256, nonce::GlobalRng<ThreadRng>>>,
}

impl SchnorrFunAdaptorSignatureScheme {
    pub fn new() -> Self {
        let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
        let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);
        Self { schnorr }
    }
}

impl AdaptorSignatureScheme for SchnorrFunAdaptorSignatureScheme {
    type SigningKey = schnorr_fun::fun::KeyPair<EvenY>;
    type VerificationKey = schnorr_fun::fun::Point<EvenY>;
    type AnticipationPoint = schnorr_fun::fun::Point;
    type Attestation = schnorr_fun::fun::Scalar;
    type PreSignature = schnorr_fun::adaptor::EncryptedSignature;
    type Signature = schnorr_fun::Signature;
    type Message = String;

    fn pre_sign(
        &self,
        signing_key: &Self::SigningKey,
        message: &Self::Message,
        atp_point: &Self::AnticipationPoint,
    ) -> Self::PreSignature {
        let msg = Message::<Public>::plain("text-bitcoin", message.as_bytes());

        self.schnorr.encrypted_sign(signing_key, atp_point, msg)
    }

    fn pre_verify(
        &self,
        verification_key: &Self::VerificationKey,
        message: &Self::Message,
        atp_point: &Self::AnticipationPoint,
        pre_signature: &Self::PreSignature,
    ) -> bool {
        let msg = Message::<Public>::plain("text-bitcoin", message.as_bytes());

        self.schnorr
            .verify_encrypted_signature(verification_key, atp_point, msg, pre_signature)
    }

    fn adapt(
        &self,
        pre_signature: &Self::PreSignature,
        attestation: &Self::Attestation,
    ) -> Self::Signature {
        self.schnorr
            .decrypt_signature(attestation.clone(), pre_signature.clone())
    }

    fn extract(
        &self,
        signature: &Self::Signature,
        pre_signature: &Self::PreSignature,
        atp_point: &Self::AnticipationPoint,
    ) -> Self::Attestation {
        self.schnorr
            .recover_decryption_key(atp_point, pre_signature, signature)
            .unwrap() // TODO: UNSAFE WITH UNWRAP FOR NOW
    }
}
