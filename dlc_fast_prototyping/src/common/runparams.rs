// src/common/runparams.rs

use crate::crypto_utils::basis_crypto_utils::BasisCryptoUtils;
use crate::crypto_utils::simple_crypto_utils::SimpleCryptoUtils;
use crate::parser::parser_mock::MockU32Parser;
// use crate::parser::parser_out_u32::SimpleOutU32Parser;
use crate::{
    adaptor_signature_scheme::{EcdsaAdaptorSignatureScheme, SchnorrAdaptorSignatureScheme},
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
pub type MyCryptoUtils = BasisCryptoUtils;
pub type MyOracle = RandIntOracle<MyCryptoUtils>;
pub type MyParser = MockU32Parser;

//type MyDlcController = .... -> spravit nejaky typ podobne ako MyOracle

// adaptor signatures optimization tu moze byt ako nejaky flag napr
