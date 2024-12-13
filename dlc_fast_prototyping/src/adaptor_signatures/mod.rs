pub trait AdaptorSignatureScheme {
    type SigningKey;
    type VerificationKey;
    type AnticipationPoint;
    type Attestation;
    type PreSignature;
    type Signature;
    type Message;

    fn pre_sign(
        signing_key: &Self::SigningKey,
        message: &Self::Message,
        atp_point: &Self::AnticipationPoint,
    ) -> Self::PreSignature;

    fn pre_verify(
        verification_key: &Self::VerificationKey,
        message: &Self::Message,
        atp_point: &Self::AnticipationPoint,
        pre_signature: &Self::PreSignature,
    ) -> bool;

    fn adapt(
        pre_signature: &Self::PreSignature,
        attestation: &Self::Attestation,
    ) -> Self::Signature;

    fn extract(
        signature: &Self::Signature,
        pre_signature: &Self::PreSignature,
        atp_point: &Self::AnticipationPoint,
    ) -> Self::Attestation;
}
