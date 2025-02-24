use std::{io::Error, str::FromStr};

use secp256k1_zkp::{PublicKey, SecretKey, SECP256K1};

use crate::{adaptor_signature_scheme::AdaptorSignatureScheme, common::types};

pub trait DlcStorage<ASigS, Out>
where
    ASigS: AdaptorSignatureScheme,
    Out: types::Outcome,
{
    fn new(nb_outcomes: usize) -> Self
    where
        Self: Sized;

    fn put_element(&mut self, outcome: &Out, element: StorageElement<ASigS>) -> Result<(), Error>;

    fn get_element(&self, outcome: &Out) -> Option<StorageElement<ASigS>>;
    fn get_all_my_adaptors(&self) -> Vec<ASigS::AdaptorSignature>;

    fn get_all_elements_vec_ref(&self) -> &Vec<StorageElement<ASigS>>;

    fn update_cp_adaptors(
        &mut self,
        cp_adaptors: Vec<ASigS::AdaptorSignature>,
    ) -> Result<(), Error>;
}

pub struct StorageElement<ASigS: AdaptorSignatureScheme> {
    pub cet: types::Cet,
    pub anticipation_point: PublicKey,
    pub my_adaptor_signature: Option<ASigS::AdaptorSignature>,
    pub cp_adaptor_signature: Option<ASigS::AdaptorSignature>,
}

impl<ASigS: AdaptorSignatureScheme> Default for StorageElement<ASigS> {
    fn default() -> Self {
        let default_pub_key =
            SecretKey::from_str("0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap()
                .public_key(SECP256K1);
        Self {
            cet: String::new(),
            anticipation_point: default_pub_key,
            my_adaptor_signature: None,
            cp_adaptor_signature: None,
        }
    }
}

impl<ASigS> Clone for StorageElement<ASigS>
where
    ASigS: AdaptorSignatureScheme,
    ASigS::AdaptorSignature: Clone,
    types::Cet: Clone,
    types::AnticipationPoint: Clone,
{
    fn clone(&self) -> Self {
        StorageElement {
            cet: self.cet.clone(),
            anticipation_point: self.anticipation_point.clone(),
            my_adaptor_signature: self.my_adaptor_signature.clone(),
            cp_adaptor_signature: self.cp_adaptor_signature.clone(),
        }
    }
}

pub mod simple_array_storage;
