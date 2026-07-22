use crate::schema::TailDescentCertificateJson;
use crate::VerificationError;
use collatz_affine::ValuationWord;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Computes the exact analytical critical child valuation a_crit for a valuation prefix.
/// For all child valuations a_k >= a_crit, the integer descent threshold B_{k+1} <= 1.
/// a_crit = max(1, bitlength(3 * c_k + 2^{A_k} + 3^{k+1}) - A_k)
pub fn compute_a_crit(word: &ValuationWord) -> u32 {
    let k = word.len();
    let a_k = word.total_valuation();

    // Recompute c_k
    let mut c_k = BigUint::zero();
    let mut partial_sum = 0u64;
    for &a_i in word.as_slice() {
        c_k = (&c_k * 3u32) + (BigUint::one() << partial_sum);
        partial_sum += a_i as u64;
    }

    // V = 3 * c_k + 2^{A_k} + 3^{k+1}
    let pow3_k_plus_1 = BigUint::from(3u32).pow((k + 1) as u32);
    let pow2_a_k = BigUint::one() << a_k;
    let v = (&c_k * 3u32) + &pow2_a_k + &pow3_k_plus_1;

    let bit_len = v.bits(); // exact bit length of V

    if bit_len <= a_k {
        1
    } else {
        let diff = bit_len - a_k;
        diff.max(1) as u32
    }
}

/// Generates a TailDescentCertificateJson (tail_descent_v1) artifact for a prefix word.
pub fn generate_tail_descent_certificate(word: ValuationWord) -> Result<TailDescentCertificateJson, VerificationError> {
    if word.is_empty() {
        return Err(VerificationError::InvalidValuationWord("Valuation word cannot be empty".to_string()));
    }

    let a_k = word.total_valuation();
    let a_crit = compute_a_crit(&word);

    let mut c_k = BigUint::zero();
    let mut partial_sum = 0u64;
    for &a_i in word.as_slice() {
        c_k = (&c_k * 3u32) + (BigUint::one() << partial_sum);
        partial_sum += a_i as u64;
    }

    Ok(TailDescentCertificateJson {
        schema_version: "tail_descent_v1".to_string(),
        prefix_word: word.as_slice().iter().map(|&a| a as u32).collect(),
        prefix_total_twos: a_k,
        prefix_constant: c_k.to_string(),
        minimum_child_valuation: a_crit,
        proof_bound: "1".to_string(),
    })

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_a_crit_valid() {
        let word = ValuationWord::new(vec![1, 1, 2]).unwrap();
        let a_crit = compute_a_crit(&word);
        assert!(a_crit >= 1);

        let cert = generate_tail_descent_certificate(word).unwrap();
        assert_eq!(cert.schema_version, "tail_descent_v1");
        assert_eq!(cert.proof_bound, "1");
    }
}
