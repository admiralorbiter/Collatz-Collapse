use collatz_affine::ValuationWord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WordForcingStatus {
    /// Source guard forces exact valuation word sequence
    ExactWord,
    /// Source guard forces terminal-at-least valuation prefix
    TerminalAtLeast,
    /// Valuation word sequence is not forced
    NotForced,
}

#[derive(Debug, Clone)]
pub struct CylinderImage {
    pub source_residue: u64,
    pub source_exponent: u32,
    pub valuation_word: Vec<u32>,
    pub total_valuation: u32,
    pub odd_steps: u32,
    pub affine_constant: u128,
    pub target_base_image: u128,
    pub quotient_multiplier: u128,
    pub required_quotient_bits: u32,
}

impl CylinderImage {
    /// Correct quotient-bit calculation: h_required = max(0, q - m + A - h_curr)
    pub fn compute_required_quotient_bits(
        m: u32,
        h_curr: u32,
        total_valuation_a: u32,
        target_requested_q: u32,
    ) -> u32 {
        let current_precision = m + h_curr;
        (target_requested_q + total_valuation_a).saturating_sub(current_precision)
    }

    pub fn compute_exact_image(
        source_residue: u64,
        source_exponent: u32,
        word: &ValuationWord,
        requested_target_q: u32,
    ) -> Result<Self, String> {
        let k = word.len() as u32;
        let vals: Vec<u32> = word.as_slice().iter().map(|&v| v as u32).collect();
        let total_a: u32 = vals.iter().sum();

        let mut c_k: u128 = 0;
        let mut a_curr: u32 = 0;
        for &v in &vals {
            c_k = 3 * c_k + (1 << a_curr);
            a_curr += v;
        }

        let pow3_k = 3u128.pow(k);
        let src_r = source_residue as u128;
        let num = pow3_k * src_r + c_k;
        let two_a = 1u128 << total_a;

        if !num.is_multiple_of(two_a) {
            return Err(format!(
                "Exact division failure: (3^{k} * {src_r} + {c_k}) mod 2^{total_a} = {} != 0",
                num % two_a
            ));
        }

        let target_base_image = num / two_a;
        let quotient_multiplier = pow3_k;
        let required_quotient_bits =
            Self::compute_required_quotient_bits(source_exponent, 0, total_a, requested_target_q);

        Ok(Self {
            source_residue,
            source_exponent,
            valuation_word: vals.to_vec(),
            total_valuation: total_a,
            odd_steps: k,
            affine_constant: c_k,
            target_base_image,
            quotient_multiplier,
            required_quotient_bits,
        })
    }
}

pub struct SemanticGate;

impl SemanticGate {
    pub fn verify_word_forcing(
        _residue: u64,
        modulus_exponent: u32,
        word: &ValuationWord,
    ) -> WordForcingStatus {
        let vals: Vec<u32> = word.as_slice().iter().map(|&v| v as u32).collect();
        let total_a: u32 = vals.iter().sum();
        let required_exact_exponent = total_a + 1;

        if modulus_exponent >= required_exact_exponent {
            WordForcingStatus::ExactWord
        } else if modulus_exponent >= total_a {
            WordForcingStatus::TerminalAtLeast
        } else {
            WordForcingStatus::NotForced
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quotient_bit_formula_correctness() {
        // Source m=5 (mod 32), h_curr=0, A=4, requested target q=5 (mod 32)
        // h_required = max(0, 5 - 5 + 4 - 0) = 4
        let h = CylinderImage::compute_required_quotient_bits(5, 0, 4, 5);
        assert_eq!(h, 4);

        // Source m=6 (mod 64), h_curr=0, A=5, requested target q=5 (mod 32)
        // h_required = max(0, 5 - 6 + 5 - 0) = 4
        let h2 = CylinderImage::compute_required_quotient_bits(6, 0, 5, 5);
        assert_eq!(h2, 4);
    }

    #[test]
    fn test_exact_word_forcing_status() {
        let w1 = ValuationWord::new(vec![1, 1, 2]).unwrap(); // A = 4
        assert_eq!(
            SemanticGate::verify_word_forcing(7, 5, &w1),
            WordForcingStatus::ExactWord
        );
        assert_eq!(
            SemanticGate::verify_word_forcing(7, 4, &w1),
            WordForcingStatus::TerminalAtLeast
        );
        assert_eq!(
            SemanticGate::verify_word_forcing(7, 3, &w1),
            WordForcingStatus::NotForced
        );
    }
}
