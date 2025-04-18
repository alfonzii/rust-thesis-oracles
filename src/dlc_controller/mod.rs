// src/dlc_controller/mod.rs

use secp256k1_zkp::PublicKey;

use crate::{
    adaptor_signature_scheme::AdaptorSignatureScheme, common::types, config::MySignature,
    crypto_utils::CryptoUtils, oracle::Oracle,
};
use std::{io::Error, sync::Arc};

/// Role of a DLC participant: Offerer or Accepter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerType {
    Offerer,
    Accepter,
}

/// Discreet Log Contract controller interface.
/// Parameterized by adaptor‐signature scheme `ASigS`, crypto engine `CU`, and oracle `O`.
/// Implements the core protocol steps: initialization, input loading, storage setup,
/// key/adaptor exchange, verification, attestation handling, and final transaction finalization.
pub trait DlcController<ASigS, CU, O>
where
    ASigS: AdaptorSignatureScheme,
    CU: CryptoUtils,
    O: Oracle,
{
    /// Creates a new controller with a given name and oracle.
    fn new(ctype: ControllerType, oracle: Arc<O>) -> Self;

    /// Loads DLC input from a file.
    fn load_input(&mut self, input_path: &str) -> Result<(), Error>;

    /// Initializes all necessary storage structures before use.
    fn init_storage(&mut self) -> Result<(), Error>;

    /// Returns this controller's verification key.
    fn share_verification_key(&self) -> PublicKey;

    /// Returns a list of adaptors for the current DLC.
    fn share_adaptors(&self) -> Vec<ASigS::AdaptorSignature>;

    /// Saves the counterparty's verification key.
    fn save_cp_verification_key(&mut self, cp_verification_key: PublicKey);

    /// Saves the counterparty's adaptors.
    fn save_cp_adaptors(&mut self, cp_adaptors: Vec<ASigS::AdaptorSignature>);

    /// Verifies the counterparty's adaptors.
    fn verify_cp_adaptors(&self) -> bool;

    /// Updates the stored adaptors with verified counterparty information.
    fn update_cp_adaptors(&mut self) -> Result<(), Error>;

    /// Waits for oracle attestation to proceed with finalizing the DLC.
    fn wait_attestation(&mut self) -> Result<(), Error>;

    /// Finalizes the transaction using the relevant signatures.
    fn finalize_tx(&self) -> types::FinalizedTx<MySignature>;
}

pub mod very_simple_controller;
