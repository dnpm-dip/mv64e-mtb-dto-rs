use serde::{Deserialize, Serialize};

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
    pub ngs_reports: Option<Vec<SomaticNgsReportMetadata>>,

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
    pub notes: Option<Vec<String>>,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_recommendations: Option<Vec<ProcedureRecommendation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebiopsy_requests: Option<Vec<RebiopsyRequest>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<CodingMtbCarePlanStatusReason>,

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

    pub reason: CodingGeneticCounselingRecommendationReason,
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
pub struct CodingGeneticCounselingRecommendationReason {
    pub code: ReasonCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ReasonCode {
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
    pub category: Option<CodingMtbMedicationRecommendationCategory>,

    pub id: String,

    pub issued_on: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_evidence: Option<LevelOfEvidence>,

    pub medication: Vec<CodingAtcUnregisteredMedication>,

    pub patient: Reference,

    pub priority: CodingRecommendationPriority,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_variants: Option<Vec<GeneAlterationReference>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_type: Option<CodingMtbMedicationRecommendationUseType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingMtbMedicationRecommendationCategory {
    pub code: CodingMtbMedicationRecommendationCategoryCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CodingMtbMedicationRecommendationCategoryCode {
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
    pub addendums: Option<Vec<CodingLevelOfEvidenceAddendum>>,

    pub grading: CodingLevelOfEvidenceGrading,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<PublicationReference>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingLevelOfEvidenceAddendum {
    pub code: AddendumCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum AddendumCode {
    #[serde(rename = "is")]
    Is,

    #[serde(rename = "iv")]
    Iv,

    R,

    Z,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingLevelOfEvidenceGrading {
    pub code: LevelOfEvidenceCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum LevelOfEvidenceCode {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum PublicationSystem {
    #[serde(rename = "https://pubmed.ncbi.nlm.nih.gov")]
    HttpsPubmedNcbiNlmNihGov,

    #[serde(rename = "https://www.doi.org")]
    HttpsWwwDoiOrg,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingAtcUnregisteredMedication {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub system: RequestedMedicationSystem,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum RequestedMedicationSystem {
    #[serde(rename = "http://fhir.de/CodeSystem/bfarm/atc")]
    HttpFhirDeCodeSystemBfarmAtc,

    Undefined,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingRecommendationPriority {
    pub code: PriorityCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum PriorityCode {
    #[serde(rename = "1")]
    The1,

    #[serde(rename = "2")]
    The2,

    #[serde(rename = "3")]
    The3,

    #[serde(rename = "4")]
    The4,
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
pub struct CodingMtbMedicationRecommendationUseType {
    pub code: UseTypeCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum UseTypeCode {
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
#[serde(rename_all = "camelCase")]
pub struct ProcedureRecommendation {
    pub code: CodingMtbProcedureRecommendationCategory,

    pub id: String,

    pub issued_on: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_evidence: Option<LevelOfEvidence>,

    pub patient: Reference,

    pub priority: CodingRecommendationPriority,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_variants: Option<Vec<GeneAlterationReference>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingMtbProcedureRecommendationCategory {
    pub code: CodingMtbProcedureRecommendationCategoryCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CodingMtbProcedureRecommendationCategoryCode {
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
pub struct CodingMtbCarePlanStatusReason {
    pub code: CodingMtbCarePlanStatusReasonCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingMtbCarePlanStatusReasonCode {
    #[serde(rename = "no-target")]
    NoTarget,

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
pub struct MtbStudyEnrollmentRecommendation {
    pub id: String,

    pub issued_on: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_evidence: Option<CodingLevelOfEvidenceGrading>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<CodingAtcUnregisteredMedication>>,

    pub patient: Reference,

    pub priority: CodingRecommendationPriority,

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

#[derive(Debug, Serialize, Deserialize)]
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

    pub status: CodingClaimResponseStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<CodingClaimResponseStatusReason>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingClaimResponseStatus {
    pub code: CodingClaimResponseStatusCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum CodingClaimResponseStatusCode {
    Accepted,

    Rejected,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingClaimResponseStatusReason {
    pub code: CodingClaimResponseStatusReasonCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingClaimResponseStatusReasonCode {
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
    pub requested_medication: Option<Vec<CodingAtcUnregisteredMedication>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<CodingClaimStage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingClaimStage {
    pub code: StageCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum StageCode {
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
    pub guideline_treatment_status: Option<CodingMtbDiagnosisGuidelineTreatmentStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histology: Option<Vec<Reference>>,

    pub id: String,

    #[serde(rename = "type")]
    pub mtb_diagnosis_type: Type,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging: Option<Staging>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topography: Option<Coding>,
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
pub struct CodingMtbDiagnosisGuidelineTreatmentStatus {
    pub code: GuidelineTreatmentStatusCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum GuidelineTreatmentStatusCode {
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

    pub value: CodingMtbDiagnosis,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingMtbDiagnosis {
    pub code: CodingMtbDiagnosisCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum CodingMtbDiagnosisCode {
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

    pub method: CodingTumorStagingMethod,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_classifications: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tnm_classification: Option<TnmClassification>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTumorStagingMethod {
    pub code: CodingTumorStagingMethodCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum CodingTumorStagingMethodCode {
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
#[serde(rename_all = "camelCase")]
pub struct FollowUp {
    pub date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_status: Option<CodingFollowUpPatientStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingFollowUpPatientStatus {
    pub code: PatientStatusCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum PatientStatusCode {
    #[serde(rename = "lost-to-fu")]
    LostToFu,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct OncoProcedure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub based_on: Option<Reference>,

    pub code: CodingOncoProcedure,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<CodingMtbTherapyIntent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PeriodDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reference>,

    pub recorded_on: String,

    pub status: CodingTherapyStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<CodingMtbTherapyStatusReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapy_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingOncoProcedure {
    pub code: CodingOncoProcedureCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingOncoProcedureCode {
    #[serde(rename = "nuclear-medicine")]
    NuclearMedicine,

    #[serde(rename = "radio-therapy")]
    RadioTherapy,

    Surgery,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingMtbTherapyIntent {
    pub code: IntentCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum IntentCode {
    K,

    P,

    S,

    X,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTherapyStatus {
    pub code: CodingTherapyStatusCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingTherapyStatusCode {
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
pub struct CodingMtbTherapyStatusReason {
    pub code: CodingMtbTherapyStatusReasonCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingMtbTherapyStatusReasonCode {
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
    pub category: Option<CodingMtbSystemicTherapyCategory>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<CodingMtbTherapyIntent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<CodingAtcUnregisteredMedication>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,

    pub patient: Reference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PeriodDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_fulfillment_status: Option<CodingMtbSystemicTherapyRecommendationFulfillmentStatus>,

    pub recorded_on: String,

    pub status: CodingTherapyStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<CodingMtbTherapyStatusReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapy_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingMtbSystemicTherapyCategory {
    pub code: CodingMtbSystemicTherapyCategoryCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CodingMtbSystemicTherapyCategoryCode {
    A,

    I,

    N,

    O,

    S,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingMtbSystemicTherapyRecommendationFulfillmentStatus {
    pub code: RecommendationFulfillmentStatusCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationFulfillmentStatusCode {
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

    pub method: CodingTumorCellContentMethod,

    pub patient: Reference,

    pub specimen: Reference,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTumorCellContentMethod {
    pub code: CodingTumorCellContentMethodCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum CodingTumorCellContentMethodCode {
    Bioinformatic,

    Histologic,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorMorphology {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    pub patient: Reference,

    pub specimen: Reference,

    pub value: Coding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct IhcReport {
    pub block_ids: Vec<String>,

    pub id: String,

    pub issued_on: String,

    pub journal_id: String,

    pub patient: Reference,

    pub results: IhcReportResults,

    pub specimen: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct IhcReportResults {
    pub msi_mmr: Vec<MsiMmr>,

    pub protein_expression: Vec<ProteinExpression>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MsiMmr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ic_score: Option<CodingProteinExpressionIcScore>,

    pub id: String,

    pub patient: Reference,

    pub protein: Coding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_score: Option<CodingProteinExpressionTcScore>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tps_score: Option<i64>,

    pub value: CodingProteinExpressionResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingProteinExpressionIcScore {
    pub code: IcScoreCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum IcScoreCode {
    #[serde(rename = "0")]
    The0,

    #[serde(rename = "1")]
    The1,

    #[serde(rename = "2")]
    The2,

    #[serde(rename = "3")]
    The3,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingProteinExpressionTcScore {
    pub code: TcScoreCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TcScoreCode {
    #[serde(rename = "0")]
    The0,

    #[serde(rename = "1")]
    The1,

    #[serde(rename = "2")]
    The2,

    #[serde(rename = "3")]
    The3,

    #[serde(rename = "4")]
    The4,

    #[serde(rename = "5")]
    The5,

    #[serde(rename = "6")]
    The6,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingProteinExpressionResult {
    pub code: ProteinExpressionResultCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ProteinExpressionResultCode {
    Exp,

    #[serde(rename = "not-exp")]
    NotExp,

    #[serde(rename = "1+")]
    The1,

    #[serde(rename = "2+")]
    The2,

    #[serde(rename = "3+")]
    The3,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ProteinExpression {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ic_score: Option<CodingProteinExpressionIcScore>,

    pub id: String,

    pub patient: Reference,

    pub protein: Coding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_score: Option<CodingProteinExpressionTcScore>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tps_score: Option<i64>,

    pub value: CodingProteinExpressionResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct SomaticNgsReportMetadata {
    pub id: String,

    pub issued_on: String,

    pub metadata: Vec<Metadata>,

    pub patient: Reference,

    pub results: NgsReportResults,

    #[serde(rename = "type")]
    pub somatic_ngs_report_metadata_type: CodingNgsReport,

    pub specimen: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
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
    pub brcaness: Option<BrcAness>,

    pub copy_number_variants: Vec<Cnv>,

    pub dna_fusions: Vec<DnaFusion>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_score: Option<HrdScore>,

    pub rna_fusions: Vec<RnaFusion>,

    pub rna_seqs: Vec<RnaSeq>,

    pub simple_variants: Vec<Snv>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmb: Option<Tmb>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_cell_content: Option<TumorCellContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct BrcAness {
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
    pub cnv_type: CodingCnv,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_number_neutral_lo_h: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_range: Option<EndRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<VariantExternalId>>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization: Option<Vec<CodingBaseVariantLocalization>>,

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

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingCnv {
    pub code: CodingCnvCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingCnvCode {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ExternalIdSystem {
    #[serde(rename = "https://cancer.sanger.ac.uk/cosmic")]
    HttpsCancerSangerAcUkCosmic,

    #[serde(rename = "https://www.ensembl.org")]
    HttpsWwwEnsemblOrg,

    #[serde(rename = "https://www.ncbi.nlm.nih.gov/entrez")]
    HttpsWwwNcbiNlmNihGovEntrez,

    #[serde(rename = "https://www.ncbi.nlm.nih.gov/snp")]
    HttpsWwwNcbiNlmNihGovSnp,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingBaseVariantLocalization {
    pub code: CodingBaseVariantLocalizationCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingBaseVariantLocalizationCode {
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
    pub localization: Option<Vec<CodingBaseVariantLocalization>>,

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
    pub interpretation: Option<CodingHrdScoreInterpretation>,

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
pub struct CodingHrdScoreInterpretation {
    pub code: CodingHrdScoreInterpretationCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum CodingHrdScoreInterpretationCode {
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
    pub localization: Option<Vec<CodingBaseVariantLocalization>>,

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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TranscriptIdSystem {
    #[serde(rename = "https://www.ensembl.org")]
    HttpsWwwEnsemblOrg,

    #[serde(rename = "https://www.ncbi.nlm.nih.gov/refseq")]
    HttpsWwwNcbiNlmNihGovRefseq,
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
    pub localization: Option<Vec<CodingBaseVariantLocalization>>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dna_change: Option<Coding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exon_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<VariantExternalId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene: Option<Coding>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretation: Option<CodingClinVar>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization: Option<Vec<CodingBaseVariantLocalization>>,

    pub patient: Reference,

    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protein_change: Option<Coding>,

    pub read_depth: i64,

    pub ref_allele: String,

    pub transcript_id: TranscriptId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingClinVar {
    pub code: CodingClinVarCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CodingClinVarCode {
    #[serde(rename = "1")]
    The1,

    #[serde(rename = "2")]
    The2,

    #[serde(rename = "3")]
    The3,

    #[serde(rename = "4")]
    The4,

    #[serde(rename = "5")]
    The5,
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
    pub interpretation: Option<CodingTmbInterpretation>,

    pub patient: Reference,

    pub specimen: Reference,

    pub value: TmbResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTmbInterpretation {
    pub code: CodingHrdScoreInterpretationCode,

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
pub struct CodingNgsReport {
    pub code: CodingNgsReportCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingNgsReportCode {
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
    pub address: Address,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<Age>,

    pub birth_date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<String>,

    pub gender: CodingGender,

    pub health_insurance: HealthInsurance,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_site: Option<Coding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vital_status: Option<CodingVitalStatus>,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Unit {
    Months,

    Years,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingGender {
    pub code: GenderCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum GenderCode {
    Female,

    Male,

    Other,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthInsurance {
    #[serde(rename = "type")]
    pub health_insurance_type: CodingHealthInsurance,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Reference>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingHealthInsurance {
    pub code: CodingHealthInsuranceCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CodingHealthInsuranceCode {
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

    #[serde(rename = "SOZ")]
    Soz,

    #[serde(rename = "UNK")]
    Unk,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingVitalStatus {
    pub code: VitalStatusCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum VitalStatusCode {
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

    pub value: CodingEcog,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingEcog {
    pub code: CodingEcogCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CodingEcogCode {
    #[serde(rename = "0")]
    The0,

    #[serde(rename = "1")]
    The1,

    #[serde(rename = "2")]
    The2,

    #[serde(rename = "3")]
    The3,

    #[serde(rename = "4")]
    The4,

    #[serde(rename = "5")]
    The5,
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
    pub prior_diagnostic_report_type: CodingMolecularDiagnosticReport,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,

    pub specimen: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingMolecularDiagnosticReport {
    pub code: CodingMolecularDiagnosticReportCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingMolecularDiagnosticReportCode {
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

    pub method: CodingResponseMethod,

    pub patient: Reference,

    pub therapy: Reference,

    pub value: CodingRecist,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingResponseMethod {
    pub code: CodingResponseMethodCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CodingResponseMethodCode {
    #[serde(rename = "RANO")]
    Rano,

    #[serde(rename = "RECIST")]
    Recist,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingRecist {
    pub code: CodingRecistCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum CodingRecistCode {
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
    pub tumor_specimen_type: CodingTumorSpecimen,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Collection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    pub localization: CodingTumorSpecimenCollectionLocalization,

    pub method: CodingTumorSpecimenCollectionMethod,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTumorSpecimenCollectionLocalization {
    pub code: CodingTumorSpecimenCollectionLocalizationCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingTumorSpecimenCollectionLocalizationCode {
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
pub struct CodingTumorSpecimenCollectionMethod {
    pub code: CodingTumorSpecimenCollectionMethodCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingTumorSpecimenCollectionMethodCode {
    Biopsy,

    Cytology,

    #[serde(rename = "liquid-biopsy")]
    LiquidBiopsy,

    Resection,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTumorSpecimen {
    pub code: CodingTumorSpecimenCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CodingTumorSpecimenCode {
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
