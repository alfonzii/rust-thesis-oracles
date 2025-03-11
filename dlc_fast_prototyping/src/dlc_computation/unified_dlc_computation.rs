// src/dlc_computation/unified_dlc_computation.rs

use crate::{
    adaptor_signature_scheme::AdaptorSignatureScheme,
    common::{self, types, OutcomeU32},
    crypto_utils::CryptoUtils,
    dlc_computation::DlcComputation,
    dlc_storage::StorageElement,
};
use secp256k1_zkp::{Keypair, Message, PublicKey, SecretKey};
use std::marker::PhantomData;

#[cfg(feature = "parallel-cpt")]
use rayon::prelude::*;

pub struct UnifiedDlcComputation<ASigS: AdaptorSignatureScheme, CU: CryptoUtils> {
    _phantom1: PhantomData<ASigS>,
    _phantom2: PhantomData<CU>,
}

impl<ASigS: AdaptorSignatureScheme, CU: CryptoUtils> UnifiedDlcComputation<ASigS, CU> {
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

impl<ASigS, CU> DlcComputation<ASigS, CU, types::OutcomeU32> for UnifiedDlcComputation<ASigS, CU>
where
    ASigS: AdaptorSignatureScheme,
    CU: CryptoUtils + Sync,
    ASigS::AdaptorSignature: Send + Sync,
{
    fn compute_storage_elements_vec(
        parsed_contract: &types::ParsedContract<types::OutcomeU32>,
        total_collateral: u64,
        signing_keypair: &Keypair,
        oracle_public_key: &PublicKey,
        oracle_public_nonce: &PublicKey,
    ) -> Vec<StorageElement<ASigS>> {
        let crypto_utils_engine = CU::new(oracle_public_key, oracle_public_nonce);

        #[cfg(feature = "parallel-cpt")]
        {
            // Use parallel iteration:
            parsed_contract
                .par_iter()
                .map(|(outcome, payout)| {
                    // 1. Create CET (string, BTC tx, ...)
                    let cet_str = common::fun::create_cet(*payout, total_collateral);
                    // 2. Create message from CET which will be used later for all math operations
                    let msg = common::fun::create_message(&cet_str).unwrap();
                    // 3. Compute anticipation point in parallel
                    let atp_point = crypto_utils_engine
                        .compute_anticipation_point(outcome)
                        .unwrap();
                    // 4. Pre-sign
                    let my_adaptor = ASigS::pre_sign(signing_keypair, &msg, &atp_point);
                    // 5. Create storage element
                    Self::create_storage_element(cet_str, atp_point, my_adaptor)
                })
                .collect() // gather results into a Vec
        }
        #[cfg(not(feature = "parallel-cpt"))]
        {
            parsed_contract
                .iter()
                .map(|(outcome, payout)| {
                    let cet_str = common::fun::create_cet(*payout, total_collateral);
                    let msg = common::fun::create_message(&cet_str).unwrap();
                    let atp_point = crypto_utils_engine
                        .compute_anticipation_point(outcome)
                        .unwrap();
                    let my_adaptor = ASigS::pre_sign(signing_keypair, &msg, &atp_point);
                    Self::create_storage_element(cet_str, atp_point, my_adaptor)
                })
                .collect()
        }
    }

    fn verify_cp_adaptors(
        verification_key: &PublicKey,
        cp_adaptors: &Vec<ASigS::AdaptorSignature>,
        storage_elements_vec: &Vec<StorageElement<ASigS>>,
    ) -> bool {
        // Check lengths (TODO: ak pouzijem asig relevant optimization tak toto asi nebude platit)
        assert_eq!(
            cp_adaptors.len(),
            storage_elements_vec.len(),
            "cp_adaptors and storage_elements_vec must have the same length"
        );

        #[cfg(feature = "parallel-cpt")]
        {
            // Parallel iteration
            cp_adaptors
                .par_iter() // parallel iterator over &Vec<ASigS::AdaptorSignature>
                .zip(storage_elements_vec.par_iter()) // zip with parallel iterator over &Vec<StorageElement<ASigS>>
                .all(|(cp_adaptor, storage_element)| {
                    // For each pair, do the logic in parallel:
                    let msg = common::fun::create_message(&storage_element.cet).unwrap();
                    ASigS::pre_verify(
                        verification_key,
                        &msg,
                        &storage_element.anticipation_point,
                        cp_adaptor,
                    )
                })
        }
        #[cfg(not(feature = "parallel-cpt"))]
        {
            for (cp_adaptor, storage_element) in cp_adaptors.iter().zip(storage_elements_vec.iter())
            {
                let msg = common::fun::create_message(&storage_element.cet).unwrap();
                if !ASigS::pre_verify(
                    verification_key,
                    &msg,
                    &storage_element.anticipation_point,
                    cp_adaptor,
                ) {
                    return false;
                }
            }
            true
        }
    }
}
