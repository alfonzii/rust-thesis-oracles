use std::io::Error;

use crate::common::Outcome;
use crate::dlc_storage::{DlcStorage, StorageElement};
use crate::{adaptor_signature_scheme::AdaptorSignatureScheme, common::types};

pub struct SimpleArrayStorage<ASigS: AdaptorSignatureScheme> {
    storage: Vec<StorageElement<ASigS>>,
}

impl<ASigS> DlcStorage<ASigS, types::OutcomeU32> for SimpleArrayStorage<ASigS>
where
    ASigS: AdaptorSignatureScheme,
{
    fn new(nb_outcomes: usize) -> Self {
        let storage = vec![StorageElement::<ASigS>::default(); nb_outcomes];
        Self { storage }
    }

    fn put_element(
        &mut self,
        outcome: &types::OutcomeU32,
        element: StorageElement<ASigS>,
    ) -> Result<(), Error> {
        let index = outcome.get_value() as usize;
        if index < self.storage.len() {
            self.storage[index] = element;
            Ok(())
        } else {
            Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Outcome index out of bounds",
            ))
        }
    }

    fn get_element(&self, outcome: &types::OutcomeU32) -> Option<StorageElement<ASigS>> {
        let index = outcome.get_value() as usize;
        if index < self.storage.len() {
            Some(self.storage[index].clone())
        } else {
            None
        }
    }

    fn get_all_my_adaptors(&self) -> Vec<ASigS::AdaptorSignature> {
        self.storage
            .iter()
            .filter_map(|element| element.my_adaptor_signature.clone())
            .collect()
    }

    fn get_all_elements_vec_ref(&self) -> &Vec<StorageElement<ASigS>> {
        &self.storage
    }

    fn update_cp_adaptors(
        &mut self,
        cp_adaptors: Vec<ASigS::AdaptorSignature>,
    ) -> Result<(), Error> {
        if cp_adaptors.len() != self.storage.len() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid number of cp adaptors",
            ));
        }

        for (element, cp_adaptor) in self.storage.iter_mut().zip(cp_adaptors) {
            element.cp_adaptor_signature = Some(cp_adaptor);
        }

        Ok(())
    }
}
