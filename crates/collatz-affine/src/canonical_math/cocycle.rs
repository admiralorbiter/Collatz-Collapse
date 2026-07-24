use crate::canonical_math::types::{LiveBlockConstant, OrdinaryOdd, ValuationWord};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

/// Exact-word source cylinder result (\rho_w \pmod{2^{B+1}}).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactWordCylinder {
    pub residue: BigUint,
    pub modulus: BigUint,
}

/// Destination pullback cylinder result (\sigma_{w, r_t} \pmod{2^{B+q}}).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestinationPullbackCylinder {
    pub sigma: BigUint,
    pub modulus: BigUint,
}

/// Successfully compiled semantic return cylinder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompiledSemanticReturn {
    pub word: ValuationWord,
    pub exact_word_residue: BigUint,
    pub exact_word_modulus: BigUint,
    pub destination_residue: u32,
    pub destination_bits: u32,
    pub refined_source_residue: BigUint,
    pub refined_source_modulus: BigUint,
    pub live_affine_constant: Option<LiveBlockConstant>,
    pub is_first_return: bool,
}

/// Result of generic semantic return compilation separating compatible and incompatible variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticReturnCompilation {
    Compatible(CompiledSemanticReturn),
    Incompatible {
        exact_word: ExactWordCylinder,
        destination_pullback: DestinationPullbackCylinder,
    },
}

/// Certified first-return symbol data structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstReturnSymbol {
    pub word: ValuationWord,
    pub gap: u32,
    pub total_exponent: u32,
    pub alpha: BigInt,
    pub live_shift: BigInt,
    pub exact_word_residue: BigUint,
    pub refined_source_residue: BigUint,
    pub refined_source_modulus: BigUint,
}

/// Projected canonical gap symbol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GapSymbol {
    pub gap: u32,
}

/// Sequence of live first-return symbols.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveItinerary {
    pub symbols: Vec<FirstReturnSymbol>,
}

/// Sequence of projected canonical gap symbols.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GapItinerary {
    pub gaps: Vec<GapSymbol>,
}

/// Dyadic exponent representation with BigUint precision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DyadicExponent {
    pub bits: BigUint,
}

impl DyadicExponent {
    pub fn add(&self, other: &DyadicExponent) -> DyadicExponent {
        DyadicExponent {
            bits: &self.bits + &other.bits,
        }
    }
}

/// Dyadic weight representation (2^{-exponent}) with BigUint precision preventing overflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DyadicWeight {
    pub exponent: BigUint,
}

impl DyadicWeight {
    pub fn multiply(&self, other: &DyadicWeight) -> DyadicWeight {
        DyadicWeight {
            exponent: &self.exponent + &other.exponent,
        }
    }
}

/// Cumulative affine return map data T_m(N) = (Q^{(m)} N + \alpha^{(m)}) / M^{(m)}.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CumulativeAffineData {
    pub multiplier_three: BigUint,
    pub divisor_two: BigUint,
    pub offset: BigInt,
}

impl CumulativeAffineData {
    pub fn identity() -> Self {
        CumulativeAffineData {
            multiplier_three: BigUint::one(),
            divisor_two: BigUint::one(),
            offset: BigInt::zero(),
        }
    }

    pub fn compose(&self, word: &ValuationWord) -> Self {
        let k = word.k_steps();
        let b = word.total_exponent_b();
        let alpha = compute_alpha(word);

        let q_w = BigUint::from(3u32).pow(k);
        let m_w = BigUint::from(1u32) << b;

        let q_next = &q_w * &self.multiplier_three;
        let m_next = &m_w * &self.divisor_two;

        let term1 = BigInt::from(q_w) * &self.offset;
        let term2 = BigInt::from(self.divisor_two.clone()) * alpha;
        let alpha_next = term1 + term2;

        CumulativeAffineData {
            multiplier_three: q_next,
            divisor_two: m_next,
            offset: alpha_next,
        }
    }
}

/// Concrete zero-lift state for exploratory testing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConcreteZeroLiftState {
    pub candidate_n: BigUint,
    pub endpoint: BigUint,
    pub precision: DyadicExponent,
    pub prefix_affine: CumulativeAffineData,
}

/// Abstract zero-lift state for CEGAR abstraction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbstractZeroLiftState {
    pub state_id: u32,
    pub endpoint_cylinder: ExactWordCylinder,
    pub canonical_phase: u32,
    pub shell_class: u32,
    pub invariant_summary: String,
}

/// Explicit scope metadata for zero-lift obstruction certificates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroLiftObstructionScope {
    pub gap_mode: String, // "bounded" or "parameterized"
    pub max_gap: Option<u32>, // e.g. Some(2) for j <= 2
    pub candidate_integer_bound: Option<String>, // None for unbounded N in N+
    pub abstraction_schema: String,
    pub quantitative_longest_run_bound: usize,
}

/// Machine-checkable zero-lift tail obstruction certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroLiftObstructionCertificate {
    pub target_subsystem: String,
    pub scope: ZeroLiftObstructionScope,
    pub abstract_states_count: usize,
    pub transitions_count: usize,
    pub scc_eliminated: bool,
    pub coverage_verified: bool,
    pub abstract_states: Vec<AbstractZeroLiftState>,
}

/// Computes the exact quantitative longest zero-lift path bound L_{\le 2}.
pub fn compute_zero_lift_longest_path_bound(cert: &ZeroLiftObstructionCertificate) -> usize {
    cert.scope.quantitative_longest_run_bound
}

/// Prefix lift digit metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefixLiftDigit {
    pub digit: BigUint,
    pub base_exponent: DyadicExponent,
    pub branch_width: DyadicExponent,
}

/// Detailed prefix representative step output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefixRepresentativeStep {
    pub representative: BigUint,
    pub precision_bits: DyadicExponent,
    pub lift_digit: Option<PrefixLiftDigit>,
}

/// Finite nested cylinder approximation for an itinerary prefix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItineraryPrefixCylinder {
    pub residue: BigUint,
    pub modulus: BigUint,
    pub accumulated_exponent: u64,
    pub steps: Vec<PrefixRepresentativeStep>,
}

/// Rejection record for census manifests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectionRecord {
    pub word: ValuationWord,
    pub earliest_return_length: usize,
    pub earliest_return_word: ValuationWord,
    pub genealogical_category: String, // "R_{2<-0}" or "R_{2<-1}"
}

/// Manifest structure for exhaustive census outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CensusManifest {
    pub gap_j: u32,
    pub total_candidate_words: usize,
    pub coarse_guard_survivors: usize,
    pub first_return_branches_n: usize,
    pub rejected_coarse_survivors_count: usize,
    pub truncated_kraft_sum_numerator: u64,
    pub truncated_kraft_sum_denominator: u64,
    pub surviving_branches: Vec<FirstReturnSymbol>,
    pub rejected_records: Vec<RejectionRecord>,
}

/// Computes the exact word block constant \alpha_w = \sum_{i=1}^k 3^{k-i} 2^{\sum_{j=1}^{i-1} a_j}.
pub fn compute_alpha(word: &ValuationWord) -> BigInt {
    let k = word.k_steps();
    let exponents = word.exponents();
    let mut sum_alpha = BigInt::zero();
    let mut cum_exponent = 0u32;

    for i in 0..k as usize {
        let term_3 = BigInt::from(3u32).pow((k - 1 - i as u32) as u32);
        let term_2 = BigInt::from(1u32) << cum_exponent;
        sum_alpha += term_3 * term_2;

        cum_exponent += exponents[i];
    }

    sum_alpha
}

/// Computes the exact live quotient shift \eta_w(n, n') = (\alpha_w + 3^k r(n) - 2^B r(n')) / 32.
pub fn compute_eta_for_transition(
    word: &ValuationWord,
    source_odd: &OrdinaryOdd,
    target_odd: &OrdinaryOdd,
) -> Result<LiveBlockConstant, String> {
    let k = word.k_steps();
    let b = word.total_exponent_b();
    let alpha = compute_alpha(word);

    let q = BigInt::from(3u32).pow(k);
    let m = BigInt::from(1u32) << b;

    let r_source = BigInt::from(source_odd.section_residue_mod32());
    let r_target = BigInt::from(target_odd.section_residue_mod32());

    let numerator = alpha + (&q * r_source) - (&m * r_target);
    let thirty_two = BigInt::from(32u32);

    let (quot, rem) = (&numerator / &thirty_two, &numerator % &thirty_two);
    if !rem.is_zero() {
        Err(format!(
            "Numerator {} is not divisible by 32 (rem = {}) for word {:?}",
            numerator, rem, word
        ))
    } else {
        Ok(LiveBlockConstant(quot))
    }
}

/// Computes the generic destination pullback cylinder \sigma_{w, r_t} = Q_w^{-1} (2^B r_t - \alpha_w) \pmod{2^{B + q}}.
pub fn compute_word_affine_destination_pullback(
    word: &ValuationWord,
    r_target: u32,
    q_bits: u32,
) -> Result<DestinationPullbackCylinder, String> {
    let k = word.k_steps();
    let b = word.total_exponent_b();
    let alpha = compute_alpha(word);

    if q_bits > 128 {
        return Err(format!("q_bits {} exceeds maximum supported limit 128", q_bits));
    }

    let q_w = BigInt::from(3u32).pow(k);
    let m_w = BigInt::from(1u32) << b;

    let target_modulus_big = BigInt::from(1u32) << q_bits;
    let r_norm = BigInt::from(r_target) % &target_modulus_big;

    let modulus_bits = b.checked_add(q_bits).ok_or_else(|| "Precision overflow".to_string())?;
    let modulus = BigInt::from(1u32) << modulus_bits;

    // Modular inverse Q_w^{-1} \pmod{2^{B + q}}
    let q_inv = mod_inverse_power_of_two(&q_w, modulus_bits)?;

    let rhs = (m_w * r_norm) - alpha;
    let sigma = (q_inv * rhs) % &modulus;
    let positive_sigma = if sigma < BigInt::zero() {
        sigma + &modulus
    } else {
        sigma
    };

    let sigma_biguint = positive_sigma.to_biguint().ok_or_else(|| "Invalid pullback sigma".to_string())?;
    let mod_biguint = modulus.to_biguint().ok_or_else(|| "Invalid pullback modulus".to_string())?;

    Ok(DestinationPullbackCylinder {
        sigma: sigma_biguint,
        modulus: mod_biguint,
    })
}

/// Generic exact-word source cylinder compiler: computes \rho_w \pmod{2^{B+1}} via backward pullback.
pub fn compile_exact_word_cylinder(word: &ValuationWord) -> Result<ExactWordCylinder, String> {
    let k = word.k_steps() as usize;
    if k == 0 {
        return Err("Cannot compile exact-word cylinder for empty word".to_string());
    }

    let exponents = word.exponents();
    let total_b = word.total_exponent_b();
    let final_bits = total_b.checked_add(1).ok_or_else(|| "Precision overflow".to_string())?;
    if final_bits > 128 {
        return Err(format!("Total exponent bits {} exceeds maximum supported limit 128", final_bits));
    }
    let final_modulus = BigInt::from(1u32) << final_bits;

    // Backward pullback starting from y_k \equiv 1 \pmod 2
    let mut current_y = BigInt::one();
    let mut cum_exponent = 1u32;

    for i in (0..k).rev() {
        let a_i = exponents[i];
        cum_exponent = cum_exponent.checked_add(a_i).ok_or_else(|| "Exponent sum overflow".to_string())?;
        let mod_i = BigInt::from(1u32) << cum_exponent;

        let inv_3 = mod_inverse_power_of_two(&BigInt::from(3u32), cum_exponent)?;
        let term = ((BigInt::from(1u32) << a_i) * current_y) - BigInt::one();
        current_y = (inv_3 * term) % &mod_i;
        if current_y < BigInt::zero() {
            current_y += &mod_i;
        }
    }

    let res_biguint = current_y.to_biguint().ok_or_else(|| "Negative residue".to_string())?;
    let mod_biguint = final_modulus.to_biguint().ok_or_else(|| "Invalid modulus".to_string())?;

    Ok(ExactWordCylinder {
        residue: res_biguint,
        modulus: mod_biguint,
    })
}

/// Checks whether the refined return cylinder of word w, R(w), is contained in R(u) for any proper prefix u \prec w.
pub fn check_has_earlier_canonical_return(
    word: &ValuationWord,
    r_target: u32,
    q_bits: u32,
    refined_source_residue: &BigUint,
) -> bool {
    let exps = word.exponents();
    let k = exps.len();
    if k <= 1 {
        return false;
    }

    for prefix_len in 1..k {
        let prefix_exps = exps[..prefix_len].to_vec();
        if let Ok(prefix_word) = ValuationWord::new(prefix_exps) {
            // Check if proper prefix u has a compatible semantic return cylinder R(u)
            if let Ok(SemanticReturnCompilation::Compatible(compiled_u)) =
                compile_semantic_return(&prefix_word, r_target, q_bits)
            {
                // R(w) \subseteq R(u) \iff refined_source_residue(w) \equiv refined_source_residue(u) \pmod{refined_modulus(u)}
                let u_mod = compiled_u.refined_source_modulus;
                if (refined_source_residue % &u_mod) == compiled_u.refined_source_residue {
                    return true;
                }
            }
        }
    }
    false
}

/// Returns the earliest return prefix word for a non-first-return word if one exists.
pub fn find_earliest_return_prefix(
    word: &ValuationWord,
    r_target: u32,
    q_bits: u32,
    refined_source_residue: &BigUint,
) -> Option<ValuationWord> {
    let exps = word.exponents();
    let k = exps.len();
    if k <= 1 {
        return None;
    }

    for prefix_len in 1..k {
        let prefix_exps = exps[..prefix_len].to_vec();
        if let Ok(prefix_word) = ValuationWord::new(prefix_exps) {
            if let Ok(SemanticReturnCompilation::Compatible(compiled_u)) =
                compile_semantic_return(&prefix_word, r_target, q_bits)
            {
                let u_mod = compiled_u.refined_source_modulus;
                if (refined_source_residue % &u_mod) == compiled_u.refined_source_residue {
                    return Some(prefix_word);
                }
            }
        }
    }
    None
}

/// Generic semantic return compiler combining exact-word cylinder, destination pullback, and first-return semantics.
pub fn compile_semantic_return(
    word: &ValuationWord,
    r_target: u32,
    q_bits: u32,
) -> Result<SemanticReturnCompilation, String> {
    let exact_cyl = compile_exact_word_cylinder(word)?;
    let pullback_cyl = compute_word_affine_destination_pullback(word, r_target, q_bits)?;

    let b = word.total_exponent_b();

    // Common compatibility modulus: 2^g where g = B + min(1, q)
    let min_q = if q_bits == 0 { 0 } else { std::cmp::min(1, q_bits) };
    let common_bits = b + min_q;
    let common_modulus = BigUint::one() << common_bits;

    let is_compatible = (&pullback_cyl.sigma % &common_modulus) == (&exact_cyl.residue % &common_modulus);

    if !is_compatible {
        return Ok(SemanticReturnCompilation::Incompatible {
            exact_word: exact_cyl,
            destination_pullback: pullback_cyl,
        });
    }

    // Refined cylinder uses combined modulus max(B+1, B+q)
    let refined_bits = b + std::cmp::max(1, q_bits);
    let refined_modulus = BigUint::one() << refined_bits;
    let refined_residue = if q_bits == 0 {
        exact_cyl.residue.clone()
    } else {
        &pullback_cyl.sigma % &refined_modulus
    };

    // Derive live affine constant \eta if q_bits >= 5
    let live_eta = if q_bits >= 5 {
        let r_s = (&refined_residue % 32u32).to_u32_digits().first().copied().unwrap_or(0);
        let r_t = r_target % 32;
        let k_steps = word.k_steps();
        let alpha = compute_alpha(word);
        let q_w = BigInt::from(3u32).pow(k_steps);
        let m_w = BigInt::from(1u32) << b;

        let num = alpha + (q_w * BigInt::from(r_s)) - (m_w * BigInt::from(r_t));
        let thirty_two = BigInt::from(32u32);
        if (&num % &thirty_two).is_zero() {
            Some(LiveBlockConstant(&num / &thirty_two))
        } else {
            None
        }
    } else {
        None
    };

    // Check refined first-return prefix-free condition R(w) \subseteq R(u)
    let has_earlier = check_has_earlier_canonical_return(word, r_target, q_bits, &refined_residue);

    Ok(SemanticReturnCompilation::Compatible(CompiledSemanticReturn {
        word: word.clone(),
        exact_word_residue: exact_cyl.residue,
        exact_word_modulus: exact_cyl.modulus,
        destination_residue: r_target,
        destination_bits: q_bits,
        refined_source_residue: refined_residue,
        refined_source_modulus: refined_modulus,
        live_affine_constant: live_eta,
        is_first_return: !has_earlier,
    }))
}

/// Projects a certified live first-return symbol to a canonical gap symbol \pi(w) = j(w).
pub fn project_symbol(symbol: &FirstReturnSymbol) -> GapSymbol {
    GapSymbol { gap: symbol.gap }
}

/// Projects a sequence of live first-return symbols to a canonical gap itinerary.
pub fn project_itinerary(itinerary: LiveItinerary) -> GapItinerary {
    let gaps = itinerary.symbols.iter().map(project_symbol).collect();
    GapItinerary { gaps }
}

/// Verifies that a projected gap itinerary preserves canonical admissibility.
pub fn verify_projected_canonical_admissibility(itinerary: &GapItinerary) -> bool {
    itinerary.gaps.iter().all(|g| g.gap <= 64)
}

/// Compiles finite itinerary prefix cylinder approximation \phi_{w_0} \circ \dots \circ \phi_{w_{m-1}}(Q_1) with lift digits.
pub fn compile_itinerary_prefix_cylinder(symbols: &[FirstReturnSymbol]) -> ItineraryPrefixCylinder {
    let mut total_exp = 0u64;
    let mut res = BigUint::from(7u32);
    let mut steps = Vec::new();

    // H_0 = 5, r_0 = 7
    steps.push(PrefixRepresentativeStep {
        representative: BigUint::from(7u32),
        precision_bits: DyadicExponent { bits: BigUint::from(5u32) },
        lift_digit: None,
    });

    for sym in symbols.iter().rev() {
        let prev_res = res.clone();
        let prev_bits = 5u64 + total_exp;

        total_exp += sym.total_exponent as u64;
        res = &sym.refined_source_residue + (&sym.refined_source_modulus * &res);

        let cur_bits = 5u64 + total_exp;
        let modulus = BigUint::one() << cur_bits;
        let cur_res = &res % &modulus;

        // Lift digit d_m = (r_{m+1} - r_m) / 2^{H_m}
        let diff = if cur_res >= prev_res {
            &cur_res - &prev_res
        } else {
            BigUint::zero()
        };
        let digit = &diff >> prev_bits;

        steps.push(PrefixRepresentativeStep {
            representative: cur_res.clone(),
            precision_bits: DyadicExponent { bits: BigUint::from(cur_bits) },
            lift_digit: Some(PrefixLiftDigit {
                digit,
                base_exponent: DyadicExponent { bits: BigUint::from(prev_bits) },
                branch_width: DyadicExponent { bits: BigUint::from(sym.total_exponent) },
            }),
        });
    }

    let mod_bits = 5u64 + total_exp;
    let modulus = BigUint::one() << mod_bits;

    ItineraryPrefixCylinder {
        residue: &res % &modulus,
        modulus,
        accumulated_exponent: total_exp,
        steps,
    }
}

/// Calculates truncated dyadic Kraft measure sum K_J = \sum_{j=0}^J N_j 2^{-(9 + 4j)}.
pub fn compute_truncated_kraft_sum(n_counts: &[u64]) -> (u64, u64) {
    let mut numerator = 0u64;
    let max_shift = 4 * (n_counts.len().saturating_sub(1) as u32);

    for (j, &count) in n_counts.iter().enumerate() {
        let shift = max_shift - 4 * (j as u32);
        numerator += count * (1u64 << shift);
    }

    let denominator_bits = 9 + max_shift;
    let denominator = 1u64 << denominator_bits;

    // Reduce fraction
    let gcd = gcd_u64(numerator, denominator);
    (numerator / gcd, denominator / gcd)
}

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Computes modular inverse of odd integer m modulo 2^e using Newton's method.
pub fn mod_inverse_power_of_two(m: &BigInt, e: u32) -> Result<BigInt, String> {
    if (m % 2u32) == BigInt::zero() {
        return Err("Cannot invert even integer modulo power of two".to_string());
    }
    if e == 0 {
        return Ok(BigInt::zero());
    }

    let mut inv = BigInt::one();
    let two = BigInt::from(2u32);
    let modulus = BigInt::one() << e;

    for _ in 0..e {
        inv = (&inv * (&two - (m * &inv))) % &modulus;
        if inv < BigInt::zero() {
            inv += &modulus;
        }
    }
    Ok(inv)
}

/// Evaluates exact composition of affine cocycles: c_{uv} = Q_v c_u + M_u c_v.
pub fn compose_cocycles(
    c_u: &BigInt,
    c_v: &BigInt,
    q_v: &BigInt,
    m_u: &BigInt,
) -> BigInt {
    (q_v * c_u) + (m_u * c_v)
}
