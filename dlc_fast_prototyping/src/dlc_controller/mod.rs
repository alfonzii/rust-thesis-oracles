// src/dlc_controller/mod.rs

pub trait DlcController {
    type ContractConfigInput;
    type Oracle;
    type Attestation;
    type Transaction;
    type AdaptorSignature;

    fn parse_contract_input(
        &self,
        contract: &Self::ContractConfigInput,
    ) -> types::ContractDescriptor<Self::Outcome>;

    //fn init storage (self)

    //fn share_adaptors(self) -> vec adaptor signature

    //fn wait_cp_adaptors(self) -> vec adaptorsignature

    //fn wait_attestation(self) -> OracleAttestation<outcome, attestaion>

    //fn finalize_tx(self, outcome:Outcome, attestation: Atestation) -> Transaction

    // fn broadcast_to_blockchain(self, FinelizedTx)
}

// just some commented skeleton. Going to be changed soon.
