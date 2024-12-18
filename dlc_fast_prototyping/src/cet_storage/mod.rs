pub trait CetStorage {
    type ContractDescriptor;
    type CET;
    type Signature;
    type AdaptorSignature;
    type AnticipationPoint;
    type Outcome;

    fn create(contract_descriptor: &Self::ContractDescriptor); // internally creates a fills new structure that holds CETs
    fn create_adaptors(); // TODO: anticipation points are needed to create adaptors. They might be provided as parameter into this function or we might need to create them here.
                          // Now it is as it is, and we will see in progress, if we leave the function as is now(without paramateers) orr we would do it another way
    fn get_adaptors() -> Vec<Self::AdaptorSignature>;
    fn verify_put_cp_adaptors(counterparty_adaptors: Vec<Self::AdaptorSignature>) -> bool;
    fn get(outcome: &Self::Outcome) -> (Self::CET, Self::Signature);
    //fn finalize (outcome, attestation) -> Self::btc_tx;
}
