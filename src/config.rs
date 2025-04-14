// src/config.rs
#[cfg(all(feature = "ecdsa", feature = "schnorr"))]
compile_error!("Features 'ecdsa' and 'schnorr' cannot be enabled at the same time.");

#[cfg(not(any(feature = "ecdsa", feature = "schnorr")))]
compile_error!("Either feature 'ecdsa' or 'schnorr' must be enabled.");

pub mod constants {
    // Changeable
    pub const NB_DIGITS: u8 = 14; // number of maximum digits that represent outcome
    pub const CONTRACT_INPUT_PATH: &str =
        "./input_contracts/sample_contracts/reduced_contract_input.json";

    // NOT TO CHANGE!
    pub const NB_OUTCOMES: u32 = 1 << NB_DIGITS;
    pub const MAX_OUTCOME: u32 = NB_OUTCOMES - 1;
    pub const ZERO_OUTCOME_ATP: u32 = 1000; // random value bigger than NB_DIGITS representing zero outcome anticipation point value calc

    const _: () = {
        // we disallow more than 32 digits, because we represent outcome with u32.
        if NB_DIGITS > 32 {
            panic!("NB_DIGITS must be less than or equal to 32");
        }
    };
}

// adaptor signatures optimization tu moze byt ako nejaky flag napr, bud ako constant, runparam alebo ako feature. to je jedno, up to decision

pub mod runparams {
    // Use cargo features to select implementations:
    #[cfg(feature = "ecdsa")]
    pub use crate::adaptor_signature_scheme::EcdsaAdaptorSignatureScheme as MyAdaptorSignatureScheme;
    #[cfg(feature = "ecdsa")]
    pub type MySignature = secp256k1_zkp::ecdsa::Signature;

    #[cfg(feature = "schnorr")]
    pub use crate::adaptor_signature_scheme::SchnorrAdaptorSignatureScheme as MyAdaptorSignatureScheme;
    #[cfg(feature = "schnorr")]
    pub type MySignature = secp256k1_zkp::schnorr::Signature;

    // To use different implementations of CryptoUtils, Oracle, and Parser,
    // just change the type aliases below.
    //
    // If you ever want to use your own implementation of CryptoUtils, Oracle, or Parser,
    // just implement the respective trait and change the type alias here.
    pub type MyCryptoUtils = crate::crypto_utils::basis_crypto_utils::BasisCryptoUtils; // method for computing anticipation points (now either simple or basis)
    pub type MyOracle = crate::oracle::RandIntOracle<MyCryptoUtils>; // oracle implementation
    pub type MyParser = crate::parser::parser_out_u32::SimpleOutU32Parser; // parser implementation
}

/*
Feature params to run the program
  "ecdsa" or "schnorr" - type of adaptor signatures scheme to be used (one of them must be used)

Further features, might be either on or off
  "parallel-cpt" - parallel computation of anticipation points / adaptor signatures (if off, then serial)
  "parallel-parser" - read and parse input intervals in parallel or serial if off
  "enable-benchmarks" - enable program to run in benchmark mode
*/

// Re-export so that consumers only need to import from config
pub use constants::*;
pub use runparams::*;
