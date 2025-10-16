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
            diagnoses: vec![],
            performance_status: None,
            episodes_of_care: vec![],
            histology_reports: None,
            guideline_therapies: None,
            ngs_reports: None,
            ihc_reports: None,
            patient: Patient {
                address: None,
                age: None,
                birth_date: String::new(),
                date_of_death: None,
                gender: GenderCoding {
                    code: GenderCodingCode::Female,
                    display: None,
                    system: None,
                    version: None,
                },
                id: patient_id.to_string(),
                health_insurance: HealthInsurance {
                    health_insurance_type: HealthInsuranceCoding {
                        code: HealthInsuranceCodingCode::Unk,
                        display: None,
                        system: None,
                        version: None,
                    },
                    reference: None,
                },
                vital_status: None,
                managing_site: None,
            },
            responses: None,
            specimens: None,
            guideline_procedures: None,
            family_member_histories: None,
            follow_ups: None,
            prior_diagnostic_reports: None,
            systemic_therapies: None,
            metadata: None,
            msi_findings: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::*;

    const MTB_JSON: &str = include_str!("../tests/mv64e-mtb-fake-patient.json");

    #[test]
    fn should_deserialize_json_string() {
        let mtbfile = Mtb::from_str(MTB_JSON);
        assert!(mtbfile.is_ok())
    }

    #[test]
    fn should_keep_timezone() {
        let m = Regex::from_str(r#""birthDate":"\d{4}-\d{2}-\d{2}""#)
            .unwrap()
            .find(MTB_JSON)
            .unwrap();

        let mtbfile = Mtb::from_str(MTB_JSON).unwrap();
        let actual = serde_json::to_string(&mtbfile).unwrap();
        assert!(actual.contains(m.as_str()));
    }
}
