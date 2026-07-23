use collatz_affine::AffinePrefix;
use num_rational::Ratio;

/// Krasikov-Lagarias (2003) Linear Difference Inequality Solver.
/// Evaluates rational linear potential functions V_r(n) = q_r * n + b_r
/// to certify trajectory contraction across macrosteps before heavy SMT/SyGuS synthesis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinearPotential {
    pub coefficient_q: Ratio<i64>,
    pub constant_b: Ratio<i64>,
}

impl LinearPotential {
    pub fn new(q_num: i64, q_den: i64, b_num: i64, b_den: i64) -> Self {
        Self {
            coefficient_q: Ratio::new(q_num, q_den),
            constant_b: Ratio::new(b_num, b_den),
        }
    }

    /// Evaluates V_r(n) = q_r * n + b_r for a rational input n.
    pub fn evaluate(&self, n: &Ratio<i64>) -> Ratio<i64> {
        (&self.coefficient_q * n) + &self.constant_b
    }

    /// Evaluates linear potential difference across macrostep n_k = (3^k * n_0 + c_k) / 2^{A_k}.
    /// Returns V_r(n_k) - V_r(n_0).
    pub fn compute_difference(&self, prefix: &AffinePrefix, n_0: i64) -> Option<Ratio<i64>> {
        let k = prefix.odd_steps;
        let a_k = prefix.total_twos;

        let pow3_k = 3i64.checked_pow(k as u32)?;
        let pow2_a = 1i64.checked_shl(a_k as u32)?;

        let c_k_u64 = prefix
            .constant
            .to_u64_digits()
            .first()
            .cloned()
            .unwrap_or(0);
        if c_k_u64 > i64::MAX as u64 {
            return None;
        }

        let n_0_ratio = Ratio::from_integer(n_0);
        let n_k_numerator = (pow3_k.checked_mul(n_0)?).checked_add(c_k_u64 as i64)?;
        let n_k_ratio = Ratio::new(n_k_numerator, pow2_a);

        let v_start = self.evaluate(&n_0_ratio);
        let v_end = self.evaluate(&n_k_ratio);

        Some(v_end - v_start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use collatz_affine::ValuationWord;

    #[test]
    fn test_krasikov_lagarias_potential() {
        let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
        let prefix = AffinePrefix::from_valuation_word(word).unwrap();
        let potential = LinearPotential::new(1, 1, 0, 1); // V(n) = n

        let diff = potential.compute_difference(&prefix, 39).unwrap();
        assert!(diff < Ratio::from_integer(0)); // Described descent!
    }
}
