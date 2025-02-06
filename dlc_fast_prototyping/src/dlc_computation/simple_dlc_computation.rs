// src/dlc_computation/simple_dlc_computation.rs

use secp256k1_zkp::Message;
use sha2::{Digest, Sha256};

use crate::{
    adaptor_signature_scheme::AdaptorSignatureScheme,
    common::{types, Outcome, OutcomeU32},
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
    fn create_cet(payout: u32, total_collateral: u32) -> String {
        format!(
            "Alice gets {} sats and Bob gets {} sats from DLC",
            total_collateral - payout,
            payout
        )
    }

    fn create_message(cet_str: &String) -> Message {
        let hash = Sha256::digest(cet_str.as_bytes());
        let hashed_message: [u8; 32] = hash.into();
        Message::from_digest_slice(&hashed_message).unwrap()
    }

    fn compute_anticipation_point(
        public_key: &types::PublicKey,
        public_nonce: &types::PublicNonce,
        outcome: &impl Outcome,
    ) -> types::AnticipationPoint {
        CU::compute_anticipation_point(public_key, public_nonce, outcome).unwrap()
    }

    fn compute_adaptor_signature(
        signing_key: &types::SigningKey,
        cet_str: &String,
        anticipation_point: &types::AnticipationPoint,
    ) -> ASigS::AdaptorSignature {
        let message = Self::create_message(cet_str);
        ASigS::pre_sign(signing_key, &message, anticipation_point)
    }

    fn create_storage_element(
        cet: types::Cet,
        anticipation_point: types::AnticipationPoint,
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
        contr_desc: &types::ContractDescriptor<OutcomeU32>,
        total_collateral: u32,
        sign_key: &types::SigningKey,
        oracle_pub_key: &types::PublicKey,
        oracle_pub_nonce: &types::PublicNonce,
    ) -> Vec<StorageElement<ASigS>> {
        let mut storage_elements_vec = Vec::with_capacity(contr_desc.len());

        for (outcome, payout) in contr_desc.iter() {
            let cet_str = Self::create_cet(*payout, total_collateral);
            let anticipation_point =
                Self::compute_anticipation_point(oracle_pub_key, oracle_pub_nonce, outcome);
            let my_adaptor_signature =
                Self::compute_adaptor_signature(sign_key, &cet_str, &anticipation_point);
            let storage_element =
                Self::create_storage_element(cet_str, anticipation_point, my_adaptor_signature);
            storage_elements_vec.push(storage_element);
        }

        storage_elements_vec
    }

    fn verify_cp_adaptors(
        verif_key: &types::PublicKey,
        cp_adaptors: &Vec<ASigS::AdaptorSignature>,
        storage_elements_vec: &Vec<StorageElement<ASigS>>,
    ) -> bool {
        assert_eq!(
            cp_adaptors.len(),
            storage_elements_vec.len(),
            "cp_adaptors and storage_elements_vec must have the same length"
        );

        for (cp_adaptor, storage_element) in cp_adaptors.iter().zip(storage_elements_vec.iter()) {
            let message = Self::create_message(&storage_element.cet);

            let is_verified = ASigS::pre_verify(
                verif_key,
                &message,
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
