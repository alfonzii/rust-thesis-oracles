// src/common/types.rs

use secp256k1_zkp;
use secp256k1_zkp::PublicKey as SecpPublicKey;
use secp256k1_zkp::SecretKey;

/// -- Aliases for conceptually different but physically identical types --
// Public key-like objects
pub type VerificationKey = SecpPublicKey;
pub type AnticipationPoint = SecpPublicKey;
pub type PublicKey = SecpPublicKey;
pub type PublicNonce = SecpPublicKey;

// Private key-like objects
pub type SigningKey = SecretKey;
pub type Attestation = SecretKey;
pub type PrivateKey = SecretKey;
pub type PrivateNonce = SecretKey;

// Other
pub type Cet = String; // Contract Execution Transaction (esentially not signed Tx)
pub type ContractDescriptor<O: Outcome> = Vec<(O, u32)>;

/// The final Bitcoin transaction or any other on-chain transaction type
/// that will be broadcasted after finalization.

pub struct FinalizedTx<Sig> {
    pub payload: Cet,
    pub signature1: Sig,
    pub signature2: Sig,
}

impl<Sig> FinalizedTx<Sig> {
    pub fn new(payload: Cet, signature1: Sig, signature2: Sig) -> Self {
        Self {
            payload,
            signature1,
            signature2,
        }
    }
}

pub struct MultisigFundAddress {
    public_key1: PublicKey,
    public_key2: PublicKey,
}

pub trait Outcome {
    type ValueType;

    /// Return the value of this Outcome.
    fn get_value(&self) -> Self::ValueType;

    /// Return some form of "indicator" at the given position.
    fn get_indicator(&self, position: u8) -> bool;

    /// Serialize this Outcome into bytes to store or transmit.
    fn serialize(&self) -> Vec<u8>; // TODO: will need to remake to [u8; N], because we will be creating lots of small vectors. Instead, we can cap size of String to be equal to size of u32 and then, we know how big to return.
}

// A simple integer-based outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutcomeU32 {
    value: u32,
}

impl Outcome for OutcomeU32 {
    type ValueType = u32;

    fn get_value(&self) -> u32 {
        self.value
    }

    fn get_indicator(&self, position: u8) -> bool {
        debug_assert!(position < 32, "Position must be less than 32");
        (self.value >> position) & 1 == 1
    }

    fn serialize(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }
}

impl From<u32> for OutcomeU32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl From<OutcomeU32> for u32 {
    fn from(outcome: OutcomeU32) -> u32 {
        outcome.value
    }
}

// A string-based binary outcome
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutcomeBinStr {
    value: String,
}

impl Outcome for OutcomeBinStr {
    type ValueType = String;

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn get_indicator(&self, position: u8) -> bool {
        debug_assert!(
            position < self.value.len() as u8,
            "Position must be less than the length of the string"
        );
        self.value.chars().nth(position as usize) == Some('1')
    }

    fn serialize(&self) -> Vec<u8> {
        self.value.as_bytes().to_vec()
    }
}

impl From<String> for OutcomeBinStr {
    fn from(value: String) -> Self {
        // TODO: theoretically, bcs of performance, if we can assume correct string, doesn't have to be here
        if !value.chars().all(|c| c == '0' || c == '1') {
            panic!("OutcomeBinStr can only contain '0' and '1' characters.");
        }
        Self { value }
    }
}

impl From<OutcomeBinStr> for String {
    fn from(outcome: OutcomeBinStr) -> Self {
        outcome.value
    }
}

// TODO: written here but applies to whole project! might think about if renaming isnt needed. we are using whole names like public_key, private_key... maybe using just priv_key and pub_key would be enough and more readable.
