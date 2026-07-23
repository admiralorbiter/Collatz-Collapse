use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExponentSoundnessReport {
    pub e1: u64,
    pub e2: u64,
    pub modulus_bits: u32,
    pub is_mod_congruent: bool,
    pub biguint_congruent: bool,
    pub is_exact_exponent_distinguished: bool,
}

pub struct ExponentSoundnessEngine;

impl ExponentSoundnessEngine {
    /// Compute 3^e mod 2^bits
    pub fn compute_3_pow_mod_2bits(e: u64, bits: u32) -> BigUint {
        let p3 = BigUint::from(3u64).pow(e as u32);
        let m = BigUint::from(2u64).pow(bits);
        &p3 % &m
    }

    /// Run Option A: Mod 16 Insufficiency Test (e1 = 2, e2 = 18)
    pub fn run_mod16_mutation_test() -> ExponentSoundnessReport {
        let e1 = 2u64;
        let e2 = 18u64;
        let bits = 16u32;

        let val1 = Self::compute_3_pow_mod_2bits(e1, bits);
        let val2 = Self::compute_3_pow_mod_2bits(e2, bits);

        let is_mod_congruent = (e1 % 16) == (e2 % 16);
        let biguint_congruent = val1 == val2;

        ExponentSoundnessReport {
            e1,
            e2,
            modulus_bits: bits,
            is_mod_congruent,
            biguint_congruent,
            is_exact_exponent_distinguished: !biguint_congruent,
        }
    }

    /// Run Option B: Mod 64 Insufficiency Test (e1 = 2, e2 = 66)
    pub fn run_mod64_mutation_test() -> ExponentSoundnessReport {
        let e1 = 2u64;
        let e2 = 66u64;
        let bits = 16u32;

        let val1 = Self::compute_3_pow_mod_2bits(e1, bits);
        let val2 = Self::compute_3_pow_mod_2bits(e2, bits);

        let is_mod_congruent = (e1 % 64) == (e2 % 64);
        let biguint_congruent = val1 == val2;

        ExponentSoundnessReport {
            e1,
            e2,
            modulus_bits: bits,
            is_mod_congruent,
            biguint_congruent,
            is_exact_exponent_distinguished: !biguint_congruent,
        }
    }

    /// Compute exact polynomial expansion multiplicities for (1 + x + ... + x^8)^d
    /// Returns 8d + 1 exact exponent layer pairs (e = 6d + 3s, multiplicity)
    pub fn compute_exponent_layer_multiplicities(depth: usize) -> Vec<(u64, BigUint)> {
        let mut poly = vec![BigUint::from(1u64)];
        let base = vec![BigUint::from(1u64); 9]; // 1 + x + x^2 + ... + x^8

        for _ in 0..depth {
            let mut next_poly = vec![BigUint::from(0u64); poly.len() + 8];
            for (i, c1) in poly.iter().enumerate() {
                for (j, c2) in base.iter().enumerate() {
                    next_poly[i + j] += c1 * c2;
                }
            }
            poly = next_poly;
        }

        let mut layers = Vec::new();
        for (s, mult) in poly.into_iter().enumerate() {
            let e = 6 * (depth as u64) + 3 * (s as u64);
            layers.push((e, mult));
        }

        layers
    }
}
