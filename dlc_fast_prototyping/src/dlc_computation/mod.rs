// src/dlc_computation/mod.rs

use secp256k1_zkp::{Keypair, PublicKey};

use crate::{
    adaptor_signature_scheme::AdaptorSignatureScheme, common::types, crypto_utils::CryptoUtils,
    dlc_storage::StorageElement,
};

pub trait DlcComputation<ASigS, CU, Out>
where
    ASigS: AdaptorSignatureScheme,
    CU: CryptoUtils,
    Out: types::Outcome,
{
    fn compute_storage_elements_vec(
        parsed_contract: &types::ParsedContract<Out>,
        total_collateral: types::PayoutT,
        signing_keypair: &Keypair,
        oracle_public_key: &PublicKey,
        oracle_public_nonce: &PublicKey,
    ) -> Vec<StorageElement<ASigS>>;

    fn verify_cp_adaptors(
        verification_key: &PublicKey,
        cp_adaptors: &Vec<ASigS::AdaptorSignature>,
        storage_elements_vec: &Vec<StorageElement<ASigS>>,
    ) -> bool;
}

pub mod unified_dlc_computation;
