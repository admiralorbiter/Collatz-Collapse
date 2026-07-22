use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PathSemanticsCertificateJson {
    pub schema_version: String,
    pub path_id: String,
    pub valuation_words: Vec<Vec<u32>>,
    pub total_odd_steps_k: u32,
    pub total_valuation_a: u32,
    pub composite_constant_c: String,
    pub exact_source_cylinder_residue: String,
    pub exact_source_modulus_exponent: u32,
    pub nonemptiness_witness: String,
    pub intermediate_guards_verified: bool,
    pub return_image_residue: String,
    pub return_modulus_exponent: u32,
    pub universal_path_inclusion_proved: bool,
}

pub struct PathSemanticsEngine;

impl PathSemanticsEngine {
    /// Composes affine maps for a sequence of macrosteps w_1, w_2, ..., w_j
    /// c_new = 3^(k_step) * c_prev + 2^(A_prev) * c_step
    pub fn compose_path_affine_map(words: &[Vec<u32>]) -> (u32, u32, u128) {
        let mut total_k = 0u32;
        let mut total_a = 0u32;
        let mut composite_c = 0u128;

        for word in words {
            let k_step = word.len() as u32;
            let mut c_step = 0u128;
            let mut a_step = 0u32;
            for &v in word {
                c_step = 3 * c_step + (1 << a_step);
                a_step += v;
            }

            composite_c = composite_c * (3u128.pow(k_step)) + (1u128 << total_a) * c_step;
            total_k += k_step;
            total_a += a_step;
        }

        (total_k, total_a, composite_c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_affine_composition() {
        let words = vec![vec![1, 1, 2], vec![1, 2, 2]];
        let (k, a, c) = PathSemanticsEngine::compose_path_affine_map(&words);
        assert_eq!(k, 6);
        assert_eq!(a, 9);
        assert_eq!(c, 881);

        let words_w2 = vec![vec![1, 1, 2], vec![1, 2, 2], vec![1, 1, 2], vec![1, 2, 2]];
        let (k2, a2, c2) = PathSemanticsEngine::compose_path_affine_map(&words_w2);
        assert_eq!(k2, 12);
        assert_eq!(a2, 18);
        assert_eq!(c2, 1093321);
    }
}
