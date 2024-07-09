use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mtb {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub care_plans: Option<Vec<MtbCarePlan>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_responses: Option<Vec<ClaimResponse>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<Claim>>,

    pub consent: Consent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses: Option<Vec<MtbDiagnosis>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecog_status: Option<Vec<PerformanceStatus>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<MtbEpisode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub genetic_counselling_requests: Option<Vec<GeneticCounselingRecommendation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histology_reports: Option<Vec<HistologyReport>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_guideline_therapies: Option<Vec<MtbMedicationTherapy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_therapies: Option<Vec<MolecularTherapy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ngs_reports: Option<Vec<SomaticNgsReport>>,

    pub patient: MtbPatient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_guideline_therapies: Option<Vec<MtbMedicationTherapy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<MtbMedicationRecommendation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<Response>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub specimens: Option<Vec<TumorSpecimen>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub study_inclusion_requests: Option<Vec<StudyEnrollmentRecommendation>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MtbCarePlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub genetic_counselling_request: Option<String>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_target_finding: Option<NoTargetFinding>,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub study_inclusion_requests: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoTargetFinding {
    pub diagnosis: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub patient: Patient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Patient {
    pub id: String,

    #[serde(rename = "type")]
    pub patient_type: PatientType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatientType {
    Patient,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse {
    pub claim: ClaimResponseClaim,

    pub id: String,

    pub issued_on: String,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<ClaimResponseStatusReason>,

    pub status: CodingClaimResponseStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimResponseClaim {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_response_claim_type: Option<ClaimResponseClaimType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClaimResponseClaimType {
    Claim,
}

#[derive(Debug, Serialize, Deserialize)]
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
pub struct CodingClaimResponseStatus {
    pub code: ClaimResponseStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimResponseStatus {
    Accepted,

    Rejected,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub id: String,

    pub issued_on: String,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient: Option<Patient>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ConsentStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentStatus {
    Active,

    Rejected,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub status_history: Option<Vec<StatusHistory>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_grade: Option<Coding>,
}

#[derive(Debug, Serialize, Deserialize)]
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
pub struct StatusHistory {
    pub date: String,

    pub status: MtbDiagnosisTumorSpread,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MtbDiagnosisTumorSpread {
    Local,

    Metastasized,

    #[serde(rename = "tumor-free")]
    TumorFree,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceStatus {
    pub effective_date: String,

    pub id: String,

    pub patient: Patient,

    pub value: CodingEcog,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodingEcog {
    pub code: PurpleCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PurpleCode {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MtbEpisode {
    pub id: String,

    pub patient: Patient,

    pub period: PeriodLocalDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodLocalDate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    pub start: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneticCounselingRecommendation {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub patient: Patient,

    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistologyReport {
    pub id: String,

    pub issued_on: String,

    pub patient: Patient,

    pub specimen: HistologyReportSpecimen,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_cell_content: Option<TumorCellContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_morphology: Option<TumorMorphology>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistologyReportSpecimen {
    pub id: String,

    #[serde(rename = "type")]
    pub specimen_type: SpecimenType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecimenType {
    #[serde(rename = "TumorSpecimen")]
    TumorSpecimen,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TumorCellContent {
    pub id: String,

    pub method: TumorCellContentMethod,

    pub specimen: String,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TumorCellContentMethod {
    Bioinformatic,

    Histologic,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TumorMorphology {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    pub patient: Patient,

    pub specimen: String,

    pub value: Coding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MtbMedicationTherapy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub based_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<String>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<Coding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_done_reason: Option<CodingTherapyStatusReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PeriodLocalDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_stopped: Option<CodingTherapyStatusReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TherapyStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapy_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodingTherapyStatusReason {
    pub code: NotDoneReasonCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NotDoneReasonCode {
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
pub struct MolecularTherapy {
    pub history: Vec<MtbMedicationTherapy>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SomaticNgsReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brcaness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_number_variants: Option<Vec<Cnv>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dna_fusions: Option<Vec<DnaFusion>>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,

    pub metadata: Vec<Metadatum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub msi: Option<f64>,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rna_fusions: Option<Vec<RnaFusion>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rna_seqs: Option<Vec<RnaSeq>>,

    pub sequencing_type: Coding,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_variants: Option<Vec<Snv>>,

    pub specimen: NgsReportSpecimen,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmb: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_cell_content: Option<TumorCellContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cnv {
    pub chromosome: Chromosome,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_a: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_b: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_number_neutral_lo_h: Option<Vec<CopyNumberNeutralLoH>>,

    pub end_range: EndRange,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_copy_number: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_affected_genes: Option<Vec<ReportedAffectedGene>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_focality: Option<String>,

    pub start_range: StartRange,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_copy_number: Option<i64>,

    #[serde(rename = "type")]
    pub cnv_type: CnvType,
}

#[derive(Debug, Serialize, Deserialize)]
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
#[serde(rename_all = "kebab-case")]
pub enum CnvType {
    #[serde(rename = "high-level-gain")]
    HighLevelGain,

    Loss,

    #[serde(rename = "low-level-gain")]
    LowLevelGain,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyNumberNeutralLoH {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportedAffectedGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Debug, Serialize, Deserialize)]
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
pub struct DnaFusionFusionPartner3Prime {
    pub chromosome: Chromosome,

    pub gene: PurpleGene,

    pub position: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnaFusionFusionPartner5Prime {
    pub chromosome: Chromosome,

    pub gene: FluffyGene,

    pub position: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadatum {
    pub kit_manufacturer: String,

    pub kit_type: String,

    pub pipeline: String,

    pub reference_genome: String,

    pub sequencer: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
#[serde(rename_all = "camelCase")]
pub struct RnaFusionFusionPartner3Prime {
    pub exon: String,

    pub gene: TentacledGene,

    pub position: f64,

    pub strand: RnaFusionStrand,

    pub transcript_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RnaFusionStrand {
    #[serde(rename = "+")]
    Empty,

    #[serde(rename = "-")]
    RnaFusionStrand,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusionFusionPartner5Prime {
    pub exon: String,

    pub gene: StickyGene,

    pub position: f64,

    pub strand: RnaFusionStrand,

    pub transcript_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RnaSeq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_ranking: Option<i64>,

    pub ensembl_id: String,

    pub entrez_id: String,

    pub fragments_per_kilobase_million: f64,

    #[serde(rename = "fromNGS")]
    pub from_ngs: bool,

    pub gene: RnaSeqGene,

    pub id: String,

    pub library_size: i64,

    pub raw_counts: i64,

    pub tissue_corrected_expression: bool,

    pub transcript_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RnaSeqGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snv {
    pub allelic_frequency: f64,

    pub alt_allele: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amino_acid_change: Option<Coding>,

    pub chromosome: Chromosome,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cosmic_id: Option<String>,

    #[serde(rename = "dbSNPId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snp_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dna_change: Option<Coding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene: Option<SimpleVariantGene>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretation: Option<Coding>,

    pub read_depth: i64,

    pub ref_allele: String,

    pub start_end: StartEnd,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleVariantGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartEnd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NgsReportSpecimen {
    pub id: String,

    #[serde(rename = "type")]
    pub specimen_type: SpecimenType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MtbPatient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    pub birth_date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<String>,

    pub gender: CodingGender,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub municipality_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Female,

    Male,

    Other,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MtbMedicationRecommendation {
    pub diagnosis: String,

    pub id: String,

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
    pub priority: Option<TherapyRecommendationPriority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_variants: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelOfEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addendums: Option<Vec<CodingLevelOfEvidenceAddendum>>,

    pub grading: CodingLevelOfEvidenceGrading,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<ReferencePublication>>,
}

#[derive(Debug, Serialize, Deserialize)]
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
pub enum AddendumCode {
    #[serde(rename = "is")]
    Is,

    #[serde(rename = "iv")]
    Iv,

    R,

    Z,
}

#[derive(Debug, Serialize, Deserialize)]
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
pub struct ExtId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<System>,

    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum System {
    #[serde(rename = "https://pubmed.ncbi.nlm.nih.gov/")]
    HttpsPubmedNcbiNlmNihGov,
}

#[derive(Debug, Serialize, Deserialize)]
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
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub effective_date: String,

    pub id: String,

    pub patient: Patient,

    pub therapy: ResponseTherapy,

    pub value: CodingRecist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTherapy {
    pub id: String,

    #[serde(rename = "type")]
    pub response_therapy_type: ResponseTherapyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseTherapyType {
    #[serde(rename = "MTBMedicationTherapy")]
    MtbMedicationTherapy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodingRecist {
    pub code: FluffyCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FluffyCode {
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
pub struct TumorSpecimen {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd10: Option<Coding>,

    pub id: String,

    pub patient: Patient,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_specimen_type: Option<CodingTumorSpecimenType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub date: String,

    pub localization: CodingTumorSpecimenCollectionLocalization,

    pub method: CodingTumorSpecimenCollectionMethod,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodingTumorSpecimenCollectionLocalization {
    pub code: TumorSpecimenCollectionLocalization,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TumorSpecimenCollectionLocalization {
    Metastasis,

    #[serde(rename = "primary-tumor")]
    PrimaryTumor,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodingTumorSpecimenCollectionMethod {
    pub code: TumorSpecimenCollectionMethod,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
pub struct CodingTumorSpecimenType {
    pub code: TumorSpecimenType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
#[serde(rename_all = "camelCase")]
pub struct StudyEnrollmentRecommendation {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub nct_number: String,

    pub patient: Patient,

    pub reason: String,
}
