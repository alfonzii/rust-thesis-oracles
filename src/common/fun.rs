// src/common/fun.rs

use secp256k1_zkp::Message;
use sha2::{Digest, Sha256};

use super::types;

// Based on payout and total collateral (from contract descriptor), create a string that represents the CET
// INFO: From https://github.com/discreetlogcontracts/dlcspecs/blob/master/PayoutCurve.md, 'payout' represents satoshis for offerer
// and accepter satoshis are 'total_collateral - payout'
pub fn create_cet(payout: types::PayoutT, total_collateral: types::PayoutT) -> String {
    format!(
        "Alice gets {} sats and Bob gets {} sats from DLC",
        payout,
        total_collateral - payout,
    )
}

// Create a message from the CET (might be of any type representable as bytes)
pub fn create_message<T: AsRef<[u8]>>(cet: T) -> Result<Message, secp256k1_zkp::UpstreamError> {
    let hash = Sha256::digest(cet.as_ref());
    let hashed_msg: [u8; 32] = hash.into();
    Message::from_digest_slice(&hashed_msg)
}
