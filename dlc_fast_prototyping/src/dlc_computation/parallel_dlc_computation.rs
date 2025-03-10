// src/dlc_computation/parallel_dlc_computation.rs

use rayon::prelude::*;
use std::marker::PhantomData; // for parallel iterators

use secp256k1_zkp::{Keypair, PublicKey, SecretKey};

use crate::{
    adaptor_signature_scheme::AdaptorSignatureScheme,
    common::{self, types},
    crypto_utils::CryptoUtils,
    dlc_computation::DlcComputation,
    dlc_storage::StorageElement,
};

pub struct ParallelDlcComputation<ASigS: AdaptorSignatureScheme, CU: CryptoUtils> {
    _phantom1: PhantomData<ASigS>,
    _phantom2: PhantomData<CU>,
}

impl<ASigS: AdaptorSignatureScheme, CU: CryptoUtils> ParallelDlcComputation<ASigS, CU> {
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

impl<ASigS, CU> DlcComputation<ASigS, CU, types::OutcomeU32> for ParallelDlcComputation<ASigS, CU>
where
    ASigS: AdaptorSignatureScheme,
    CU: CryptoUtils + Sync,
    ASigS::AdaptorSignature: Send + Sync,
{
    fn compute_storage_elements_vec(
        parsed_contract: &types::ParsedContract<types::OutcomeU32>,
        total_collateral: u32,
        signing_keypair: &Keypair,
        oracle_public_key: &PublicKey,
        oracle_public_nonce: &PublicKey,
    ) -> Vec<StorageElement<ASigS>> {
        // Setup as before
        let crypto_utils_engine = CU::new(oracle_public_key, oracle_public_nonce);

        // Use parallel iteration:
        parsed_contract
            .par_iter() // creates a parallel iterator
            .map(|(outcome, payout)| {
                // 1. Create CET (string or whatever you do)
                let cet_str = common::fun::create_cet(*payout, total_collateral);

                // 2. Create message
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

    fn verify_cp_adaptors(
        verification_key: &PublicKey,
        cp_adaptors: &Vec<ASigS::AdaptorSignature>,
        storage_elements_vec: &Vec<StorageElement<ASigS>>,
    ) -> bool {
        // 1. Check lengths
        assert_eq!(
            cp_adaptors.len(),
            storage_elements_vec.len(),
            "cp_adaptors and storage_elements_vec must have the same length"
        );

        // 2. Parallel iteration
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
}
