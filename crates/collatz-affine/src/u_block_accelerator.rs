use num_bigint::BigUint;

/// Result of accelerating a maximal block of u-returns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UBlockResult {
    pub initial_k: BigUint,
    pub initial_valuation_x: u64,
    pub arbitrary_u_count_l: u64,
    pub final_valuation_x: u64,
    pub final_k: BigUint,
    pub final_unit_u: BigUint,
}

/// Accelerator for consecutive u-returns.
pub struct UBlockAccelerator;

impl UBlockAccelerator {
    /// Computes the exact 2-adic valuation x = v_2(32*(11k+3)) = 5 + v_2(11k+3).
    pub fn compute_valuation_x(k: &BigUint) -> u64 {
        let val_expr = (BigUint::from(11u32) * k) + BigUint::from(3u32);
        let trailing_zeros = val_expr.trailing_zeros().unwrap_or(0);
        5 + trailing_zeros
    }

    /// Computes arbitrary u-phase countdown length l_u(x) = max(0, (x - 5) / 4).
    pub fn arbitrary_u_countdown_length(x: u64) -> u64 {
        if x < 9 {
            0
        } else {
            (x - 5) / 4
        }
    }

    /// Evaluates maximal u-step acceleration for state k.
    /// Requirements: k \equiv 7 \pmod{16} (i.e. x \ge 9 for u execution).
    pub fn accelerate(k: &BigUint) -> Result<UBlockResult, String> {
        let x = Self::compute_valuation_x(k);
        if x < 9 {
            return Err(format!("State k = {} is not in Q_1 u-execution guard (x = {} < 9)", k, x));
        }

        // Maximal consecutive u-steps from arbitrary state: l_u(x) = (x - 5) / 4
        let l = Self::arbitrary_u_countdown_length(x);

        let expr_0 = (BigUint::from(11u32) * k) + BigUint::from(3u32);
        let pow_27 = BigUint::from(27u32).pow(l as u32);
        let pow_16 = BigUint::from(16u32).pow(l as u32);

        let num = &pow_27 * &expr_0;
        if &num % &pow_16 != BigUint::from(0u32) {
            return Err("Exact division failed in u-acceleration".to_string());
        }
        let expr_l = &num / &pow_16;

        if expr_l < BigUint::from(3u32) || (&expr_l - BigUint::from(3u32)) % BigUint::from(11u32) != BigUint::from(0u32) {
            return Err("Integral quotient failed in u-acceleration".to_string());
        }
        let final_k = (&expr_l - BigUint::from(3u32)) / BigUint::from(11u32);

        let final_x = x - 4 * l;
        let final_unit = &expr_l >> (final_x - 5);

        Ok(UBlockResult {
            initial_k: k.clone(),
            initial_valuation_x: x,
            arbitrary_u_count_l: l,
            final_valuation_x: final_x,
            final_k,
            final_unit_u: final_unit,
        })
    }
}
