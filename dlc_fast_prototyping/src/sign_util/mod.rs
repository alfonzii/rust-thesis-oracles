pub trait SignUtil {
    type PublicKey;
    type PubNonce;
    type Outcome;
    type AnticipationPoint;

    fn create_atp_point(
        pub_key: &Self::PublicKey,
        pub_nonce: &Self::PubNonce,
        outcome: &Self::Outcome,
    ) -> Self::AnticipationPoint;
}
