use std::io::Error;

use crate::{adaptor_signature_scheme::AdaptorSignatureScheme, common::types};

pub trait DlcStorage<ASigS, Outcome>
where
    ASigS: AdaptorSignatureScheme,
    Outcome: types::Outcome,
{
    fn new(nb_outcomes: usize) -> Self
    where
        Self: Sized;

    fn put_element(
        &mut self,
        outcome: &Outcome,
        element: StorageElement<ASigS>,
    ) -> Result<(), Error>;

    fn get_element(&self, outcome: &Outcome) -> Option<StorageElement<ASigS>>;
    fn get_all_my_adaptors(&self) -> Vec<ASigS::AdaptorSignature>;

    fn update_cp_adaptors(
        &mut self,
        cp_adaptors: Vec<ASigS::AdaptorSignature>,
    ) -> Result<(), Error>;
}

pub struct StorageElement<ASigS: AdaptorSignatureScheme> {
    pub cet: types::Cet,
    pub anticipation_point: types::AnticipationPoint,
    pub my_adaptor_signature: Option<ASigS::AdaptorSignature>,
    pub counterparty_adaptor_signature: Option<ASigS::AdaptorSignature>,
}

pub mod simple_array_storage;
