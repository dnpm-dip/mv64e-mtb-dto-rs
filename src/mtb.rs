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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses: Option<Vec<MtbDiagnosis>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<MtbEpisode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes_of_care: Option<Vec<EpisodeOfCare>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub genetic_counselling_requests: Option<Vec<GeneticCounselingRecommendation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guideline_procedures: Option<Vec<OncoProdecure>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guideline_therapies: Option<Vec<MtbMedicationTherapy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histology_reports: Option<Vec<HistologyReport>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ihc_reports: Option<Vec<IhcReport>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_therapies: Option<Vec<MolecularTherapy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ngs_reports: Option<Vec<SomaticNgsReport>>,

    pub patient: MtbPatient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_status: Option<Vec<PerformanceStatus>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<MtbMedicationRecommendation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<Response>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub specimens: Option<Vec<TumorSpecimen>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub study_inclusion_requests: Option<Vec<StudyEnrollmentRecommendation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapies: Option<Vec<Therapy>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MtbCarePlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub genetic_counseling_recommendation: Option<GeneticCounselingRecommendation>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_recommendations: Option<Vec<MtbMedicationRecommendation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_target_finding: Option<NoTargetFinding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<CodingCarePlanStatusReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub study_enrollment_recommendations: Option<Vec<StudyEnrollmentRecommendation>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct GeneticCounselingRecommendation {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub patient: Patient,

    pub reason: Coding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Patient {
    pub id: String,

    #[serde(rename = "type")]
    pub patient_type: PatientType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum PatientType {
    Patient,
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
pub struct Reference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub id: String,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MtbMedicationRecommendation {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_evidence: Option<LevelOfEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ngs_report: Option<String>,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<CodingTherapyRecommendationPriority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_variants: Option<Vec<Reference>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelOfEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addendums: Option<Vec<CodingLevelOfEvidenceAddendum>>,

    pub grading: CodingLevelOfEvidenceGrading,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<ReferencePublication>>,
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
    pub code: GradingCode,

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
pub enum GradingCode {
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
#[serde(rename_all = "camelCase")]
pub struct ReferencePublication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<ExtId>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_publication_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<ExtIdSystem>,

    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ExtIdSystem {
    #[serde(rename = "https://pubmed.ncbi.nlm.nih.gov/")]
    HttpsPubmedNcbiNlmNihGov,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTherapyRecommendationPriority {
    pub code: TherapyRecommendationPriority,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TherapyRecommendationPriority {
    #[serde(rename = "1")]
    Prio1,

    #[serde(rename = "2")]
    Prio2,

    #[serde(rename = "3")]
    Prio3,

    #[serde(rename = "4")]
    Prio4,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct NoTargetFinding {
    pub diagnosis: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub patient: Patient,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingCarePlanStatusReason {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StudyEnrollmentRecommendation {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_evidence: Option<Coding>,

    pub patient: Patient,

    pub reason: Reference,

    pub studies: Vec<Study>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_variants: Option<Vec<Reference>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Study {
    pub system: String,

    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse {
    pub claim: ClaimResponseClaim,

    pub id: String,

    pub issued_on: String,

    pub patient: Patient,

    pub status: CodingClaimResponseStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<CodingClaimResponseStatusReason>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimResponseClaim {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_response_claim_type: Option<ClaimResponseClaimType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ClaimResponseClaimType {
    Claim,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingClaimResponseStatus {
    pub code: ClaimResponseStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum ClaimResponseStatus {
    Accepted,

    Rejected,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingClaimResponseStatusReason {
    pub code: ClaimResponseStatusReason,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ClaimResponseStatusReason {
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

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<Recommendation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<Coding>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recommendation {
    pub id: String,

    #[serde(rename = "type")]
    pub recommendation_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MtbDiagnosis {
    pub code: Coding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guideline_treatment_status: Option<Coding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histology_results: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd_o3_t: Option<Coding>,

    pub id: String,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_history: Option<Vec<StageHistory>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topography: Option<Coding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_grade: Option<CodingTumorGrade>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_grade: Option<Coding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_grading: Option<Coding>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageHistory {
    pub date: String,

    pub stage: CodingTumorSpread,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTumorSpread {
    pub code: StageCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum StageCode {
    Local,

    Metastasized,

    #[serde(rename = "tumor-free")]
    TumorFree,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTumorGrade {
    pub code: TumorGradeCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TumorGradeCode {
    G1,

    G2,

    G3,

    G4,

    #[serde(rename = "GX")]
    Gx,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MtbEpisode {
    pub id: String,

    pub patient: Patient,

    pub period: PeriodLocalDate,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodLocalDate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    pub start: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EpisodeOfCare {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses: Option<Vec<Reference>>,

    pub id: String,

    pub patient: Reference,

    pub period: PeriodLocalDate,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct OncoProdecure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub based_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Coding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<String>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PeriodLocalDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CodingTherapyStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<CodingTherapyStatusReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapy_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTherapyStatus {
    pub code: TherapyStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum TherapyStatus {
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
pub struct CodingTherapyStatusReason {
    pub code: StatusReasonCode,

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
pub enum StatusReasonCode {
    #[serde(rename = "chronic-remission")]
    ChronicRemission,

    #[serde(rename = "continued-externally")]
    ContinuedExternally,

    Deterioration,

    #[serde(rename = "lost-to-fu")]
    LostToFu,

    #[serde(rename = "medical-reason")]
    MedicalReason,

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

    Toxicity,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MtbMedicationTherapy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub based_on: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<String>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PeriodLocalDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CodingTherapyStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<CodingTherapyStatusReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapy_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct HistologyReport {
    pub id: String,

    pub issued_on: String,

    pub patient: Patient,

    pub results: HistologyReportResults,

    pub specimen: HistologyReportSpecimen,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct HistologyReportResults {
    pub tumor_cell_content: TumorCellContent,

    pub tumor_morphology: TumorMorphology,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorCellContent {
    pub id: String,

    pub method: CodingTumorCellContentMethod,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient: Option<Patient>,

    pub specimen: TumorCellContentSpecimen,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTumorCellContentMethod {
    pub code: TumorCellContentMethod,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum TumorCellContentMethod {
    Bioinformatic,

    Histologic,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorCellContentSpecimen {
    pub id: String,

    #[serde(rename = "type")]
    pub specimen_type: SpecimenType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SpecimenType {
    #[serde(rename = "TumorSpecimen")]
    TumorSpecimen,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorMorphology {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    pub patient: Patient,

    pub specimen: TumorMorphologySpecimen,

    pub value: Coding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorMorphologySpecimen {
    pub id: String,

    #[serde(rename = "type")]
    pub specimen_type: SpecimenType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistologyReportSpecimen {
    pub id: String,

    #[serde(rename = "type")]
    pub specimen_type: SpecimenType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct IhcReport {
    pub block_id: ExternalId,

    pub date: String,

    pub id: String,

    pub journal_id: ExternalId,

    pub msi_mmr_results: Vec<MsiMmrResult>,

    pub patient: Reference,

    pub protein_expression_results: Vec<ProteinExpressionResult>,

    pub specimen: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MsiMmrResult {
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
pub struct ProteinExpressionResult {
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
pub struct MolecularTherapy {
    pub history: Vec<MtbMedicationTherapy>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct SomaticNgsReport {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub metadata: Vec<Metadatum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub msi: Option<f64>,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<NgsReportResults>,

    pub sequencing_type: Coding,

    pub specimen: NgsReportSpecimen,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Metadatum {
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
    pub chromosome: CodingChromosome,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_a: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_b: Option<f64>,

    #[serde(rename = "type")]
    pub cnv_type: CodingCnvType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_number_neutral_lo_h: Option<Vec<CodingGene>>,

    pub end_range: EndRange,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient: Option<Patient>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_copy_number: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_affected_genes: Option<Vec<CodingGene>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_focality: Option<String>,

    pub start_range: StartRange,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_copy_number: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingChromosome {
    pub code: ChromosomeCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<ChromosomeSystem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum ChromosomeCode {
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
#[serde(rename_all = "snake_case")]
pub enum ChromosomeSystem {
    Chromosome,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingCnvType {
    pub code: CnvType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum CnvType {
    #[serde(rename = "high-level-gain")]
    HighLevelGain,

    Loss,

    #[serde(rename = "low-level-gain")]
    LowLevelGain,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingGene {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<GeneSystem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum GeneSystem {
    #[serde(rename = "https://www.genenames.org/")]
    HttpsWwwGenenamesOrg,
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
pub struct StartRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct DnaFusion {
    #[serde(rename = "fusionPartner3prime")]
    pub fusion_partner3_prime: DnaFusionFusionPartner3Prime,

    #[serde(rename = "fusionPartner5prime")]
    pub fusion_partner5_prime: DnaFusionFusionPartner5Prime,

    pub id: String,

    pub reported_num_reads: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnaFusionFusionPartner3Prime {
    pub chromosome: CodingChromosome,

    pub gene: CodingGene,

    pub position: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnaFusionFusionPartner5Prime {
    pub chromosome: CodingChromosome,

    pub gene: Gene,

    pub position: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Gene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HrdScore {
    pub components: Components,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretation: Option<Coding>,

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
#[serde(rename_all = "camelCase")]
pub struct RnaFusion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cosmic_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,

    #[serde(rename = "fusionPartner3prime")]
    pub fusion_partner3_prime: RnaFusionFusionPartner3Prime,

    #[serde(rename = "fusionPartner5prime")]
    pub fusion_partner5_prime: RnaFusionFusionPartner5Prime,

    pub id: String,

    pub reported_num_reads: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusionFusionPartner3Prime {
    pub exon: String,

    pub gene: CodingGene,

    pub position: f64,

    pub strand: RnaFusionStrand,

    pub transcript_id: String,
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
#[serde(rename_all = "camelCase")]
pub struct RnaFusionFusionPartner5Prime {
    pub exon: String,

    pub gene: CodingGene,

    pub position: f64,

    pub strand: RnaFusionStrand,

    pub transcript_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct RnaSeq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_ranking: Option<i64>,

    pub ensembl_id: String,

    pub entrez_id: String,

    pub fragments_per_kilobase_million: f64,

    #[serde(rename = "fromNGS")]
    pub from_ngs: bool,

    pub gene: CodingGene,

    pub id: String,

    pub library_size: i64,

    pub raw_counts: i64,

    pub tissue_corrected_expression: bool,

    pub transcript_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Snv {
    pub allelic_frequency: f64,

    pub alt_allele: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amino_acid_change: Option<Coding>,

    pub chromosome: CodingChromosome,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cosmic_id: Option<String>,

    #[serde(rename = "dbSNPId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snp_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dna_change: Option<Coding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<ExternalId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene: Option<CodingGene>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretation: Option<Coding>,

    pub patient: Patient,

    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protein_change: Option<Coding>,

    pub read_depth: i64,

    pub ref_allele: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_id: Option<ExternalId>,
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
    pub interpretation: Option<Coding>,

    pub patient: Reference,

    pub specimen: Reference,

    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Value {
    pub unit: String,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NgsReportSpecimen {
    pub id: String,

    #[serde(rename = "type")]
    pub specimen_type: SpecimenType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MtbPatient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<ValueWithUnit>,

    pub birth_date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<String>,

    pub gender: CodingGender,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_insurance: Option<HealthInsurance>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vital_status: Option<VitalStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub municipality_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValueWithUnit {
    pub unit: Unit,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Unit {
    Years,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingGender {
    pub code: Gender,

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
pub enum Gender {
    Female,

    Male,

    Other,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct HealthInsurance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub ext_id: ExternalId,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_insurance_type: Option<Type>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Type {
    Organization,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VitalStatus {
    pub code: VitalStatusCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
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

    pub patient: Patient,

    pub value: CodingEcog,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingEcog {
    pub code: EcogCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum EcogCode {
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub effective_date: String,

    pub id: String,

    pub patient: Patient,

    pub therapy: ResponseTherapy,

    pub value: CodingRecist,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseTherapy {
    pub id: String,

    #[serde(rename = "type")]
    pub response_therapy_type: ResponseTherapyType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ResponseTherapyType {
    #[serde(rename = "MTBMedicationTherapy")]
    MtbMedicationTherapy,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingRecist {
    pub code: RecistCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum RecistCode {
    #[serde(rename = "CR")]
    Cr,

    #[serde(rename = "MR")]
    Mr,

    #[serde(rename = "NA")]
    Na,

    #[serde(rename = "NYA")]
    Nya,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<Diagnosis>,

    pub id: String,

    pub patient: Patient,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_specimen_type: Option<CodingTumorSpecimenType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Collection {
    pub date: String,

    pub localization: CodingTumorSpecimenCollectionLocalization,

    pub method: CodingTumorSpecimenCollectionMethod,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTumorSpecimenCollectionLocalization {
    pub code: TumorSpecimenCollectionLocalization,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum TumorSpecimenCollectionLocalization {
    Metastasis,

    #[serde(rename = "primary-tumor")]
    PrimaryTumor,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTumorSpecimenCollectionMethod {
    pub code: TumorSpecimenCollectionMethod,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum TumorSpecimenCollectionMethod {
    Biopsy,

    Cytology,

    #[serde(rename = "liquid-biopsy")]
    LiquidBiopsy,

    Resection,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Diagnosis {
    #[serde(rename = "type")]
    pub diagnosis_type: String,

    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodingTumorSpecimenType {
    pub code: TumorSpecimenType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum TumorSpecimenType {
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
pub struct Therapy {
    pub history: Vec<MtbMedicationTherapy>,
}
