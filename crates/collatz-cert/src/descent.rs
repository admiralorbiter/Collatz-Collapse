use crate::schema::DescentCertificateJson;
use collatz_affine::{AffinePrefix, ValuationWord};
use collatz_core::odd_step;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Generates a valid DescentCertificateJson for a given valuation word if it satisfies multiplicative contraction.
pub fn generate_descent_certificate(word: ValuationWord) -> Result<DescentCertificateJson, String> {
    let prefix = AffinePrefix::from_valuation_word(word.clone()).map_err(|e| e.to_string())?;

    if !prefix.is_multiplicative_contracting() {
        return Err("Valuation word is not multiplicatively contracting (2^A_k <= 3^k)".to_string());
    }

    let threshold = prefix.compute_descent_threshold().ok_or("Failed to compute descent threshold")?;
    let modulus = BigUint::one() << prefix.modulus_exponent;

    // Independently generate and verify all positive exceptions e < B where e == starting_residue mod 2^A_k
    let mut exceptions = Vec::new();
    let mut current = prefix.starting_residue.clone();

    // If starting_residue is 0 (even), add modulus to find smallest positive odd representative
    if current.is_zero() || (&current & BigUint::one()).is_zero() {
        current += &modulus;
    }

    let k = prefix.odd_steps;
    while current < threshold {
        // Verify concrete descent for this exception
        let mut val = current.clone();
        let mut descended = false;

        for _ in 0..k {
            let step = odd_step(&val).map_err(|e| e.to_string())?;
            val = step.to;
            if val < current || val.is_one() {
                descended = true;
                break;
            }
        }

        if !descended {
            return Err(format!("Exception {} failed concrete descent verification", current));
        }

        exceptions.push(current.to_string());
        current += &modulus;
    }

    let val_u32: Vec<u32> = word.as_slice().iter().map(|&a| a as u32).collect();

    Ok(DescentCertificateJson {
        schema_version: "descent_v1".to_string(),
        valuation_word: val_u32,
        total_twos: prefix.total_twos,
        odd_steps: prefix.odd_steps,
        starting_residue: prefix.starting_residue.to_string(),
        modulus_exponent: prefix.modulus_exponent,
        constant: prefix.constant.to_string(),
        descent_threshold: threshold.to_string(),
        checked_exceptions: exceptions,
    })
}
