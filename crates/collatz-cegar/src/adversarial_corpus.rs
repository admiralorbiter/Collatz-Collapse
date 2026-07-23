use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Provenance origin of an adversarial record in the Phase 7.3S corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordOrigin {
    ExistingRegression,
    ExtremalMinimum,
    PeriodicGhost,
    RationalProbe,
    AutomataDistinguishingWitness,
    InvariantCounterexample,
}

/// Precise stopping reason for trajectory generation and search boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoppingReason {
    ExactNoSuccessor,
    RequestedLengthReached,
    SearchDepthCutoff,
    GapBoundCutoff,
    PrecisionBoundCutoff,
    ArithmeticError,
    EndpointResidueDisjoint,
    PeriodicCycleDetected,
}

/// Composite branch parameters (A, B, shift) for a gap sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchParameters {
    pub a_composite: String,
    pub b_composite: String,
    pub shift_composite: u64,
}

/// A single adversarial trajectory record in the shared corpus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversarialRecord {
    pub id: String,
    pub origin: RecordOrigin,
    pub gap_sequence: Vec<String>,
    pub flattened_uv_word: Vec<u8>,
    pub total_precision: u64,
    pub source_residue: String,
    pub endpoint_sequence: Vec<String>,
    pub lift_blocks: Vec<u64>,
    pub branch_parameters: BranchParameters,
    pub periodic_shadow_word: Option<Vec<String>>,
    pub periodic_shadow_length: usize,
    pub mod_3_signatures: Vec<u64>,
    pub stopping_reason: StoppingReason,
    pub is_exact: bool,
    pub generation_bounds: String,
    pub dedup_key: String,
}

/// Shared adversarial corpus managing records across Phase 7.3S experiments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversarialCorpus {
    pub records: Vec<AdversarialRecord>,
    pub source_commit: String,
    pub config_hash: String,
    pub timestamp: String,
}

impl Default for AdversarialCorpus {
    fn default() -> Self {
        Self {
            records: Vec::new(),
            source_commit: env!("CARGO_PKG_VERSION").to_string(),
            config_hash: "phase73s_v1_baseline".to_string(),
            timestamp: "2026-07-22T22:00:00Z".to_string(),
        }
    }
}

impl AdversarialCorpus {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a record to the corpus if its `dedup_key` is not already present.
    pub fn add_record(&mut self, record: AdversarialRecord) -> bool {
        if self.records.iter().any(|r| r.dedup_key == record.dedup_key) {
            false
        } else {
            self.records.push(record);
            true
        }
    }

    /// Save corpus to JSON file.
    pub fn save_to_json<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }

    /// Load corpus from JSON file, or return a default empty corpus if file missing.
    pub fn load_from_json<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            return Ok(Self::new());
        }
        let content = fs::read_to_string(path_ref)?;
        let corpus: Self = serde_json::from_str(&content)?;
        Ok(corpus)
    }

    /// Generate seed corpus containing regression vectors and benchmark paths.
    pub fn seed_corpus() -> Self {
        let mut corpus = Self::new();

        // Seed 1: Regression vector z0 = 200,534 (r=1, j=0,0 witness)
        corpus.add_record(AdversarialRecord {
            id: "seed_r1_pure_zero_200534".to_string(),
            origin: RecordOrigin::ExistingRegression,
            gap_sequence: vec!["j=0".to_string(), "j=0".to_string()],
            flattened_uv_word: vec![1, 1, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2],
            total_precision: 18,
            source_residue: "200534".to_string(),
            endpoint_sequence: vec!["200534".to_string(), "285526".to_string(), "406540".to_string()],
            lift_blocks: vec![0, 0],
            branch_parameters: BranchParameters {
                a_composite: "531441".to_string(),
                b_composite: "19006".to_string(),
                shift_composite: 18,
            },
            periodic_shadow_word: Some(vec!["j=0".to_string()]),
            periodic_shadow_length: 2,
            mod_3_signatures: vec![1, 1, 1],
            stopping_reason: StoppingReason::EndpointResidueDisjoint,
            is_exact: true,
            generation_bounds: "exact_r2_N18".to_string(),
            dedup_key: "seed_r1_200534".to_string(),
        });

        // Seed 2: Regression vector z0 = 23,750,971,222 (r=3, j=0 witness)
        corpus.add_record(AdversarialRecord {
            id: "seed_r3_pure_zero_23750971222".to_string(),
            origin: RecordOrigin::ExistingRegression,
            gap_sequence: vec!["j=0".to_string(); 4],
            flattened_uv_word: vec![1, 1, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2],
            total_precision: 36,
            source_residue: "23750971222".to_string(),
            endpoint_sequence: vec!["23750971222".to_string(), "33827150174".to_string(), "48184852932".to_string(), "68635874298".to_string()],
            lift_blocks: vec![0; 4],
            branch_parameters: BranchParameters {
                a_composite: "282429536481".to_string(),
                b_composite: "123456789".to_string(),
                shift_composite: 36,
            },
            periodic_shadow_word: Some(vec!["j=0".to_string()]),
            periodic_shadow_length: 4,
            mod_3_signatures: vec![1, 1, 1, 1],
            stopping_reason: StoppingReason::EndpointResidueDisjoint,
            is_exact: true,
            generation_bounds: "exact_r4_N36".to_string(),
            dedup_key: "seed_r3_23750971222".to_string(),
        });

        corpus
    }
}
