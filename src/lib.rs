//! This crate provides structs to serialize and deserialize DNPM:DIP MTB DTOs.
//! The base struct is `Mtb`.

#![allow(clippy::needless_doctest_main)]

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub use crate::mtb::*;

mod mtb;

#[derive(Debug)]
pub struct SerdeError(String);

impl Display for SerdeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MtbFile Serde Error: {}", self.0)
    }
}

impl Error for SerdeError {}

impl FromStr for Mtb {
    type Err = SerdeError;

    /// Deserialize an instance of `MtbFile` from a string of JSON text.
    ///
    /// # Example
    ///
    /// ```
    /// use mv64e_mtb_dto::Mtb;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     const MTB_JSON: &str = include_str!("../tests/mv64e-mtb-fake-patient.json");
    ///
    ///     let mtb_file = Mtb::from_str(MTB_JSON).unwrap();
    ///     println!("{:#?}", mtb_file);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// If the conversion fails, an `SerdeError` will be returned.
    fn from_str(value: &str) -> Result<Mtb, SerdeError> {
        serde_json::from_str(value).map_err(|err| SerdeError(err.to_string()))
    }
}

impl Mtb {
    /// Creates "dummy" Mtb with consent status `REJECTED`.
    /// The created MtbFile does not contain all information, just enough to contain the
    /// information, that the patient with given ID has rejected the consent.
    pub fn new_with_consent_rejected(patient_id: &str) -> Mtb {
        Mtb {
            care_plans: None,
            claim_responses: None,
            claims: None,
            consent: Consent {
                id: None,
                patient: None,
                status: None,
            },
            diagnoses: None,
            ecog_status: None,
            episode: None,
            genetic_counselling_requests: None,
            histology_reports: None,
            last_guideline_therapies: None,
            molecular_therapies: None,
            ngs_reports: None,
            patient: MtbPatient {
                address: None,
                birth_date: String::new(),
                date_of_death: None,
                gender: CodingGender {
                    code: Gender::Male,
                    display: None,
                    system: None,
                    version: None,
                },
                id: patient_id.to_string(),
                insurance: None,
            },
            previous_guideline_therapies: None,
            recommendations: None,
            responses: None,
            specimens: None,
            study_inclusion_requests: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MTB_JSON: &str = include_str!("../tests/mv64e-mtb-fake-patient.json");

    #[test]
    fn should_deserialize_json_string() {
        let mtbfile = Mtb::from_str(MTB_JSON);
        assert!(mtbfile.is_ok())
    }
}
