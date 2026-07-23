use crate::inversion::hensel_inverse_3_pow;
use crate::{
    AffineError, CanonicalCylinder, ExactWordCylinder, MacrostepData, Q1Quotient,
    QuotientAffineRule, QuotientRegisterMachine, ValuationWord, Q1_EXPONENT, Q1_RESIDUE,
};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Signed};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositiveImageProgression {
    pub start: BigUint,
    pub step: BigUint,
}

/// Single-macrostep return classification over a base state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardedReturnClassification {
    pub exact_word_cylinder: CanonicalCylinder,
    pub based_return_cylinder: CanonicalCylinder,
    pub positive_image: PositiveImageProgression,
    pub target_cylinder: CanonicalCylinder,
    pub quotient_guard: CanonicalCylinder,
    pub quotient_rule: QuotientAffineRule,
}

/// Checkpoint status along a multi-step guarded path trajectory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathCheckpoint {
    pub step_index: usize,
    pub word: ValuationWord,
    pub image_cylinder: CanonicalCylinder,
    pub base_membership_verified: bool,
}

/// Multi-step guarded path classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardedPathClassification {
    pub steps: Vec<ValuationWord>,
    pub source_cylinder: CanonicalCylinder,
    pub quotient_guard: CanonicalCylinder,
    pub checkpoints: Vec<PathCheckpoint>,
    pub final_target: CanonicalCylinder,
}

/// Classifies broad, exact, based-return, positive image, and quotient guard for a macrostep over base r mod 2^q.
pub fn classify_guarded_return(
    macrostep: &MacrostepData,
    base: &CanonicalCylinder,
) -> Result<GuardedReturnClassification, AffineError> {
    let exact_word = ExactWordCylinder::from_valuation_word(macrostep.word().clone())?;
    let rule = QuotientRegisterMachine::derive_rule(macrostep)?;

    // Source based-return cylinder: a_p * n + c_p \equiv 2^{A_p} * r (mod 2^{A_p + q})
    // n \equiv (2^{A_p} * r - c_p) * a_p^{-1} (mod 2^{A_p + q})
    let total_exp = macrostep.total_valuation() + base.modulus_exponent;
    let modulus = BigUint::one() << total_exp;

    let b_val = macrostep.denominator();
    let r_val = &base.residue;
    let c_val = macrostep.constant();

    let target_b_r = b_val * r_val;
    let target_bigint = BigInt::from_biguint(Sign::Plus, target_b_r % &modulus);
    let c_bigint = macrostep.constant_int();

    let diff = (target_bigint - c_bigint) % BigInt::from_biguint(Sign::Plus, modulus.clone());
    let positive_diff = if diff.is_negative() {
        diff + BigInt::from_biguint(Sign::Plus, modulus.clone())
    } else {
        diff
    };

    let k_steps = macrostep.odd_steps() as u32;
    let inv_a_k = hensel_inverse_3_pow(total_exp).modpow(&BigUint::from(k_steps), &modulus);

    let diff_biguint = positive_diff.to_biguint().unwrap();
    let based_residue = (diff_biguint * inv_a_k) % &modulus;
    let based_return_cylinder = CanonicalCylinder::new(based_residue.clone(), total_exp);

    // Calculate start of positive image progression: F_p(n_0)
    let num_start = (macrostep.multiplier() * &based_residue) + c_val;
    let start_img = num_start / b_val;
    let step_img = macrostep.multiplier() << base.modulus_exponent;

    let positive_image = PositiveImageProgression {
        start: start_img,
        step: step_img,
    };

    let quotient_guard =
        CanonicalCylinder::new(rule.guard_residue.clone(), rule.guard_modulus_exponent);

    Ok(GuardedReturnClassification {
        exact_word_cylinder: exact_word.source,
        based_return_cylinder,
        positive_image,
        target_cylinder: base.clone(),
        quotient_guard,
        quotient_rule: rule,
    })
}

/// Composes a multiword guarded path over Q_1 and records intermediate checkpoints.
pub fn compose_guarded_path(
    steps: &[MacrostepData],
    base: &CanonicalCylinder,
) -> Result<GuardedPathClassification, AffineError> {
    if steps.is_empty() {
        return Err(AffineError::EmptyValuationWord);
    }

    let mut current_target = CanonicalCylinder::new(BigUint::from(0u32), 0);
    let mut current_exp = 0u64;

    // Work backward from final target using inverse-guard composition
    let mut word_list = Vec::new();
    let mut rules = Vec::new();

    for step in steps.iter().rev() {
        word_list.push(step.word().clone());
        let rule = QuotientRegisterMachine::derive_rule(step)?;

        let (pred_res, pred_exp) = QuotientRegisterMachine::preimage_guard(
            &rule,
            &current_target.residue,
            current_exp,
        );

        current_target = CanonicalCylinder::new(pred_res, pred_exp);
        current_exp = pred_exp;
        rules.push(rule);
    }

    word_list.reverse();
    rules.reverse();

    let quotient_guard = current_target.clone();

    // Map quotient guard to source cylinder in n-space
    let source_residue = (&quotient_guard.residue << Q1_EXPONENT) + Q1_RESIDUE;
    let source_exp = quotient_guard.modulus_exponent + Q1_EXPONENT as u64;
    let source_cylinder = CanonicalCylinder::new(source_residue, source_exp);

    // Compute forward checkpoints
    let mut checkpoints = Vec::new();
    let mut curr_n = source_cylinder.residue.clone();

    for (idx, step) in steps.iter().enumerate() {
        let q = Q1Quotient::from_integer(&curr_n)?;
        let outcome = QuotientRegisterMachine::eval_transition(step, &q)?;

        match outcome {
            crate::ReturnTransitionOutcome::BasedReturn { image, .. } => {
                let img_mod_exp = source_exp; // upper bound on precision
                let img_mod = BigUint::one() << img_mod_exp;
                let img_res = &image % img_mod;

                let is_q1 = (&image % 32u32) == BigUint::from(Q1_RESIDUE);
                checkpoints.push(PathCheckpoint {
                    step_index: idx,
                    word: step.word().clone(),
                    image_cylinder: CanonicalCylinder::new(img_res, img_mod_exp),
                    base_membership_verified: is_q1,
                });
                curr_n = image;
            }
            _ => {
                return Err(AffineError::ZeroValuation(idx));
            }
        }
    }

    Ok(GuardedPathClassification {
        steps: word_list,
        source_cylinder,
        quotient_guard,
        checkpoints,
        final_target: base.clone(),
    })
}
