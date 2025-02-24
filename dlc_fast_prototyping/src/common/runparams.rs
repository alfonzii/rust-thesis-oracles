// src/common/runparams.rs

use crate::{
    adaptor_signature_scheme::{EcdsaAdaptorSignatureScheme, SchnorrAdaptorSignatureScheme},
    crypto_utils::simple_crypto_utils::SimpleCryptoUtils,
    oracle::RandIntOracle,
};

#[cfg(feature = "ecdsa")]
pub type MyAdaptorSignatureScheme = EcdsaAdaptorSignatureScheme;
#[cfg(feature = "ecdsa")]
pub type MySignature = secp256k1_zkp::ecdsa::Signature;

#[cfg(feature = "schnorr")]
pub type MyAdaptorSignatureScheme = SchnorrAdaptorSignatureScheme;
#[cfg(feature = "schnorr")]
pub type MySignature = secp256k1_zkp::schnorr::Signature;

// Change following types to test different approaches to DLC
pub type MyCryptoUtils = SimpleCryptoUtils;
pub type MyOracle = RandIntOracle<MyCryptoUtils>;
//type MyDlcController = .... -> spravit nejaky typ podobne ako MyOracle

// adaptor signatures optimization tu moze byt ako nejaky flag napr
