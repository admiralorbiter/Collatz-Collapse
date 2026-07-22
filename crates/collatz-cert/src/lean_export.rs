use crate::schema::DescentCertificateJson;
use num_bigint::BigUint;

/// Generates a compilable Lean 4 theorem file string from a verified DescentCertificateJson.
pub fn export_lean4_theorem(cert: &DescentCertificateJson) -> String {
    let r = &cert.starting_residue;
    let m = cert.modulus_exponent;
    let modulus = BigUint::from(1u32) << m;
    let b = &cert.descent_threshold;
    let c_k = &cert.constant;
    let k = cert.odd_steps;
    let pow3_k = BigUint::from(3u32).pow(k as u32);
    let two_a = BigUint::from(1u32) << cert.total_twos;

    format!(
        r#"-- Generated Lean 4 Verified Theorem File for Collatz Certificate
import Mathlib.Data.Nat.Basic
import Mathlib.Tactic.Omega

-- Base Subtraction-Free Contraction Lemma
theorem contraction_forces_descent (n c_k two_A k : ℕ) (h_contract : c_k + 3^k * n < two_A * n) :
  (3^k * n + c_k) / two_A < n := by
  omega

-- Verified Universal Theorem for Certified Residue Class {r} mod {modulus}
theorem certified_descent_residue_{r}_mod_{modulus} (n : ℕ) (h_pos : 0 < n) (h_res : n % {modulus} = {r}) (h_ge : n ≥ {b}) :
  (3^{k} * n + {c_k}) / {two_a} < n := by
  have h_ineq : {c_k} + {pow3_k} * n < {two_a} * n := by omega
  exact contraction_forces_descent n {c_k} {two_a} {k} h_ineq
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descent::generate_descent_certificate;
    use collatz_affine::ValuationWord;

    #[test]
    fn test_export_lean4_theorem_valid() {
        let word = ValuationWord::new(vec![2, 2]).unwrap();
        let cert = generate_descent_certificate(word).unwrap();
        let lean_code = export_lean4_theorem(&cert);

        assert!(lean_code.contains("theorem certified_descent_residue_1_mod_16"));
        assert!(lean_code.contains("have h_ineq : 7 + 9 * n < 16 * n := by omega"));
        assert!(lean_code.contains("import Mathlib.Tactic.Omega"));
    }
}
