// src/dlc_computation/mod.rs

use crate::{
    adaptor_signature_scheme::AdaptorSignatureScheme,
    common::{types, Outcome},
    crypto_utils::CryptoUtils,
    dlc_storage::StorageElement,
};

pub trait DlcComputation<ASigS, CU, O>
where
    ASigS: AdaptorSignatureScheme,
    CU: CryptoUtils,
    O: Outcome,
{
    fn compute_storage_elements_vec(
        // TODO: dat sem mozno niekde nb_outcomes, lebo pri pushovani do vec storageelement budeme realokovat a my vlastne tak nejak tusime, aky velky ma byy ten vektor. bud velkosti Buff ktory vracia parser, alebo velkosti "nb_outcomes"
        contr_desc: &types::ContractDescriptor<O>,
        sign_key: &types::SigningKey,
        oracle_pub_key: &types::PublicKey,
        oracle_pub_nonces_vec: &Vec<types::PublicNonce>,
    ) -> Vec<StorageElement<ASigS>>;

    fn verify_cp_adaptors(
        verif_key: &types::PublicKey,
        cp_adaptors: &Vec<ASigS::AdaptorSignature>,
        storage_elements_vec: &Vec<StorageElement<ASigS>>,
    ) -> bool;
}

//fn get_unified_outcome(outcome: types::Outcome) -> Storage::UniOutcome;
// TODO: tieto unified outcome gettery budu potom pravdepodobne v Parseri pretoze ten to uz sam nejak unifikuje/aggreguje, takze dava zmysel, zeby si to rovno niekde ulozil a nasledne sa ho mozeme dotazovat
