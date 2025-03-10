// src/dlc_computation/mod.rs

use secp256k1_zkp::{Keypair, PublicKey, SecretKey};

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
        parsed_contract: &types::ParsedContract<Out>,
        total_collateral: u32,
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

pub mod parallel_dlc_computation;
pub mod serial_dlc_computation;
