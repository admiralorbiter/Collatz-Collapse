use crate::accelerated_transition_system::AcceleratedTransitionSystemEngine;
use num_bigint::BigUint;

/// Result of evaluating an induced v-to-v transition on parameter t (where U = 81 + 256*t).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InducedVTransition {
    pub input_t: BigUint,
    pub input_z: Option<BigUint>,
    pub is_positive_realizable: bool,
    pub corresponding_k: Option<BigUint>,
    pub valuation_delta: u64,
    pub u_step_count_j: u64,
    pub is_valid_v_return: bool,
    pub next_unit_u: BigUint,
    pub next_t: Option<BigUint>,
    pub next_z: Option<BigUint>,
    pub next_t_realizable: bool,
    pub q_signature_mod27: u32,
}

/// Dyadic branch normal form for a specific j intervening u-steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DyadicBranchNormalForm {
    pub j: u64,
    pub c_j: BigUint,
    pub modulus_c: BigUint,
    pub d_j: BigUint,
    pub multiplier_d: BigUint,
    pub mu_j: u64,
    pub c_j_normalized: BigUint,
    pub d_j_normalized: BigUint,
}

/// Induced v-to-v map engine.
pub struct InducedVMapEngine;

impl InducedVMapEngine {
    /// Checks positive realizability condition: t \equiv 1 \pmod{11}.
    pub fn is_positive_realizable(t: &BigUint) -> bool {
        (t % BigUint::from(11u32)) == BigUint::from(1u32)
    }

    /// Converts parameter t to normalized coordinate z = (t - 1) / 11.
    pub fn t_to_z(t: &BigUint) -> Option<BigUint> {
        if !Self::is_positive_realizable(t) {
            None
        } else {
            Some((t - BigUint::from(1u32)) / BigUint::from(11u32))
        }
    }

    /// Converts normalized coordinate z to parameter t = 1 + 11*z.
    pub fn z_to_t(z: &BigUint) -> BigUint {
        BigUint::from(1u32) + (BigUint::from(11u32) * z)
    }

    /// Converts parameter t to ordinary quotient state k = (159 + 512*t) / 11.
    pub fn t_to_k(t: &BigUint) -> Option<BigUint> {
        if !Self::is_positive_realizable(t) {
            return None;
        }
        let num = BigUint::from(159u32) + (BigUint::from(512u32) * t);
        Some(num / BigUint::from(11u32))
    }

    /// Computes delta = v_2(231 + 729*t).
    pub fn compute_delta(t: &BigUint) -> u64 {
        let val_expr = BigUint::from(231u32) + (BigUint::from(729u32) * t);
        val_expr.trailing_zeros().unwrap_or(0)
    }

    /// Evaluates the induced v-to-v transition for parameter t.
    pub fn eval_step(t: &BigUint) -> Result<InducedVTransition, String> {
        let is_realizable = Self::is_positive_realizable(t);
        let z_opt = Self::t_to_z(t);
        let k_opt = Self::t_to_k(t);

        let delta = Self::compute_delta(t);
        if delta < 1 || !(delta - 1).is_multiple_of(4) {
            let val_expr = BigUint::from(231u32) + (BigUint::from(729u32) * t);
            let pow_delta = BigUint::from(1u32) << delta;
            let u_raw = &val_expr / &pow_delta;
            return Ok(InducedVTransition {
                input_t: t.clone(),
                input_z: z_opt,
                is_positive_realizable: is_realizable,
                corresponding_k: k_opt,
                valuation_delta: delta,
                u_step_count_j: delta / 4,
                is_valid_v_return: false,
                next_unit_u: u_raw,
                next_t: None,
                next_z: None,
                next_t_realizable: false,
                q_signature_mod27: 2,
            });
        }

        let j = (delta - 1) / 4;
        let pow_val = BigUint::from(1u32) << (1 + 4 * j);
        let val_expr = BigUint::from(231u32) + (BigUint::from(729u32) * t);
        let base_unit = &val_expr / &pow_val;

        let pow_27 = BigUint::from(27u32).pow(j as u32);
        let u_next = base_unit * pow_27;

        let u_mod256 = (&u_next % BigUint::from(256u32)).to_u64_digits().first().copied().unwrap_or(0);
        let is_valid_v = u_mod256 == 81;

        let next_t_opt = if is_valid_v {
            Some((&u_next - BigUint::from(81u32)) / BigUint::from(256u32))
        } else {
            None
        };

        let next_z_opt = if let Some(ref nt) = next_t_opt {
            Self::t_to_z(nt)
        } else {
            None
        };

        let next_t_realizable = next_z_opt.is_some();

        Ok(InducedVTransition {
            input_t: t.clone(),
            input_z: z_opt,
            is_positive_realizable: is_realizable,
            corresponding_k: k_opt,
            valuation_delta: delta,
            u_step_count_j: j,
            is_valid_v_return: is_valid_v,
            next_unit_u: u_next,
            next_t: next_t_opt,
            next_z: next_z_opt,
            next_t_realizable,
            q_signature_mod27: 2,
        })
    }

    /// Computes exact mu_j = (1 - c_j) * M_j^{-1} \pmod{11}.
    pub fn compute_mu_j(c_j: &BigUint, m_j: &BigUint) -> u64 {
        let eleven = BigUint::from(11u32);
        let m_mod = m_j % &eleven;
        let c_mod = c_j % &eleven;

        // Solve m_mod * mu \equiv (1 - c_mod) \pmod{11}
        let target = if BigUint::from(1u32) >= c_mod {
            BigUint::from(1u32) - c_mod
        } else {
            &eleven + BigUint::from(1u32) - c_mod
        };

        for mu in 0..11u64 {
            if (&m_mod * BigUint::from(mu)) % &eleven == target {
                return mu;
            }
        }
        0
    }

    /// Returns dyadic branch normal form for arbitrary j \ge 0.
    pub fn get_branch_normal_form(j: u64) -> Result<DyadicBranchNormalForm, String> {
        let k_exp = 9 + 4 * j;
        let m_j = BigUint::from(1u32) << k_exp;
        let q_j = BigUint::from(3u32).pow((6 + 3 * j) as u32);

        // Solve c_j \equiv 729^{-1} * (81 * 2^{1+4j} * 27^{-j} - 231) \pmod{2^{9+4j}}
        let inv_729 = AcceleratedTransitionSystemEngine::mod_inverse_pow2(&BigUint::from(729u32), k_exp as u32);
        let inv_pow27 = AcceleratedTransitionSystemEngine::mod_inverse_pow2(&BigUint::from(27u32).pow(j as u32), k_exp as u32);

        let term1 = (BigUint::from(81u32) * (BigUint::from(1u32) << (1 + 4 * j))) % &m_j;
        let term2 = (&term1 * &inv_pow27) % &m_j;

        let diff = if term2 >= BigUint::from(231u32) {
            (&term2 - BigUint::from(231u32)) % &m_j
        } else {
            let mod_diff = (BigUint::from(231u32) - &term2) % &m_j;
            if mod_diff == BigUint::from(0u32) {
                BigUint::from(0u32)
            } else {
                &m_j - mod_diff
            }
        };

        let c_j = (&diff * &inv_729) % &m_j;

        // Compute d_j = (27^j * (231 + 729 * c_j) / 2^{1+4j} - 81) / 256
        let pow_27 = BigUint::from(27u32).pow(j as u32);
        let pow_2 = BigUint::from(1u32) << (1 + 4 * j);
        let val_c = BigUint::from(231u32) + (BigUint::from(729u32) * &c_j);
        let base_u = &val_c / &pow_2;
        let u_next = base_u * pow_27;

        if u_next < BigUint::from(81u32) || (&u_next - BigUint::from(81u32)) % BigUint::from(256u32) != BigUint::from(0u32) {
            return Err(format!("d_j calculation failed for j = {}", j));
        }
        let d_j = (&u_next - BigUint::from(81u32)) / BigUint::from(256u32);

        let mu_j = Self::compute_mu_j(&c_j, &m_j);
        let c_norm = (&c_j - BigUint::from(1u32) + (&m_j * BigUint::from(mu_j))) / BigUint::from(11u32);
        let d_norm = (&d_j - BigUint::from(1u32) + (&q_j * BigUint::from(mu_j))) / BigUint::from(11u32);

        Ok(DyadicBranchNormalForm {
            j,
            c_j,
            modulus_c: m_j,
            d_j,
            multiplier_d: q_j,
            mu_j,
            c_j_normalized: c_norm,
            d_j_normalized: d_norm,
        })
    }
}
