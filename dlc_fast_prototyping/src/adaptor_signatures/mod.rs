pub trait AdaptorSignatureScheme {
    type SigningKey;
    type VerificationKey;
    type AnticipationPoint;
    type Attestation;
    type PreSignature;
    type Signature;
    type Message;

    fn pre_sign(
        &self,
        signing_key: &Self::SigningKey,
        message: &Self::Message,
        atp_point: &Self::AnticipationPoint,
    ) -> Self::PreSignature;

    fn pre_verify(
        &self,
        verification_key: &Self::VerificationKey,
        message: &Self::Message,
        atp_point: &Self::AnticipationPoint,
        pre_signature: &Self::PreSignature,
    ) -> bool;

    fn adapt(
        &self,
        pre_signature: &Self::PreSignature,
        attestation: &Self::Attestation,
    ) -> Self::Signature;

    fn extract(
        &self,
        signature: &Self::Signature,
        pre_signature: &Self::PreSignature,
        atp_point: &Self::AnticipationPoint,
    ) -> Self::Attestation;
}

mod schnorr_fun_adaptor;
pub use schnorr_fun_adaptor::SchnorrFunAdaptorSignatureScheme;
