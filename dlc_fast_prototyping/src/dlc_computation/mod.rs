// src/dlc_computation/mod.rs

use secp256k1_zkp::{PublicKey, SecretKey};

use crate::{
    adaptor_signature_scheme::AdaptorSignatureScheme, common::types, crypto_utils::CryptoUtils,
    dlc_storage::StorageElement,
};
// TODO: mozno ho bude treba spravit stavove kvoli precomputation atd. tym padom vlastne do compute_..._vec nebude treba davat vector pubnoncov
// v pripade precomputation optimalizacie, ale staci len raz, na zaciatku na vytvorenie precomp_points
pub trait DlcComputation<ASigS, CU, Out>
where
    ASigS: AdaptorSignatureScheme,
    CU: CryptoUtils,
    Out: types::Outcome,
{
    fn compute_storage_elements_vec(
        // TODO: dat sem mozno niekde nb_outcomes, lebo pri pushovani do vec storageelement budeme realokovat
        // a my vlastne tak nejak tusime, aky velky ma byy ten vektor. bud velkosti Buff ktory vracia parser, alebo velkosti "nb_outcomes"
        contract_descriptor: &types::ContractDescriptor<Out>,
        total_collateral: u32,
        signing_key: &SecretKey,
        oracle_public_key: &PublicKey,
        oracle_public_nonce: &PublicKey,
    ) -> Vec<StorageElement<ASigS>>;

    fn verify_cp_adaptors(
        verification_key: &PublicKey,
        cp_adaptors: &Vec<ASigS::AdaptorSignature>,
        storage_elements_vec: &Vec<StorageElement<ASigS>>,
    ) -> bool;
}

pub mod simple_dlc_computation;
