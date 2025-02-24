// src/dlc_computation/simple_dlc_computation.rs

use secp256k1_zkp::{PublicKey, SecretKey};

use crate::{
    adaptor_signature_scheme::AdaptorSignatureScheme,
    common::{self, types, OutcomeU32},
    crypto_utils::CryptoUtils,
    dlc_computation::DlcComputation,
    dlc_storage::StorageElement,
};
use std::marker::PhantomData;

pub struct SimpleDlcComputation<ASigS: AdaptorSignatureScheme, CU: CryptoUtils> {
    _phantom1: PhantomData<ASigS>,
    _phantom2: PhantomData<CU>,
}

impl<ASigS: AdaptorSignatureScheme, CU: CryptoUtils> SimpleDlcComputation<ASigS, CU> {
    fn create_storage_element(
        cet: types::Cet,
        anticipation_point: PublicKey,
        my_adaptor_signature: ASigS::AdaptorSignature,
    ) -> StorageElement<ASigS> {
        StorageElement {
            cet,
            anticipation_point,
            my_adaptor_signature: Some(my_adaptor_signature),
            cp_adaptor_signature: None,
        }
    }
}

impl<ASigS, CU> DlcComputation<ASigS, CU, types::OutcomeU32> for SimpleDlcComputation<ASigS, CU>
where
    ASigS: AdaptorSignatureScheme,
    CU: CryptoUtils,
{
    fn compute_storage_elements_vec(
        contract_descriptor: &types::ContractDescriptor<OutcomeU32>,
        total_collateral: u32,
        signing_key: &SecretKey,
        oracle_public_key: &PublicKey,
        oracle_public_nonce: &PublicKey,
    ) -> Vec<StorageElement<ASigS>> {
        let mut storage_elements_vec = Vec::with_capacity(contract_descriptor.len());

        for (outcome, payout) in contract_descriptor.iter() {
            // Create CET from the contract descriptor element
            let cet_str = common::fun::create_cet(*payout, total_collateral); // TODO: toto by malo asi byt nejak inak, nie ze create_cet a vytvorim string. cely tento create cet koncept by mal byt spraveny obecne. je jedno ci je CET string alebo btc tx. malo by to vsetko fungovat

            // Create message from CET which will be used later for all math operations
            let msg = common::fun::create_message(&cet_str).unwrap();

            // Compute anticipation point
            let atp_point =
                CU::compute_anticipation_point(oracle_public_key, oracle_public_nonce, outcome)
                    .unwrap();

            // Compute my adaptor signature
            let my_adaptor = ASigS::pre_sign(signing_key, &msg, &atp_point);

            // Create storage element
            let storage_element = Self::create_storage_element(cet_str, atp_point, my_adaptor);
            storage_elements_vec.push(storage_element);
        }

        storage_elements_vec
    }

    fn verify_cp_adaptors(
        verification_key: &PublicKey,
        cp_adaptors: &Vec<ASigS::AdaptorSignature>,
        storage_elements_vec: &Vec<StorageElement<ASigS>>,
    ) -> bool {
        assert_eq!(
            cp_adaptors.len(),
            storage_elements_vec.len(),
            "cp_adaptors and storage_elements_vec must have the same length"
        );

        for (cp_adaptor, storage_element) in cp_adaptors.iter().zip(storage_elements_vec.iter()) {
            let msg = common::fun::create_message(&storage_element.cet).unwrap();
            let is_verified = ASigS::pre_verify(
                verification_key,
                &msg,
                &storage_element.anticipation_point,
                cp_adaptor,
            );
            if !is_verified {
                return false;
            }
        }
        true
    }
}
