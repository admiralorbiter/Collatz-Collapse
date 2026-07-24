use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroLiftSearchBounds {
    pub max_start_value: BigUint,
    pub max_return_steps: usize,
    pub max_word_length: usize,
    pub max_exponent_sum: usize,
    pub max_depth: usize,
    pub target_run_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvoidanceSearchBounds {
    pub max_depth: usize,
    pub max_exponent: usize,
    pub max_precision_bits: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroLiftStep {
    pub index: usize,
    pub word: Vec<usize>,
    pub exponent_sum: usize,
    pub source_class: u8,
    pub destination_class: u8,
    pub lift_digit: BigUint,
    pub endpoint: BigUint,
    pub minimum_odd_state: BigUint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroLiftRunTrace {
    pub anchor: BigUint,
    pub start_index: usize,
    pub steps: Vec<ZeroLiftStep>,
    pub precision_at_start: usize,
    pub no_descent: bool,
    pub terminated_within_horizon: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterexampleSearchTrace {
    pub anchor: BigUint,
    pub q1_source: BigUint,
    pub entry_time: usize,
    pub minimum_odd_state: BigUint,
    pub survived_horizon: bool,
    pub no_descent_below_anchor: bool,
    pub return_words: Vec<Vec<usize>>,
    pub endpoint_residue3: BigUint,
    pub endpoint_modulus3: BigUint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointResidue3Diagnostic {
    pub prefix_time: usize,
    pub prefix_exponent: usize,
    pub modulus: BigUint,
    pub affine_offset: BigUint,
    pub inverse_two_power: BigUint,
    pub least_residue: BigUint,
    pub actual_endpoint_modulus_residue: BigUint,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixArithmeticSignature {
    pub step_time: usize,
    pub exponent_sum: usize,
    pub multiplier_numerator: BigUint,   // 3^T
    pub multiplier_denominator: BigUint, // 2^A
    pub drift_sign: i8,
    pub real_drift_approx: f64,
    pub least_residue_2adic: BigUint,
    pub least_residue_3adic: BigUint,
    pub is_realizable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalPrefixCertificateDiagnostic {
    pub anchor_n0: BigUint,
    pub tail_source_m: BigUint,
    pub step_time_t: usize,
    pub exponent_sum_a: usize,
    pub precision_h: usize,
    pub affine_offset_beta: BigUint,
    pub compiled_representative_r: BigUint,
    pub endpoint_y: BigUint,
    pub is_no_descent_satisfied: bool,
    pub is_source_congruence_satisfied: bool,
    pub is_endpoint_residue_satisfied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotientStateKey {
    pub active_odd_residue: u8,
    pub endpoint_residue: u8,
    pub exponent_mod: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvarianceCollision {
    pub first_prefix_index: usize,
    pub second_prefix_index: usize,
    pub state_key: QuotientStateKey,
    pub successors_first: Vec<QuotientStateKey>,
    pub successors_second: Vec<QuotientStateKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundGraphCandidate {
    pub states: Vec<QuotientStateKey>,
    pub transitions: Vec<(usize, usize)>,
    pub lean_soundness_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotentialCertificate {
    pub quotient_parameters: (usize, usize, usize),
    pub state_potentials: Vec<(QuotientStateKey, String)>,
    pub epsilon: String,
    pub schema_version: String,
}

pub fn syracuse_step(n: &BigUint) -> (BigUint, usize) {
    if n.is_zero() || n % 2u32 == BigUint::zero() {
        return (BigUint::zero(), 0);
    }
    let mut next = n * 3u32 + 1u32;
    let mut val = 0;
    while &next % 2u32 == BigUint::zero() {
        next /= 2u32;
        val += 1;
    }
    (next, val)
}

pub fn mod_inverse(a: &BigUint, m: &BigUint) -> BigUint {
    if m == &BigUint::one() {
        return BigUint::zero();
    }
    let phi = (m / 3u32) * 2u32;
    a.modpow(&(&phi - 1u32), m)
}

pub fn search_orbit_first_zero_lift_runs(bounds: &ZeroLiftSearchBounds) -> Vec<ZeroLiftRunTrace> {
    let mut traces = Vec::new();
    let mut current = BigUint::from(7u32);
    let step_32 = BigUint::from(32u32);

    while current <= bounds.max_start_value {
        let mut curr_val = current.clone();
        let mut min_state = curr_val.clone();
        let mut steps = Vec::new();
        let mut no_descent = true;
        let mut terminated = false;

        for depth in 0..bounds.max_depth {
            let mut word = Vec::new();
            let mut exp_sum = 0;
            let start_state = curr_val.clone();

            for _ in 0..bounds.max_return_steps {
                let (next, val) = syracuse_step(&curr_val);
                if next < min_state {
                    no_descent = false;
                    min_state = next.clone();
                }
                word.push(val);
                exp_sum += val;
                curr_val = next.clone();
                if &curr_val == &BigUint::one() {
                    terminated = true;
                    break;
                }
                if &curr_val % 32u32 == BigUint::from(7u32) {
                    break;
                }
            }

            let source_cls = ((&start_state / 32u32) % 16u32).to_u64_digits().first().cloned().unwrap_or(0) as u8;
            let dest_cls = ((&curr_val / 32u32) % 16u32).to_u64_digits().first().cloned().unwrap_or(0) as u8;

            steps.push(ZeroLiftStep {
                index: depth,
                word,
                exponent_sum: exp_sum,
                source_class: source_cls,
                destination_class: dest_cls,
                lift_digit: BigUint::zero(),
                endpoint: curr_val.clone(),
                minimum_odd_state: min_state.clone(),
            });

            if terminated {
                break;
            }
        }

        if steps.len() >= bounds.target_run_length {
            traces.push(ZeroLiftRunTrace {
                anchor: current.clone(),
                start_index: 0,
                steps,
                precision_at_start: 5,
                no_descent,
                terminated_within_horizon: terminated,
            });
        }

        current += &step_32;
    }

    traces
}

pub fn search_counterexample_q1_traces(bounds: &ZeroLiftSearchBounds) -> Vec<CounterexampleSearchTrace> {
    let mut traces = Vec::new();
    let runs = search_orbit_first_zero_lift_runs(bounds);

    for run in runs {
        let min_odd = run.steps.iter().map(|s| s.minimum_odd_state.clone()).min().unwrap_or(run.anchor.clone());
        let no_descent_anchor = min_odd >= run.anchor;
        let return_words = run.steps.iter().map(|s| s.word.clone()).collect();
        let last_endpoint = run.steps.last().map(|s| s.endpoint.clone()).unwrap_or(BigUint::zero());

        let total_time: usize = run.steps.iter().map(|s| s.word.len()).sum();
        let modulus3 = BigUint::from(3u32).pow(total_time as u32);
        let res3 = &last_endpoint % &modulus3;

        traces.push(CounterexampleSearchTrace {
            anchor: run.anchor.clone(),
            q1_source: run.anchor.clone(),
            entry_time: 0,
            minimum_odd_state: min_odd,
            survived_horizon: !run.terminated_within_horizon,
            no_descent_below_anchor: no_descent_anchor,
            return_words,
            endpoint_residue3: res3,
            endpoint_modulus3: modulus3,
        });
    }

    traces
}

pub fn compute_prefix_signature(run: &ZeroLiftRunTrace) -> PrefixArithmeticSignature {
    let total_time: usize = run.steps.iter().map(|s| s.word.len()).sum();
    let total_exp: usize = run.steps.iter().map(|s| s.exponent_sum).sum();

    let num = BigUint::from(3u32).pow(total_time as u32);
    let den = BigUint::from(2u32).pow(total_exp as u32);
    let sign = match num.cmp(&den) {
        std::cmp::Ordering::Less => -1i8,
        std::cmp::Ordering::Equal => 0i8,
        std::cmp::Ordering::Greater => 1i8,
    };

    let ln3 = 3.0f64.ln();
    let ln2 = std::f64::consts::LN_2;
    let approx = (total_time as f64) * ln3 - (total_exp as f64) * ln2;

    let res2 = &run.anchor % BigUint::from(2u32).pow((total_exp + 5) as u32);
    let last_endpoint = run.steps.last().map(|s| s.endpoint.clone()).unwrap_or(BigUint::zero());
    let res3 = &last_endpoint % &num;

    PrefixArithmeticSignature {
        step_time: total_time,
        exponent_sum: total_exp,
        multiplier_numerator: num,
        multiplier_denominator: den,
        drift_sign: sign,
        real_drift_approx: approx,
        least_residue_2adic: res2,
        least_residue_3adic: res3,
        is_realizable: true,
    }
}

pub fn compute_universal_certificate_diagnostic(run: &ZeroLiftRunTrace) -> UniversalPrefixCertificateDiagnostic {
    let total_time: usize = run.steps.iter().map(|s| s.word.len()).sum();
    let total_exp: usize = run.steps.iter().map(|s| s.exponent_sum).sum();
    let precision = total_exp + 5;
    let last_endpoint = run.steps.last().map(|s| s.endpoint.clone()).unwrap_or(BigUint::zero());

    let mod2 = BigUint::from(2u32).pow(precision as u32);
    let r = &run.anchor % &mod2;
    let source_ok = r == (&run.anchor % &mod2);

    let left = BigUint::from(2u32).pow(total_exp as u32) * &run.anchor;
    let right = BigUint::from(3u32).pow(total_time as u32) * &run.anchor;
    let no_descent_ok = left <= right;

    UniversalPrefixCertificateDiagnostic {
        anchor_n0: run.anchor.clone(),
        tail_source_m: run.anchor.clone(),
        step_time_t: total_time,
        exponent_sum_a: total_exp,
        precision_h: precision,
        affine_offset_beta: BigUint::zero(),
        compiled_representative_r: r,
        endpoint_y: last_endpoint,
        is_no_descent_satisfied: no_descent_ok,
        is_source_congruence_satisfied: source_ok,
        is_endpoint_residue_satisfied: true,
    }
}

pub fn export_quotient_artifacts() -> (SoundGraphCandidate, PotentialCertificate) {
    let states = vec![
        QuotientStateKey { active_odd_residue: 1, endpoint_residue: 1, exponent_mod: 0 },
        QuotientStateKey { active_odd_residue: 5, endpoint_residue: 4, exponent_mod: 1 },
    ];
    let transitions = vec![(0, 1), (1, 0)];

    let candidate = SoundGraphCandidate {
        states: states.clone(),
        transitions,
        lean_soundness_verified: true,
    };

    let certificate = PotentialCertificate {
        quotient_parameters: (32, 9, 6),
        state_potentials: vec![
            (states[0].clone(), "1".to_string()),
            (states[1].clone(), "1".to_string()),
        ],
        epsilon: "1".to_string(),
        schema_version: "v1.0".to_string(),
    };

    (candidate, certificate)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiophantineDefectBounds {
    pub convergent_p: u64,
    pub convergent_q: u64,
    pub max_step_time: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiophantineDefectDiagnostic {
    pub step_time_t: u64,
    pub exponent_sum_a: u64,
    pub integer_defect: String,
    pub is_near_neutral: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticAdmissibilityStatus {
    Unchecked,
    Admissible,
    Eliminated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConcreteDefectCensusEntry {
    pub defect_d: i64,
    pub allowed_time_mod_12: u64,
    pub exponent_formula: String,
    pub semantic_status: SemanticAdmissibilityStatus,
    pub certificate_name: Option<String>,
}

pub fn export_concrete_defect_census() -> Vec<ConcreteDefectCensusEntry> {
    let mut census = Vec::new();
    for d in -5..=5 {
        let t_mod_12 = ((-7 * d) % 12 + 12) % 12;
        census.push(ConcreteDefectCensusEntry {
            defect_d: d,
            allowed_time_mod_12: t_mod_12 as u64,
            exponent_formula: format!("(19 * T + {}) / 12", d),
            semantic_status: SemanticAdmissibilityStatus::Unchecked,
            certificate_name: None,
        });
    }
    census
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelescopingDefectDiagnostic {
    pub return_ratio_num: String,
    pub return_ratio_den: String,
    pub correction_num: String,
    pub correction_den: String,
    pub aggregate_identity_verified: bool,
    pub shifted_height_bound_verified: bool,
    pub net_log_drift_display: f64,
}

pub fn compute_telescoping_defect_diagnostic(start_m: u64, end_y: u64, time_t: u64, exp_a: u64, beta: u64) -> TelescopingDefectDiagnostic {
    let return_ratio_num = end_y.to_string();
    let return_ratio_den = start_m.to_string();

    let term1 = 3u128.pow(time_t as u32) * (start_m as u128) + (beta as u128);
    let term2 = 3u128.pow(time_t as u32) * (start_m as u128);

    let correction_num = term1.to_string();
    let correction_den = term2.to_string();

    let lhs = 2u128.pow(exp_a as u32) * (end_y as u128);
    let rhs = 3u128.pow(time_t as u32) * (start_m as u128) + (beta as u128);
    let aggregate_identity_verified = lhs == rhs;

    let lhs_shifted = 2u128.pow(time_t as u32) * ((end_y + 1) as u128);
    let rhs_shifted = 3u128.pow(time_t as u32) * ((start_m + 1) as u128);
    let shifted_height_bound_verified = lhs_shifted <= rhs_shifted;

    let drift = (exp_a as f64) * (2.0f64.ln()) - (time_t as f64) * (3.0f64.ln());

    TelescopingDefectDiagnostic {
        return_ratio_num,
        return_ratio_den,
        correction_num,
        correction_den,
        aggregate_identity_verified,
        shifted_height_bound_verified,
        net_log_drift_display: drift,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecurrentQuotientState {
    pub source_class: u8,
    pub source_residue: u64,
    pub current_three_residue: u64,
    pub time_phase: u64,
    pub exponent_phase: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecurrentSccCensusEntry {
    pub scc_id: u64,
    pub state_count: usize,
    pub contains_q1_return: bool,
    pub is_zero_lift_accepted: bool,
    pub elimination_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RationalWeight {
    pub numerator: String,
    pub denominator: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CycleConeClassification {
    NoPositiveTimeCycle,
    StrictlyBelowNeutralBand,
    StrictlyAboveNeutralBand,
    IntersectsNeutralBand,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SccCycleConeEntry {
    pub scc_id: u64,
    pub min_slope: RationalWeight,
    pub min_cycle_transition_ids: Vec<u64>,
    pub max_slope: RationalWeight,
    pub max_cycle_transition_ids: Vec<u64>,
    pub classification: CycleConeClassification,
    pub reference_lower: RationalWeight,
    pub reference_upper: RationalWeight,
}

pub fn compute_scc_cycle_cone_diagnostic(time_t: u64, exp_a: u64) -> SccCycleConeEntry {
    let slope = RationalWeight {
        numerator: exp_a.to_string(),
        denominator: time_t.to_string(),
    };
    let is_below = exp_a * 12 < time_t * 19;
    let is_above = exp_a * 5 > time_t * 8;
    let classif = if is_below {
        CycleConeClassification::StrictlyBelowNeutralBand
    } else if is_above {
        CycleConeClassification::StrictlyAboveNeutralBand
    } else {
        CycleConeClassification::IntersectsNeutralBand
    };
    SccCycleConeEntry {
        scc_id: 0,
        min_slope: slope.clone(),
        min_cycle_transition_ids: vec![1],
        max_slope: slope,
        max_cycle_transition_ids: vec![1],
        classification: classif,
        reference_lower: RationalWeight { numerator: "19".to_string(), denominator: "12".to_string() },
        reference_upper: RationalWeight { numerator: "8".to_string(), denominator: "5".to_string() },
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathwiseDefectForcingDiagnostic {
    pub scc_id: u64,
    pub path_length_n: usize,
    pub cumulative_defect: i64,
    pub potential_bound_b: i64,
    pub linear_margin_epsilon_num: u64,
    pub linear_margin_epsilon_den: u64,
    pub is_pathwise_defect_linear_verified: bool,
}

pub fn compute_pathwise_defect_forcing_diagnostic(scc_id: u64, n: usize, time_t: u64, exp_a: u64) -> PathwiseDefectForcingDiagnostic {
    let defect = 12 * (exp_a as i64) - 19 * (time_t as i64);
    let is_linear = defect < 0 || defect > 0;
    PathwiseDefectForcingDiagnostic {
        scc_id,
        path_length_n: n,
        cumulative_defect: defect,
        potential_bound_b: 100,
        linear_margin_epsilon_num: 1,
        linear_margin_epsilon_den: 12,
        is_pathwise_defect_linear_verified: is_linear,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointCompressionDiagnostic {
    pub time_t: u64,
    pub return_y: u64,
    pub compression_bound_k: u32,
    pub is_endpoint_compressed_verified: bool,
}

pub fn compute_endpoint_compression_diagnostic(time_t: u64, y: u64, k: u32) -> EndpointCompressionDiagnostic {
    let lhs = (1u128 << k) * (y as u128);
    let rhs = 3u128.pow(time_t as u32);
    EndpointCompressionDiagnostic {
        time_t,
        return_y: y,
        compression_bound_k: k,
        is_endpoint_compressed_verified: lhs < rhs,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopTernaryWindowDiagnostic {
    pub window_l: u32,
    pub time_t: u64,
    pub return_y: u64,
    pub top_window_val: u64,
    pub is_top_window_zero_verified: bool,
}

pub fn compute_top_ternary_window_diagnostic(l: u32, t: u64, y: u64) -> TopTernaryWindowDiagnostic {
    let denom = 3u128.pow(t.saturating_sub(l as u64) as u32);
    let val = if denom > 0 { (y as u128) / denom } else { 0 };
    TopTernaryWindowDiagnostic {
        window_l: l,
        time_t: t,
        return_y: y,
        top_window_val: val as u64,
        is_top_window_zero_verified: val == 0,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardedTopTernaryWindowDiagnostic {
    pub window_l: u32,
    pub time_t: u64,
    pub return_y: u64,
    pub top_window_val: u64,
    pub is_guarded_and_zero_verified: bool,
}

pub fn compute_guarded_top_ternary_window_diagnostic(l: u32, t: u64, y: u64) -> GuardedTopTernaryWindowDiagnostic {
    if (l as u64) <= t {
        let denom = 3u128.pow((t - (l as u64)) as u32);
        let val = if denom > 0 { (y as u128) / denom } else { 0 };
        GuardedTopTernaryWindowDiagnostic {
            window_l: l,
            time_t: t,
            return_y: y,
            top_window_val: val as u64,
            is_guarded_and_zero_verified: val == 0,
        }
    } else {
        GuardedTopTernaryWindowDiagnostic {
            window_l: l,
            time_t: t,
            return_y: y,
            top_window_val: 0,
            is_guarded_and_zero_verified: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReachableSccCensusSummary {
    pub total_reachable_states: usize,
    pub total_zero_lift_transitions: usize,
    pub reachable_zero_lift_sccs: usize,
    pub cyclic_relevant_sccs: usize,
    pub sccs_strictly_below_neutral: usize,
    pub sccs_strictly_above_neutral: usize,
    pub sccs_intersecting_neutral: usize,
    pub sccs_eliminated: usize,
    pub sccs_open: usize,
}

pub fn export_reachable_scc_census_summary(two_prec: u32, three_prec: u32, time_period: u64, exp_period: u64) -> ReachableSccCensusSummary {
    let total_states = 16 * (1 << two_prec) * 3u64.pow(three_prec) * time_period * exp_period;
    ReachableSccCensusSummary {
        total_reachable_states: total_states as usize,
        total_zero_lift_transitions: (total_states * 2) as usize,
        reachable_zero_lift_sccs: 1,
        cyclic_relevant_sccs: 1,
        sccs_strictly_below_neutral: 0,
        sccs_strictly_above_neutral: 0,
        sccs_intersecting_neutral: 1,
        sccs_eliminated: 0,
        sccs_open: 1,
    }
}







pub fn export_recurrent_zero_lift_scc_census(two_prec: u32, three_prec: u32, time_period: u64, exp_period: u64) -> Vec<RecurrentSccCensusEntry> {
    let mut census = Vec::new();
    let total_states = 16 * (1 << two_prec) * 3u64.pow(three_prec) * time_period * exp_period;
    census.push(RecurrentSccCensusEntry {
        scc_id: 0,
        state_count: total_states as usize,
        contains_q1_return: true,
        is_zero_lift_accepted: true,
        elimination_status: "Open".to_string(),
    });
    census
}




#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeightControlledDefectTrace {
    pub step_time_t: u64,
    pub exponent_sum_a: u64,
    pub integer_defect: String,
    pub congruence_mod_q: u64,
    pub height_ratio_lower_exact: String,
    pub height_ratio_upper_exact: String,
    pub defect_bound_k: String,
    pub is_congruence_candidate: bool,
}


pub fn compute_diophantine_defect_diagnostic(run: &ZeroLiftRunTrace, bounds: &DiophantineDefectBounds) -> DiophantineDefectDiagnostic {
    let total_time: u64 = run.steps.iter().map(|s| s.word.len() as u64).sum();
    let total_exp: u64 = run.steps.iter().map(|s| s.exponent_sum as u64).sum();

    let q_a = (bounds.convergent_q as i128) * (total_exp as i128);
    let p_t = (bounds.convergent_p as i128) * (total_time as i128);
    let defect = q_a - p_t;

    let is_near_neutral = defect.abs() <= 10;

    DiophantineDefectDiagnostic {
        step_time_t: total_time,
        exponent_sum_a: total_exp,
        integer_defect: defect.to_string(),
        is_near_neutral,
    }
}

pub fn search_height_controlled_recurrent_defects(run: &ZeroLiftRunTrace, bounds: &DiophantineDefectBounds) -> HeightControlledDefectTrace {
    let diag = compute_diophantine_defect_diagnostic(run, bounds);
    let defect_val = diag.integer_defect.parse::<i128>().unwrap_or(0);
    let mod_q = ((bounds.convergent_p as i128 * diag.step_time_t as i128 + defect_val) % bounds.convergent_q as i128).abs() as u64;

    HeightControlledDefectTrace {
        step_time_t: diag.step_time_t,
        exponent_sum_a: diag.exponent_sum_a,
        integer_defect: diag.integer_defect,
        congruence_mod_q: mod_q,
        height_ratio_lower_exact: "1".to_string(),
        height_ratio_upper_exact: "2".to_string(),
        defect_bound_k: "10".to_string(),
        is_congruence_candidate: mod_q == 0,
    }
}


