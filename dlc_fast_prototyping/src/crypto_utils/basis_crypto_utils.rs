use std::str::FromStr;

use secp256k1_zkp::{Error, PublicKey, Scalar, SecretKey};

use crate::common::constants::{NB_DIGITS, ZERO_OUTCOME_ATP};
use crate::common::types;
use crate::crypto_utils::CryptoUtils;

use crate::common::types::OutcomeU32;
use crate::crypto_utils::secp_utils::{
    schnorrsig_compute_anticipation_point, schnorrsig_compute_oracle_attestation,
};

use secp256k1_zkp::SECP256K1;

pub struct BasisCryptoUtils {
    public_key: PublicKey,
    public_nonce: PublicKey,
    precomputed_points: Vec<PublicKey>,
}

impl BasisCryptoUtils {
    fn precompute_points(&mut self) {
        for i in 0..NB_DIGITS {
            let bit_position = OutcomeU32::from(i as u32); // We are using outcome type here, bcs schnorrsig_compute.. expects it.
            let basis_atp_point = schnorrsig_compute_anticipation_point(
                SECP256K1,
                &self.public_key,
                &self.public_nonce,
                &bit_position,
            )
            .expect("Failed to compute basis anticipation point");
            self.precomputed_points[i as usize] = basis_atp_point;
        }
    }
}

impl CryptoUtils for BasisCryptoUtils {
    fn new(public_key: &PublicKey, public_nonce: &PublicKey) -> Self {
        let mut instance = Self {
            public_key: public_key.clone(),
            public_nonce: public_nonce.clone(),
            precomputed_points: vec![
                SecretKey::from_str(
                    "0000000000000000000000000000000000000000000000000000000000000001"
                )
                .unwrap()
                .public_key(SECP256K1);
                NB_DIGITS as usize
            ],
        };
        instance.precompute_points();
        instance
    }

    // TODO: will throw error if no bit is set in outcome (we catch zero outcome, so there must be some other problem like bigger number than 2^NB_DIGITS and it's first NB_DIGITS bits are not set)
    fn compute_anticipation_point(
        &self,
        outcome: &impl types::Outcome,
    ) -> Result<types::AnticipationPoint, Error> {
        use secp256k1_zkp::PublicKey;

        // If the outcome is zero, use exceptional anticipation point.
        if outcome.is_zero() {
            return schnorrsig_compute_anticipation_point(
                SECP256K1,
                &self.public_key,
                &self.public_nonce,
                &OutcomeU32::from(ZERO_OUTCOME_ATP),
            );
        }

        // Else if outcome is not zero: Select basis atp_points and combine them.
        let mut selected_basis_atps = Vec::new();
        for i in 0..NB_DIGITS {
            if outcome.get_bit(i as u8) {
                selected_basis_atps.push(&self.precomputed_points[i as usize]);
            }
        }
        let combined = PublicKey::combine_keys(&selected_basis_atps)?;
        Ok(combined)
    }

    // TODO: same as atp_point, will throw error if no bit is set in outcome
    fn compute_attestation(
        &self,
        private_key: &SecretKey,
        private_nonce: &SecretKey,
        outcome: &impl types::Outcome,
    ) -> Result<types::Attestation, Error> {
        // If the outcome is zero, use exceptional anticipation point.
        if outcome.is_zero() {
            let attestation_zero = schnorrsig_compute_oracle_attestation(
                SECP256K1,
                private_key,
                private_nonce,
                &OutcomeU32::from(ZERO_OUTCOME_ATP),
            );
            return attestation_zero;
        }

        // Else if outcome is not zero: Find first non-zero outcome bit
        let first_index = (0..NB_DIGITS)
            .find(|&i| outcome.get_bit(i as u8))
            .ok_or(Error::InvalidGenerator)?;

        // Use the partial attestation for the first set bit as the initial value.
        let mut combined = schnorrsig_compute_oracle_attestation(
            SECP256K1,
            private_key,
            private_nonce,
            &OutcomeU32::from(first_index as u32),
        )?;

        // For every subsequent set bit, compute partial attestation and add (tweak) it.
        for i in (first_index + 1)..NB_DIGITS {
            if outcome.get_bit(i as u8) {
                let partial = schnorrsig_compute_oracle_attestation(
                    SECP256K1,
                    private_key,
                    private_nonce,
                    &OutcomeU32::from(i as u32),
                )?;
                let scalar = Scalar::from(partial);
                combined = combined.add_tweak(&scalar)?;
            }
        }
        Ok(combined)
    }
}

// Add this at the bottom of your basis_crypto_utils.rs file

#[cfg(test)]
mod tests {
    use secp256k1_zkp::Secp256k1;

    use super::*;

    // Helper function to create a dummy instance of BasisCryptoUtils using a fixed secret key.
    fn create_dummy_utils() -> BasisCryptoUtils {
        // Create a fixed secret key (32 bytes all set to 1).
        let sk = SecretKey::from_slice(&[1u8; 32]).expect("32 bytes, within curve order");
        let pk = PublicKey::from_secret_key(SECP256K1, &sk);
        BasisCryptoUtils::new(&pk, &pk)
    }

    #[test]
    fn test_precompute_points() {
        let utils = create_dummy_utils();
        // Check that we have precomputed exactly NB_DIGITS points.
        assert_eq!(utils.precomputed_points.len(), NB_DIGITS as usize);
        // Optionally, check that none of the points is the identity (or zero) value by serializing.
        for &point in &utils.precomputed_points {
            assert!(point.serialize()[..].iter().any(|&b| b != 0));
        }
    }

    #[test]
    fn test_compute_anticipation_point() {
        let utils = create_dummy_utils();
        // Create an outcome with some bits set, e.g. outcome 3.
        let outcome = OutcomeU32::from(3);
        let res: Result<types::AnticipationPoint, Error> =
            utils.compute_anticipation_point(&outcome);
        assert!(
            res.is_ok(),
            "Precomputed anticipation point should be computed"
        );
        let ant_point = res.unwrap();
        // Verify that the returned public key is valid by attempting to serialize it.
        let serialized = ant_point.serialize();
        // Basic check: serialized public key should have non-zero length.
        assert!(!serialized.is_empty());
    }

    #[test]
    fn test_compute_attestation() {
        let secp = Secp256k1::new();
        // Use different fixed secret keys for signing and nonce.
        let sk = SecretKey::from_slice(&[2u8; 32]).expect("32 bytes, within curve order");
        let nonce = SecretKey::from_slice(&[3u8; 32]).expect("32 bytes, within curve order");
        let pk = PublicKey::from_secret_key(&secp, &sk);
        let utils = BasisCryptoUtils::new(&pk, &pk);

        let outcome = OutcomeU32::from(5);
        let attestation_res = utils.compute_attestation(&sk, &nonce, &outcome);
        assert!(
            attestation_res.is_ok(),
            "Attestation computation should succeed"
        );
        let _attestation = attestation_res.unwrap();
    }
}
