use num_bigint::BigUint;
use num_rational::Ratio;

/// Geometric 2-Adic Baseline & Negative-Binomial Distribution Analyzer.
/// Under normalized Haar measure on odd 2-adics, valuation steps a_i ~ Geom(1/2) with P(a_i = v) = 2^-v.
/// Total valuation A_k = sum a_i follows the Negative Binomial distribution:
/// P(A_k = s) = (s-1 C k-1) * 2^-s, s >= k.
pub struct NegativeBinomialBaseline;

impl NegativeBinomialBaseline {
    /// Computes combination nCr = n! / (r! * (n-r)!) as BigUint.
    pub fn combinations(n: u64, r: u64) -> BigUint {
        if r > n {
            return BigUint::ZERO;
        }
        if r == 0 || r == n {
            return BigUint::from(1u32);
        }
        let k = r.min(n - r);
        let mut num = BigUint::from(1u32);
        let mut den = BigUint::from(1u32);
        for i in 1..=k {
            num *= BigUint::from(n - k + i);
            den *= BigUint::from(i);
        }
        num / den
    }

    /// Computes exact probability P(A_k = s) = (s-1 C k-1) * 2^-s as a Rational Ratio.
    pub fn probability_mass(k: u64, s: u64) -> Ratio<BigUint> {
        if s < k {
            return Ratio::from_integer(BigUint::ZERO);
        }
        let comb = Self::combinations(s - 1, k - 1);
        let denom = BigUint::from(1u32) << s;
        Ratio::new(comb, denom)
    }

    /// Computes cumulative non-contracting mass for valuation depth k where A_k <= floor(k * log2(3)).
    /// For k=20, max non-contracting valuation s = floor(20 * log2(3)) = 31.
    pub fn non_contracting_mass_k20() -> Ratio<BigUint> {
        let mut sum = Ratio::from_integer(BigUint::ZERO);
        for s in 20..=31u64 {
            sum += Self::probability_mass(20, s);
        }
        sum
    }

    /// Computes cumulative contracting baseline mass for depth k=20 (1.0 - non_contracting_mass).
    pub fn contracting_baseline_mass_k20() -> Ratio<BigUint> {
        Ratio::from_integer(BigUint::from(1u32)) - Self::non_contracting_mass_k20()
    }

    /// Kraft-McMillan Verifier Invariant Check: Sum 2^{-A(w)} <= 1 for exact prefix-free cylinders.
    pub fn verify_kraft_inequality(leaf_total_twos: &[u64]) -> bool {
        let mut sum = Ratio::from_integer(BigUint::ZERO);
        for &a_k in leaf_total_twos {
            let term = Ratio::new(BigUint::from(1u32), BigUint::from(1u32) << a_k);
            sum += term;
        }
        sum <= Ratio::from_integer(BigUint::from(1u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::ToPrimitive;

    #[test]
    fn test_negative_binomial_k20_exact() {
        let non_contracting_ratio = NegativeBinomialBaseline::non_contracting_mass_k20();
        let non_contracting_f64 = non_contracting_ratio.to_f64().unwrap();

        // Exact probability for s=20..31 is 0.0748064...
        assert!((non_contracting_f64 - 0.0748064).abs() < 1e-6);

        let contracting_ratio = NegativeBinomialBaseline::contracting_baseline_mass_k20();
        let contracting_f64 = contracting_ratio.to_f64().unwrap();

        // Theoretical contracting baseline mass is 92.5194...%
        assert!((contracting_f64 - 0.9251936).abs() < 1e-6);

        // Audit Gap vs Empirical Broad Union (90.2621%) = 2.2573%
        let empirical_union = 0.902621f64;
        let audit_gap = contracting_f64 - empirical_union;
        assert!((audit_gap - 0.022573).abs() < 1e-4);
    }

    #[test]
    fn test_kraft_inequality_valid() {
        let leaves = vec![1, 2, 3, 3]; // Sum 2^-1 + 2^-2 + 2^-3 + 2^-3 = 1.0
        assert!(NegativeBinomialBaseline::verify_kraft_inequality(&leaves));
    }
}
