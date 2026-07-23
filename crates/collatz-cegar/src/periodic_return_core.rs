use crate::accelerated_branch_params::AcceleratedBranchParams;
use num_bigint::{BigInt, BigUint};

#[derive(Debug, Clone)]
pub struct PeriodicReturnCore {
    pub block: Vec<u64>,
    pub modulus_m: BigInt,
    pub multiplier_q: BigInt,
    pub affine_beta: BigInt,
    pub fixed_numerator: BigInt,
    pub fixed_denominator: BigInt,
}

#[derive(Debug, Clone)]
pub struct CoreShadowCertificate {
    pub word: Vec<u64>,
    pub core_block: Vec<u64>,
    pub valuation_v2: u32,
    pub block_precision_b_v: u32,
    pub repetitions_certified_r: usize,
    pub slack_bits: u32,
    pub endpoint_residue: BigInt,
    pub core_residue: BigInt,
    pub is_separated_at_bit: u32,
}

pub struct PeriodicReturnCoreEngine;

impl PeriodicReturnCoreEngine {
    /// Compute 2-adic modular inverse of an odd BigInt modulo 2^bits
    pub fn mod_inverse_2adic(a: &BigInt, bits: u32) -> BigInt {
        assert!(a % 2 != BigInt::from(0), "a must be odd");
        let mod_val = BigInt::from(2u64).pow(bits);
        let a_biguint = (a % &mod_val + &mod_val) % &mod_val;

        let a_num = a_biguint.to_biguint().unwrap_or_default();
        let m_num = mod_val.to_biguint().unwrap_or_default();

        let inv = a_num.modpow(&(&m_num / BigUint::from(2u64) - BigUint::from(1u64)), &m_num);
        BigInt::from(inv)
    }

    /// Compute exact rational 2-adic fixed point \xi_v = \beta_v / (Q_v - M_v)
    /// In M_v - Q_v denominator: \xi_v = -\beta_v / (M_v - Q_v)
    pub fn compute_periodic_core(block: &[u64]) -> PeriodicReturnCore {
        assert!(!block.is_empty(), "Block cannot be empty");

        let p0 = AcceleratedBranchParams::for_gap(block[0]);
        let mut m_v = BigInt::from(p0.modulus.clone());
        let mut q_v = BigInt::from(p0.multiplier.clone());
        let mut beta_v = BigInt::from(26u64);

        for &h in &block[1..] {
            let p_h = AcceleratedBranchParams::for_gap(h);
            let m_h = BigInt::from(p_h.modulus.clone());
            let q_h = BigInt::from(p_h.multiplier.clone());

            beta_v = &m_h * &beta_v + &q_v * BigInt::from(26u64);
            m_v = &m_v * &m_h;
            q_v = &q_v * &q_h;
        }

        let fixed_numerator = -&beta_v;
        let fixed_denominator = &m_v - &q_v;

        PeriodicReturnCore {
            block: block.to_vec(),
            modulus_m: m_v,
            multiplier_q: q_v,
            affine_beta: beta_v,
            fixed_numerator,
            fixed_denominator,
        }
    }

    /// Primitive Integer Valuation Formula: v_2(D - \xi_v) = v_2((Q_v - M_v) D + \beta_v)
    pub fn compute_primitive_integer_valuation(endpoint_d: &BigInt, core: &PeriodicReturnCore) -> u32 {
        // (Q_v - M_v) * D + \beta_v
        let denom_pos = &core.multiplier_q - &core.modulus_m;
        let val_expr = &denom_pos * endpoint_d + &core.affine_beta;

        if val_expr == BigInt::from(0) {
            return 999999;
        }

        let val_biguint = val_expr.to_biguint().unwrap_or_default();
        val_biguint.trailing_zeros().unwrap_or(0) as u32
    }

    /// Prove Real-Sign Negative Core Theorem & Eventually Periodic Elimination: \xi_v < 0 for all nonempty blocks
    pub fn prove_eventually_periodic_elimination(core: &PeriodicReturnCore) -> bool {
        // \xi_v = -\beta_v / (M_v - Q_v) = \beta_v / (Q_v - M_v)
        // Since Q_v > M_v and \beta_v > 0, denominator M_v - Q_v < 0, numerator -\beta_v < 0 => \xi_v < 0 as a real rational
        // Hence no positive integer D > 0 can equal or eventually converge to \xi_v < 0!
        core.affine_beta > BigInt::from(0) && core.multiplier_q > core.modulus_m && core.fixed_numerator < BigInt::from(0)
    }

    /// Compute exact Kraft/Shannon Information Decomposition bits B_v = \sum B_h
    pub fn compute_kraft_shannon_precision(block: &[u64]) -> u32 {
        let mut total_b = 0u32;
        for &h in block {
            let p_h = AcceleratedBranchParams::for_gap(h);
            total_b += p_h.precision as u32;
        }
        total_b
    }

    /// Compute 2-adic residue of \xi_v mod 2^bits
    pub fn compute_core_residue(core: &PeriodicReturnCore, bits: u32) -> BigInt {
        let inv_den = Self::mod_inverse_2adic(&core.fixed_denominator, bits);
        let mod_val = BigInt::from(2u64).pow(bits);
        (&core.fixed_numerator * &inv_den % &mod_val + &mod_val) % &mod_val
    }

    /// Evaluate Core Shadow Certificate with Primitive Integer Valuation & Error Transport Identity
    pub fn evaluate_shadow_certificate(
        word: &[u64],
        target_d: &BigInt,
        core_block: &[u64],
    ) -> CoreShadowCertificate {
        let core = Self::compute_periodic_core(core_block);
        let b_v = Self::compute_kraft_shannon_precision(core_block);

        let val2 = Self::compute_primitive_integer_valuation(target_d, &core);
        let repetitions_r = (val2 / b_v) as usize;
        let slack = val2 % b_v;

        let mod_val2 = BigInt::from(2u64).pow(val2);
        let d_res = (target_d % &mod_val2 + &mod_val2) % &mod_val2;
        let xi_res = Self::compute_core_residue(&core, val2);

        CoreShadowCertificate {
            word: word.to_vec(),
            core_block: core_block.to_vec(),
            valuation_v2: val2,
            block_precision_b_v: b_v,
            repetitions_certified_r: repetitions_r,
            slack_bits: slack,
            endpoint_residue: d_res,
            core_residue: xi_res,
            is_separated_at_bit: val2,
        }
    }
}
