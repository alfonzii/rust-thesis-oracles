// src/dlc_controller/mod.rs

use secp256k1_zkp::PublicKey;

use crate::{adaptor_signature_scheme::AdaptorSignatureScheme, common::types, oracle::Oracle};
use std::{io::Error, sync::Arc};

pub trait DlcController<ASigS: AdaptorSignatureScheme, O: Oracle> {
    fn new(name: &str, oracle: Arc<O>) -> Self;

    fn load_input(&self, input_path: &str) -> Result<(), Error>;

    fn init_storage(&mut self) -> Result<(), Error>;

    // TODO: Share and save should look differently, but for now, we will make them as they are just for the sake of simplicity
    fn share_verification_key(&self) -> PublicKey;
    fn share_adaptors(&self) -> Vec<ASigS::AdaptorSignature>;
    fn save_cp_verification_key(&mut self, cp_verification_key: PublicKey) -> ();
    fn save_cp_adaptors(&mut self, cp_adaptors: Vec<ASigS::AdaptorSignature>) -> ();

    fn verify_cp_adaptors(&self) -> bool;

    fn update_cp_adaptors(&mut self) -> Result<(), Error>;

    fn wait_attestation(&mut self) -> bool; // Returns if outcome of event is positive for my perspective of DLC

    fn finalize_tx(&self) -> types::FinalizedTx<ASigS::Signature>;

    // fn broadcast_to_blockchain(self) -> Result<(), Error>;
}

pub mod very_simple_controller;
