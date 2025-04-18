// src/config.rs

extern crate static_assertions;

// In future, "relevant adaptors optimization" could be also implemented here as a constant, runtime parameter, or feature flag. The choice is flexible and depends on the design decision.

#[cfg(all(feature = "ecdsa", feature = "schnorr"))]
compile_error!("Features 'ecdsa' and 'schnorr' cannot be enabled at the same time.");

#[cfg(not(any(feature = "ecdsa", feature = "schnorr")))]
compile_error!("Either feature 'ecdsa' or 'schnorr' must be enabled.");

#[cfg(all(feature = "basis-method", feature = "simple-method"))]
compile_error!("Features 'basis-method' and 'simple-method' cannot be enabled at the same time.");

#[cfg(not(any(feature = "basis-method", feature = "simple-method")))]
compile_error!("Either feature 'basis-method' or 'simple-method' must be enabled.");

pub mod constants {
    // Configurable constants
    pub const NB_DIGITS: u8 = 18; // Number of digits representing an outcome
    pub const CONTRACT_INPUT_PATH: &str =
        "./input_contracts/sample_contracts/numerical_contract_input.json";

    // Fixed constants (do not modify)
    pub const NB_OUTCOMES: u32 = 1 << NB_DIGITS; // Total number of possible outcomes
    pub const MAX_OUTCOME: u32 = NB_OUTCOMES - 1; // Maximum possible outcome value
    pub const ZERO_OUTCOME_ATP: u32 = 1000; // Arbitrary value greater than NB_DIGITS, used as a zero outcome anticipation point value

    // compile‑time check NB_DIGITS ≤ 32 (outcomes represented with u32)
    use static_assertions::const_assert;
    const_assert!(NB_DIGITS <= 32);
}

pub mod runparams {
    // Use cargo features to select the appropriate implementation:
    #[cfg(feature = "ecdsa")]
    pub use crate::adaptor_signature_scheme::EcdsaAdaptorSignatureScheme as MyAdaptorSignatureScheme;
    #[cfg(feature = "ecdsa")]
    pub type MySignature = secp256k1_zkp::ecdsa::Signature;

    #[cfg(feature = "schnorr")]
    pub use crate::adaptor_signature_scheme::SchnorrAdaptorSignatureScheme as MyAdaptorSignatureScheme;
    #[cfg(feature = "schnorr")]
    pub type MySignature = secp256k1_zkp::schnorr::Signature;

    // CryptoUtils implementation selection via feature flags
    #[cfg(feature = "basis-method")]
    pub type MyCryptoUtils = crate::crypto_utils::basis_crypto_utils::BasisCryptoUtils;

    #[cfg(feature = "simple-method")]
    pub type MyCryptoUtils = crate::crypto_utils::simple_crypto_utils::SimpleCryptoUtils;

    // To switch between different implementations of CryptoUtils, Oracle, and Parser,
    // modify the type aliases below.
    //
    // If you want to use a custom implementation of CryptoUtils, Oracle, or Parser,
    // implement the respective trait and update the type alias here.
    pub type MyOracle = crate::oracle::RandIntOracle<MyCryptoUtils>; // Oracle implementation
    pub type MyParser = crate::parser::parser_out_u32::SimpleOutU32Parser; // Parser implementation
}

/*
Feature flags to configure the program:
  - "ecdsa" or "schnorr": Specifies the type of adaptor signature scheme to use (one must be enabled).
  - "basis-method" or "simple-method": Specifies the type of crypto utils implementation to use (one must be enabled).

Additional optional features:
  - "parallel-cpt": Enables parallel computation of anticipation points or adaptor signatures (serial if disabled).
  - "parallel-parser": Enables parallel parsing of input intervals (serial if disabled).
  - "enable-benchmarks": Enables benchmark mode for performance evaluation.
*/

// Re-export constants and runtime parameters for easier access by consumers
pub use constants::*;
pub use runparams::*;
