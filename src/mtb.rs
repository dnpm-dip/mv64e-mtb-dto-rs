use crate::year_month_format::{serialize_option_year_month_format, serialize_year_month_format};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Mtb {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub care_plans: Option<Vec<MtbCarePlan>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_responses: Option<Vec<ClaimResponse>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<Claim>>,

    pub diagnoses: Vec<MtbDiagnosis>,

    pub episodes_of_care: Vec<MtbEpisodeOfCare>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_member_histories: Option<Vec<FamilyMemberHistory>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_ups: Option<Vec<FollowUp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guideline_procedures: Option<Vec<OncoProcedure>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guideline_therapies: Option<Vec<MtbSystemicTherapy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histology_reports: Option<Vec<HistologyReport>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ihc_reports: Option<Vec<IhcReport>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MvhMetadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub msi_findings: Option<Vec<Msi>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ngs_reports: Option<Vec<SomaticNgsReport>>,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_status: Option<Vec<PerformanceStatus>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_diagnostic_reports: Option<Vec<PriorDiagnosticReport>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<Response>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub specimens: Option<Vec<TumorSpecimen>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub systemic_therapies: Option<Vec<SystemicTherapy>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MtbCarePlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genetic_counseling_recommendation: Option<GeneticCounselingRecommendation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histology_reevaluation_requests: Option<Vec<HistologyReevaluationRequest>>,

    pub id: String,

    pub issued_on: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_recommendations: Option<Vec<MtbMedicationRecommendation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_sequencing_performed_reason: Option<CarePlanNoSequencingPerformedReasonCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_recommendations: Option<Vec<ProcedureRecommendation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebiopsy_requests: Option<Vec<RebiopsyRequest>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations_missing_reason: Option<MtbCarePlanRecommendationsMissingReasonCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub study_enrollment_recommendations: Option<Vec<MtbStudyEnrollmentRecommendation>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct GeneticCounselingRecommendation {
    pub id: String,

    pub issued_on: String,

    pub patient: Reference,

    pub reason: GeneticCounselingRecommendationReasonCoding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub id: String,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneticCounselingRecommendationReasonCoding {
    pub code: GeneticCounselingRecommendationReasonCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum GeneticCounselingRecommendationReasonCodingCode {
    #[serde(rename = "family-anamnesis")]
    FamilyAnamnesis,

    Other,

    #[serde(rename = "secondary-tumor")]
    SecondaryTumor,

    #[serde(rename = "self-anamnesis")]
    SelfAnamnesis,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct HistologyReevaluationRequest {
    pub id: String,

    pub issued_on: String,

    pub patient: Reference,

    pub specimen: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MtbMedicationRecommendation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<MtbMedicationRecommendationCategoryCoding>>,

    pub id: String,

    pub issued_on: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_evidence: Option<LevelOfEvidence>,

    pub medication: Vec<AtcUnregisteredMedicationCoding>,

    pub patient: Reference,

    pub priority: RecommendationPriorityCoding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_variants: Option<Vec<GeneAlterationReference>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_type: Option<MtbMedicationRecommendationUseTypeCoding>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbMedicationRecommendationCategoryCoding {
    pub code: MtbMedicationRecommendationCategoryCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum MtbMedicationRecommendationCategoryCodingCode {
    #[serde(rename = "CH")]
    Ch,

    #[serde(rename = "HO")]
    Ho,

    #[serde(rename = "IM")]
    Im,

    #[serde(rename = "SO")]
    So,

    #[serde(rename = "SZ")]
    Sz,

    #[serde(rename = "ZS")]
    Zs,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelOfEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addendums: Option<Vec<LevelOfEvidenceAddendumCoding>>,

    pub grading: LevelOfEvidenceGradingCoding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<PublicationReference>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelOfEvidenceAddendumCoding {
    pub code: LevelOfEvidenceAddendumCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum LevelOfEvidenceAddendumCodingCode {
    #[serde(rename = "is")]
    Is,

    #[serde(rename = "iv")]
    Iv,

    R,

    Z,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelOfEvidenceGradingCoding {
    pub code: LevelOfEvidenceGradingCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum LevelOfEvidenceGradingCodingCode {
    #[serde(rename = "m1A")]
    M1A,

    #[serde(rename = "m1B")]
    M1B,

    #[serde(rename = "m1C")]
    M1C,

    #[serde(rename = "m2A")]
    M2A,

    #[serde(rename = "m2B")]
    M2B,

    #[serde(rename = "m2C")]
    M2C,

    M3,

    M4,

    Undefined,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub id: String,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_reference_type: Option<String>,

    pub system: PublicationSystem,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum PublicationSystem {
    #[serde(rename = "https://pubmed.ncbi.nlm.nih.gov")]
    PubmedNcbiNlmNihGov,

    #[serde(rename = "https://www.doi.org")]
    DoiOrg,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AtcUnregisteredMedicationCoding {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub system: RequestedMedicationSystem,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum RequestedMedicationSystem {
    #[serde(rename = "http://fhir.de/CodeSystem/bfarm/atc")]
    FhirDeCodeSystemBfarmAtc,

    Undefined,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecommendationPriorityCoding {
    pub code: RecommendationPriorityCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum RecommendationPriorityCodingCode {
    #[serde(rename = "1")]
    Code1,

    #[serde(rename = "2")]
    Code2,

    #[serde(rename = "3")]
    Code3,

    #[serde(rename = "4")]
    Code4,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneAlterationReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene: Option<Coding>,

    pub variant: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Coding {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbMedicationRecommendationUseTypeCoding {
    pub code: MtbMedicationRecommendationUseTypeCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum MtbMedicationRecommendationUseTypeCodingCode {
    Compassionate,

    #[serde(rename = "in-label")]
    InLabel,

    #[serde(rename = "off-label")]
    OffLabel,

    #[serde(rename = "sec-preventive")]
    SecPreventive,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CarePlanNoSequencingPerformedReasonCoding {
    pub code: NoSequencingPerformedReasonCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum NoSequencingPerformedReasonCode {
    #[serde(rename = "non-genetic-cause")]
    NonGeneticCause,

    #[serde(rename = "not-rare-disease")]
    NotRareDisease,

    Other,

    Psychosomatic,

    #[serde(rename = "targeted-diagnostics-recommended")]
    TargetedDiagnosticsRecommended,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ProcedureRecommendation {
    pub code: MtbProcedureRecommendationCategoryCoding,

    pub id: String,

    pub issued_on: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_evidence: Option<LevelOfEvidence>,

    pub patient: Reference,

    pub priority: RecommendationPriorityCoding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_variants: Option<Vec<GeneAlterationReference>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbProcedureRecommendationCategoryCoding {
    pub code: MtbProcedureRecommendationCategoryCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum MtbProcedureRecommendationCategoryCodingCode {
    #[serde(rename = "AS")]
    As,

    #[serde(rename = "OP")]
    Op,

    #[serde(rename = "SO")]
    So,

    #[serde(rename = "ST")]
    St,

    #[serde(rename = "WS")]
    Ws,

    #[serde(rename = "WW")]
    Ww,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RebiopsyRequest {
    pub id: String,

    pub issued_on: String,

    pub patient: Reference,

    pub tumor_entity: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbCarePlanRecommendationsMissingReasonCoding {
    pub code: MtbCarePlanRecommendationsMissingReasonCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum MtbCarePlanRecommendationsMissingReasonCodingCode {
    #[serde(rename = "no-target")]
    NoTarget,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MtbStudyEnrollmentRecommendation {
    pub id: String,

    pub issued_on: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_evidence: Option<LevelOfEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<AtcUnregisteredMedicationCoding>>,

    pub patient: Reference,

    pub priority: RecommendationPriorityCoding,

    pub reason: Reference,

    pub study: Vec<StudyReference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_variants: Option<Vec<GeneAlterationReference>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub id: String,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub study_reference_type: Option<String>,

    pub system: StudySystem,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum StudySystem {
    #[serde(rename = "DRKS")]
    Drks,

    #[serde(rename = "EUDAMED")]
    Eudamed,

    #[serde(rename = "Eudra-CT")]
    EudraCt,

    #[serde(rename = "NCT")]
    Nct,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse {
    pub claim: Reference,

    pub id: String,

    pub issued_on: String,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClaimResponseStatusCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<Vec<ClaimResponseStatusReasonCoding>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimResponseStatusCoding {
    pub code: ClaimResponseStatusCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum ClaimResponseStatusCodingCode {
    Accepted,

    Rejected,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimResponseStatusReasonCoding {
    pub code: ClaimResponseStatusReasonCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ClaimResponseStatusReasonCodingCode {
    #[serde(rename = "approval-revocation")]
    ApprovalRevocation,

    #[serde(rename = "formal-reasons")]
    FormalReasons,

    #[serde(rename = "inclusion-in-study")]
    InclusionInStudy,

    #[serde(rename = "insufficient-evidence")]
    InsufficientEvidence,

    Other,

    #[serde(rename = "other-therapy-recommended")]
    OtherTherapyRecommended,

    #[serde(rename = "standard-therapy-not-exhausted")]
    StandardTherapyNotExhausted,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub id: String,

    pub issued_on: String,

    pub patient: Reference,

    pub recommendation: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_medication: Option<Vec<AtcUnregisteredMedicationCoding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<ClaimStageCoding>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimStageCoding {
    pub code: ClaimStageCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ClaimStageCodingCode {
    #[serde(rename = "follow-up-claim")]
    FollowUpClaim,

    #[serde(rename = "initial-claim")]
    InitialClaim,

    Revocation,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MtbDiagnosis {
    pub code: Coding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub germline_codes: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grading: Option<Grading>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guideline_treatment_status: Option<MtbDiagnosisGuidelineTreatmentStatusCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histology: Option<Vec<Reference>>,

    pub id: String,

    #[serde(rename = "type")]
    pub mtb_diagnosis_type: Type,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,

    pub patient: Reference,

    pub recorded_on: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging: Option<Staging>,

    pub topography: Coding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grading {
    pub history: Vec<TumorGrading>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorGrading {
    pub codes: Vec<Coding>,

    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbDiagnosisGuidelineTreatmentStatusCoding {
    pub code: MtbDiagnosisGuidelineTreatmentStatusCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum MtbDiagnosisGuidelineTreatmentStatusCodingCode {
    Exhausted,

    Impossible,

    #[serde(rename = "no-guidelines-available")]
    NoGuidelinesAvailable,

    #[serde(rename = "non-exhausted")]
    NonExhausted,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Type {
    pub history: Vec<History>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct History {
    pub date: String,

    pub value: MtbDiagnosisCoding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbDiagnosisCoding {
    pub code: ValueCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum ValueCode {
    Main,

    Metachronous,

    Secondary,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Staging {
    pub history: Vec<TumorStaging>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct TumorStaging {
    pub date: String,

    pub method: TumorStagingMethodCoding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_classifications: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tnm_classification: Option<TnmClassification>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorStagingMethodCoding {
    pub code: TumorStagingMethodCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum TumorStagingMethodCodingCode {
    Clinical,

    Pathologic,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TnmClassification {
    pub metastasis: Coding,

    pub nodes: Coding,

    pub tumor: Coding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbEpisodeOfCare {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses: Option<Vec<Reference>>,

    pub id: String,

    pub patient: Reference,

    pub period: PeriodDate,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodDate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    pub start: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyMemberHistory {
    pub id: String,

    pub patient: Reference,

    pub relationship: FamilyMemberHistoryRelationshipTypeCoding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyMemberHistoryRelationshipTypeCoding {
    pub code: FamilyMemberHistoryRelationshipTypeCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum FamilyMemberHistoryRelationshipTypeCodingCode {
    #[serde(rename = "EXT")]
    Ext,

    #[serde(rename = "FAMMEMB")]
    Fammemb,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct FollowUp {
    pub date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_contact_date: Option<String>,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_status: Option<FollowUpPatientStatusCoding>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FollowUpPatientStatusCoding {
    pub code: FollowUpPatientStatusCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum FollowUpPatientStatusCodingCode {
    #[serde(rename = "lost-to-fu")]
    LostToFu,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct OncoProcedure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub based_on: Option<Reference>,

    pub code: OncoProcedureCoding,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<MtbTherapyIntentCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PeriodDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reference>,

    pub recorded_on: String,

    pub status: TherapyStatusCoding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<MtbTherapyStatusReasonCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapy_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OncoProcedureCoding {
    pub code: OncoProcedureCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum OncoProcedureCodingCode {
    #[serde(rename = "nuclear-medicine")]
    NuclearMedicine,

    #[serde(rename = "radio-therapy")]
    RadioTherapy,

    Surgery,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbTherapyIntentCoding {
    pub code: MtbTherapyIntentCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum MtbTherapyIntentCodingCode {
    K,

    P,

    S,

    X,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TherapyStatusCoding {
    pub code: TherapyStatusCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum TherapyStatusCodingCode {
    Completed,

    #[serde(rename = "not-done")]
    NotDone,

    #[serde(rename = "on-going")]
    OnGoing,

    Stopped,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbTherapyStatusReasonCoding {
    pub code: MtbTherapyStatusReasonCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum MtbTherapyStatusReasonCodingCode {
    #[serde(rename = "best-supportive-care")]
    BestSupportiveCare,

    #[serde(rename = "chronic-remission")]
    ChronicRemission,

    Deterioration,

    #[serde(rename = "lost-to-fu")]
    LostToFu,

    #[serde(rename = "medical-reasons")]
    MedicalReasons,

    #[serde(rename = "no-indication")]
    NoIndication,

    Other,

    #[serde(rename = "other-therapy-chosen")]
    OtherTherapyChosen,

    #[serde(rename = "patient-death")]
    PatientDeath,

    #[serde(rename = "patient-refusal")]
    PatientRefusal,

    #[serde(rename = "patient-wish")]
    PatientWish,

    #[serde(rename = "payment-ended")]
    PaymentEnded,

    #[serde(rename = "payment-pending")]
    PaymentPending,

    #[serde(rename = "payment-refused")]
    PaymentRefused,

    Progression,

    #[serde(rename = "regular-completion")]
    RegularCompletion,

    #[serde(rename = "regular-completion-with-dosage-reduction")]
    RegularCompletionWithDosageReduction,

    #[serde(rename = "regular-completion-with-substance-change")]
    RegularCompletionWithSubstanceChange,

    Toxicity,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MtbSystemicTherapy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub based_on: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<MtbSystemicTherapyCategoryCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dosage: Option<MtbSystemicTherapyDosageDensityCoding>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<MtbTherapyIntentCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<AtcUnregisteredMedicationCoding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PeriodDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_fulfillment_status:
        Option<MtbSystemicTherapyRecommendationFulfillmentStatusCoding>,

    pub recorded_on: String,

    pub status: TherapyStatusCoding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<MtbTherapyStatusReasonCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapy_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbSystemicTherapyCategoryCoding {
    pub code: MtbSystemicTherapyCategoryCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum MtbSystemicTherapyCategoryCodingCode {
    A,

    I,

    N,

    O,

    S,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbSystemicTherapyDosageDensityCoding {
    pub code: MtbSystemicTherapyDosageDensityCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum MtbSystemicTherapyDosageDensityCodingCode {
    #[serde(rename = "over-50%")]
    Over50,

    #[serde(rename = "under-50%")]
    Under50,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbSystemicTherapyRecommendationFulfillmentStatusCoding {
    pub code: MtbSystemicTherapyRecommendationFulfillmentStatusCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum MtbSystemicTherapyRecommendationFulfillmentStatusCodingCode {
    Complete,

    Partial,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct HistologyReport {
    pub id: String,

    pub issued_on: String,

    pub patient: Reference,

    pub results: HistologyReportResults,

    pub specimen: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct HistologyReportResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_cell_content: Option<TumorCellContent>,

    pub tumor_morphology: TumorMorphology,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorCellContent {
    pub id: String,

    pub method: TumorCellContentMethodCoding,

    pub patient: Reference,

    pub specimen: Reference,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorCellContentMethodCoding {
    pub code: TumorCellContentMethodCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum TumorCellContentMethodCodingCode {
    Bioinformatic,

    Histologic,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorMorphology {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    pub patient: Reference,

    pub specimen: Reference,

    pub value: Coding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct IhcReport {
    pub id: String,

    pub issued_on: String,

    pub patient: Reference,

    pub results: IhcReportResults,

    pub specimen: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct IhcReportResults {
    pub msi_mmr: Vec<ProteinExpression>,

    pub protein_expression: Vec<ProteinExpression>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProteinExpressionIcScoreCoding {
    pub code: ProteinExpressionIcScoreCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ProteinExpressionIcScoreCodingCode {
    #[serde(rename = "0")]
    Code0,

    #[serde(rename = "1")]
    Code1,

    #[serde(rename = "2")]
    Code2,

    #[serde(rename = "3")]
    Code3,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProteinExpressionTcScoreCoding {
    pub code: ProteinExpressionTcScoreCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ProteinExpressionTcScoreCodingCode {
    #[serde(rename = "0")]
    Code0,

    #[serde(rename = "1")]
    Code1,

    #[serde(rename = "2")]
    Code2,

    #[serde(rename = "3")]
    Code3,

    #[serde(rename = "4")]
    Code4,

    #[serde(rename = "5")]
    Code5,

    #[serde(rename = "6")]
    Code6,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProteinExpressionResultCoding {
    pub code: ProteinExpressionResultCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ProteinExpressionResultCodingCode {
    Exp,

    #[serde(rename = "not-exp")]
    NotExp,

    #[serde(rename = "1+")]
    Code1Plus,

    #[serde(rename = "2+")]
    Code2Plus,

    #[serde(rename = "3+")]
    Code3Plus,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ProteinExpression {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ic_score: Option<ProteinExpressionIcScoreCoding>,

    pub id: String,

    pub patient: Reference,

    pub protein: Coding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_score: Option<ProteinExpressionTcScoreCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tps_score: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps_score: Option<i64>,

    pub value: ProteinExpressionResultCoding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MvhMetadata {
    pub model_project_consent: ModelProjectConsent,

    #[serde(rename = "type")]
    pub mvh_metadata_type: MvhSubmissionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub research_consents: Option<Vec<HashMap<String, Option<serde_json::Value>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_research_consent_missing: Option<ResearchConsentReasonMissing>,

    #[serde(rename = "transferTAN")]
    pub transfer_tan: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ResearchConsentReasonMissing {
    #[serde(rename = "consent-not-returned")]
    ConsentNotReturned,

    #[serde(rename = "organizational-issues")]
    OrganizationalIssues,

    #[serde(rename = "other-patient-reason")]
    OtherPatientReason,

    #[serde(rename = "patient-inability")]
    PatientInability,

    #[serde(rename = "patient-refusal")]
    PatientRefusal,

    #[serde(rename = "technical-issues")]
    TechnicalIssues,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelProjectConsent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    pub provisions: Vec<Provision>,

    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provision {
    pub date: String,

    #[serde(rename = "type")]
    pub provision_type: ConsentProvision,

    pub purpose: ModelProjectConsentPurpose,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum ConsentProvision {
    Deny,

    Permit,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ModelProjectConsentPurpose {
    #[serde(rename = "case-identification")]
    CaseIdentification,

    Reidentification,

    Sequencing,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum MvhSubmissionType {
    Addition,

    Correction,

    Followup,

    Initial,

    Test,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Msi {
    pub id: String,

    pub interpretation: MsiInterpretationCoding,

    pub method: MsiMethodCoding,

    pub patient: Reference,

    pub specimen: Reference,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MsiInterpretationCoding {
    pub code: MsiInterpretationCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum MsiInterpretationCodingCode {
    #[serde(rename = "mmr-deficient")]
    MmrDeficient,

    #[serde(rename = "mmr-proficient")]
    MmrProficient,

    #[serde(rename = "msi-high")]
    MsiHigh,

    #[serde(rename = "msi-low")]
    MsiLow,

    Stable,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MsiMethodCoding {
    pub code: MsiMethodCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum MsiMethodCodingCode {
    #[serde(rename = "bioinformatic")]
    Bioinformatic,

    #[serde(rename = "IHC")]
    Ihc,

    #[serde(rename = "PCR")]
    Pcr,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct SomaticNgsReport {
    pub id: String,

    pub issued_on: String,

    pub metadata: Vec<NgsReportMetadata>,

    pub patient: Reference,

    pub results: NgsReportResults,

    #[serde(rename = "type")]
    pub somatic_ngs_report_type: NgsReportCoding,

    pub specimen: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct NgsReportMetadata {
    pub kit_manufacturer: String,

    pub kit_type: String,

    pub pipeline: String,

    pub reference_genome: String,

    pub sequencer: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct NgsReportResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brcaness: Option<Brcaness>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_number_variants: Option<Vec<Cnv>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dna_fusions: Option<Vec<DnaFusion>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_score: Option<HrdScore>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rna_fusions: Option<Vec<RnaFusion>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rna_seqs: Option<Vec<RnaSeq>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_variants: Option<Vec<Snv>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmb: Option<Tmb>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_cell_content: Option<TumorCellContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Brcaness {
    pub confidence_range: ConfidenceRange,

    pub id: String,

    pub patient: Reference,

    pub specimen: Reference,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfidenceRange {
    pub max: f64,

    pub min: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Cnv {
    pub chromosome: Chromosome,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_a: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_b: Option<f64>,

    #[serde(rename = "type")]
    pub cnv_type: CnvCoding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_number_neutral_lo_h: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_range: Option<EndRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<VariantExternalId>>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization: Option<Vec<BaseVariantLocalizationCoding>>,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_copy_number: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_affected_genes: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_focality: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_range: Option<StartRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_copy_number: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum Chromosome {
    Chr1,

    Chr10,

    Chr11,

    Chr12,

    Chr13,

    Chr14,

    Chr15,

    Chr16,

    Chr17,

    Chr18,

    Chr19,

    Chr2,

    Chr20,

    Chr21,

    Chr22,

    Chr3,

    Chr4,

    Chr5,

    Chr6,

    Chr7,

    Chr8,

    Chr9,

    #[serde(rename = "chrX")]
    ChrX,

    #[serde(rename = "chrY")]
    ChrY,

    #[serde(rename = "chrMT")]
    ChrMt,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CnvCoding {
    pub code: CnvCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CnvCodingCode {
    #[serde(rename = "high-level-gain")]
    HighLevelGain,

    Loss,

    #[serde(rename = "low-level-gain")]
    LowLevelGain,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EndRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VariantExternalId {
    pub system: ExternalIdSystem,

    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ExternalIdSystem {
    #[serde(rename = "https://cancer.sanger.ac.uk/cosmic")]
    CancerSangerAcUkCosmic,

    #[serde(rename = "https://www.ensembl.org")]
    EnsemblOrg,

    #[serde(rename = "https://www.ncbi.nlm.nih.gov/entrez")]
    NcbiNlmNihGovEntrez,

    #[serde(rename = "https://www.ncbi.nlm.nih.gov/snp")]
    NcbiNlmNihGovSnp,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BaseVariantLocalizationCoding {
    pub code: BaseVariantLocalizationCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum BaseVariantLocalizationCodingCode {
    #[serde(rename = "coding-region")]
    CodingRegion,

    Intergenic,

    Intronic,

    #[serde(rename = "regulatory-region")]
    RegulatoryRegion,

    #[serde(rename = "splicing-region")]
    SplicingRegion,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StartRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct DnaFusion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<VariantExternalId>>,

    #[serde(rename = "fusionPartner3prime")]
    pub fusion_partner3_prime: DnaFusionFusionPartner3Prime,

    #[serde(rename = "fusionPartner5prime")]
    pub fusion_partner5_prime: DnaFusionFusionPartner5Prime,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization: Option<Vec<BaseVariantLocalizationCoding>>,

    pub patient: Reference,

    pub reported_num_reads: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnaFusionFusionPartner3Prime {
    pub chromosome: Chromosome,

    pub gene: Coding,

    pub position: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnaFusionFusionPartner5Prime {
    pub chromosome: Chromosome,

    pub gene: Coding,

    pub position: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HrdScore {
    pub components: Components,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretation: Option<HrdScoreInterpretationCoding>,

    pub patient: Reference,

    pub specimen: Reference,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Components {
    pub loh: f64,

    pub lst: f64,

    pub tai: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HrdScoreInterpretationCoding {
    pub code: InterpretationCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum InterpretationCodingCode {
    High,

    Intermediate,

    Low,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<VariantExternalId>>,

    #[serde(rename = "fusionPartner3prime")]
    pub fusion_partner3_prime: RnaFusionFusionPartner3Prime,

    #[serde(rename = "fusionPartner5prime")]
    pub fusion_partner5_prime: RnaFusionFusionPartner5Prime,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization: Option<Vec<BaseVariantLocalizationCoding>>,

    pub patient: Reference,

    pub reported_num_reads: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusionFusionPartner3Prime {
    pub exon_id: String,

    pub gene: Coding,

    pub position: f64,

    pub strand: RnaFusionStrand,

    pub transcript_id: TranscriptId,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum RnaFusionStrand {
    #[serde(rename = "+")]
    Empty,

    #[serde(rename = "-")]
    RnaFusionStrand,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranscriptId {
    pub system: TranscriptIdSystem,

    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum TranscriptIdSystem {
    #[serde(rename = "https://www.ensembl.org")]
    EnsemblOrg,

    #[serde(rename = "https://www.ncbi.nlm.nih.gov/refseq")]
    NcbiNlmNihGovRefseq,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusionFusionPartner5Prime {
    pub exon_id: String,

    pub gene: Coding,

    pub position: f64,

    pub strand: RnaFusionStrand,

    pub transcript_id: TranscriptId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RnaSeq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_ranking: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<VariantExternalId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene: Option<Coding>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub library_size: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization: Option<Vec<BaseVariantLocalizationCoding>>,

    pub patient: Reference,

    pub raw_counts: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tissue_corrected_expression: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_id: Option<TranscriptId>,

    pub transcripts_per_million: f64,

    pub variant: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Snv {
    pub allelic_frequency: f64,

    pub alt_allele: String,

    pub chromosome: Chromosome,

    pub dna_change: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exon_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<VariantExternalId>>,

    pub gene: Coding,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretation: Option<ClinVarCoding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization: Option<Vec<BaseVariantLocalizationCoding>>,

    pub patient: Reference,

    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protein_change: Option<String>,

    pub read_depth: i64,

    pub ref_allele: String,

    pub transcript_id: TranscriptId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClinVarCoding {
    pub code: ClinVarCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ClinVarCodingCode {
    #[serde(rename = "1")]
    Code1,

    #[serde(rename = "2")]
    Code2,

    #[serde(rename = "3")]
    Code3,

    #[serde(rename = "4")]
    Code4,

    #[serde(rename = "5")]
    Code5,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Position {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tmb {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretation: Option<TmbInterpretationCoding>,

    pub patient: Reference,

    pub specimen: Reference,

    pub value: TmbResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TmbInterpretationCoding {
    pub code: InterpretationCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TmbResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NgsReportCoding {
    pub code: NgsReportCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum NgsReportCodingCode {
    Array,

    Exome,

    #[serde(rename = "genome-long-read")]
    GenomeLongRead,

    #[serde(rename = "genome-short-read")]
    GenomeShortRead,

    Karyotyping,

    Other,

    Panel,

    Single,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<Age>,

    #[serde(serialize_with = "serialize_year_month_format")]
    pub birth_date: String,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_option_year_month_format"
    )]
    pub date_of_death: Option<String>,

    pub gender: GenderCoding,

    pub health_insurance: HealthInsurance,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_site: Option<Coding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vital_status: Option<VitalStatusCoding>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub municipality_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Age {
    pub unit: Unit,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Unit {
    Months,

    Years,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenderCoding {
    pub code: GenderCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum GenderCodingCode {
    Female,

    Male,

    Other,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthInsurance {
    #[serde(rename = "type")]
    pub health_insurance_type: HealthInsuranceCoding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Reference>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthInsuranceCoding {
    pub code: HealthInsuranceCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum HealthInsuranceCodingCode {
    #[serde(rename = "BEI")]
    Bei,

    #[serde(rename = "BG")]
    Bg,

    #[serde(rename = "GKV")]
    Gkv,

    #[serde(rename = "GPV")]
    Gpv,

    #[serde(rename = "PKV")]
    Pkv,

    #[serde(rename = "PPV")]
    Ppv,

    #[serde(rename = "SEL")]
    Sel,

    #[serde(rename = "SKT")]
    Skt,

    #[serde(rename = "SOZ")]
    Soz,

    #[serde(rename = "UNK")]
    Unk,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VitalStatusCoding {
    pub code: VitalStatusCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum VitalStatusCodingCode {
    Alive,

    Deceased,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceStatus {
    pub effective_date: String,

    pub id: String,

    pub patient: Reference,

    pub value: EcogCoding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcogCoding {
    pub code: EcogCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum EcogCodingCode {
    #[serde(rename = "0")]
    Code0,

    #[serde(rename = "1")]
    Code1,

    #[serde(rename = "2")]
    Code2,

    #[serde(rename = "3")]
    Code3,

    #[serde(rename = "4")]
    Code4,

    #[serde(rename = "5")]
    Code5,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PriorDiagnosticReport {
    pub id: String,

    pub issued_on: String,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<Reference>,

    #[serde(rename = "type")]
    pub prior_diagnostic_report_type: MolecularDiagnosticReportCoding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,

    pub specimen: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MolecularDiagnosticReportCoding {
    pub code: MolecularDiagnosticReportCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum MolecularDiagnosticReportCodingCode {
    Array,

    Exome,

    #[serde(rename = "FISH")]
    Fish,

    #[serde(rename = "fusion-panel")]
    FusionPanel,

    #[serde(rename = "gene-panel")]
    GenePanel,

    #[serde(rename = "genome-long-read")]
    GenomeLongRead,

    #[serde(rename = "genome-short-read")]
    GenomeShortRead,

    Karyotyping,

    Other,

    Panel,

    #[serde(rename = "PCR")]
    Pcr,

    Single,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub effective_date: String,

    pub id: String,

    pub method: ResponseMethodCoding,

    pub patient: Reference,

    pub therapy: Reference,

    pub value: RecistCoding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseMethodCoding {
    pub code: ResponseMethodCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ResponseMethodCodingCode {
    #[serde(rename = "RANO")]
    Rano,

    #[serde(rename = "RECIST")]
    Recist,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecistCoding {
    pub code: RecistCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum RecistCodingCode {
    #[serde(rename = "CR")]
    Cr,

    #[serde(rename = "MR")]
    Mr,

    #[serde(rename = "NA")]
    Na,

    #[serde(rename = "PD")]
    Pd,

    #[serde(rename = "PR")]
    Pr,

    #[serde(rename = "SD")]
    Sd,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorSpecimen {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,

    pub diagnosis: Reference,

    pub id: String,

    pub patient: Reference,

    #[serde(rename = "type")]
    pub tumor_specimen_type: TumorSpecimenCoding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Collection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    pub localization: TumorSpecimenCollectionLocalizationCoding,

    pub method: TumorSpecimenCollectionMethodCoding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorSpecimenCollectionLocalizationCoding {
    pub code: TumorSpecimenCollectionLocalizationCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum TumorSpecimenCollectionLocalizationCodingCode {
    #[serde(rename = "cellfree-dna")]
    CellfreeDna,

    #[serde(rename = "local-recurrence")]
    LocalRecurrence,

    Metastasis,

    #[serde(rename = "primary-tumor")]
    PrimaryTumor,

    #[serde(rename = "regional-lymph-nodes")]
    RegionalLymphNodes,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorSpecimenCollectionMethodCoding {
    pub code: TumorSpecimenCollectionMethodCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum TumorSpecimenCollectionMethodCodingCode {
    Biopsy,

    Cytology,

    #[serde(rename = "liquid-biopsy")]
    LiquidBiopsy,

    Resection,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorSpecimenCoding {
    pub code: TumorSpecimenCodingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum TumorSpecimenCodingCode {
    #[serde(rename = "cryo-frozen")]
    CryoFrozen,

    #[serde(rename = "FFPE")]
    Ffpe,

    #[serde(rename = "fresh-tissue")]
    FreshTissue,

    #[serde(rename = "liquid-biopsy")]
    LiquidBiopsy,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SystemicTherapy {
    pub history: Vec<MtbSystemicTherapy>,
}
