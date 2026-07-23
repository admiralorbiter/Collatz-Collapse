use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

/// Canonical Cover Manifest Schema v1 (cover_v1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverManifestJson {
    pub schema_version: String,
    pub total_leaves: usize,
    pub max_modulus_exponent: u64,
    pub total_scaled_measure: String,
    pub is_exact_cover: bool,
    pub merkle_root_hash: String,
    pub leaves: Vec<CoverLeafJson>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverLeafJson {
    pub valuation_word: Vec<u32>,
    pub total_twos: u64,
    pub starting_residue: String,
    pub modulus_exponent: u64,
    pub valuation_semantics: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverValidationError {
    EmptyCover,
    PrefixOverlap { word_a: Vec<u32>, word_b: Vec<u32> },
    MeasureOverflow,
    IncompleteCover { sum: String, expected: String },
}

/// Generates a CoverManifestJson (cover_v1) from a list of disjoint canonical cylinders.
pub fn build_cover_manifest(
    mut leaves: Vec<CoverLeafJson>,
) -> Result<CoverManifestJson, CoverValidationError> {
    if leaves.is_empty() {
        return Err(CoverValidationError::EmptyCover);
    }

    // 1. Sort lexicographically
    leaves.sort_by(|a, b| a.valuation_word.cmp(&b.valuation_word));

    // 2. Verify antichain property (no leaf is a prefix of another)
    for i in 0..leaves.len() {
        for j in (i + 1)..leaves.len() {
            let a = &leaves[i].valuation_word;
            let b = &leaves[j].valuation_word;
            if b.starts_with(a) {
                return Err(CoverValidationError::PrefixOverlap {
                    word_a: a.clone(),
                    word_b: b.clone(),
                });
            }
        }
    }

    // 3. Find max modulus exponent M
    let max_m = leaves.iter().map(|l| l.modulus_exponent).max().unwrap_or(0);

    // 4. Exact scaled integer 2-adic measure sum: sum_i 1 << (max_m - modulus_exponent_i)
    let mut scaled_sum: u128 = 0;
    for leaf in &leaves {
        if leaf.modulus_exponent > max_m {
            return Err(CoverValidationError::MeasureOverflow);
        }
        let shift = max_m - leaf.modulus_exponent;
        let contribution = 1u128
            .checked_shl(shift as u32)
            .ok_or(CoverValidationError::MeasureOverflow)?;
        scaled_sum = scaled_sum
            .checked_add(contribution)
            .ok_or(CoverValidationError::MeasureOverflow)?;
    }

    let expected_full_cover = 1u128
        .checked_shl(max_m as u32)
        .ok_or(CoverValidationError::MeasureOverflow)?;
    let is_exact_cover = scaled_sum == expected_full_cover;

    // 5. Compute deterministic Merkle-style root hash using std DefaultHasher
    let json_bytes = serde_json::to_vec(&leaves).unwrap_or_default();
    let mut hasher = DefaultHasher::new();
    hasher.write(&json_bytes);
    let merkle_root_hash = format!("{:016x}", hasher.finish());

    Ok(CoverManifestJson {
        schema_version: "cover_v1".to_string(),
        total_leaves: leaves.len(),
        max_modulus_exponent: max_m,
        total_scaled_measure: scaled_sum.to_string(),
        is_exact_cover,
        merkle_root_hash,
        leaves,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cover_manifest_build_valid() {
        let leaves = vec![
            CoverLeafJson {
                valuation_word: vec![1],
                total_twos: 1,
                starting_residue: "1".to_string(),
                modulus_exponent: 1,
                valuation_semantics: "terminal_at_least".to_string(),
            },
            CoverLeafJson {
                valuation_word: vec![2],
                total_twos: 1,
                starting_residue: "3".to_string(),
                modulus_exponent: 1,
                valuation_semantics: "terminal_at_least".to_string(),
            },
        ];

        let manifest = build_cover_manifest(leaves).unwrap();
        assert_eq!(manifest.schema_version, "cover_v1");
        assert_eq!(manifest.total_leaves, 2);
        assert_eq!(manifest.total_scaled_measure, "2");
        assert!(manifest.is_exact_cover);
    }

    #[test]
    fn test_cover_manifest_rejects_overlap() {
        let leaves = vec![
            CoverLeafJson {
                valuation_word: vec![1],
                total_twos: 1,
                starting_residue: "1".to_string(),
                modulus_exponent: 1,
                valuation_semantics: "terminal_at_least".to_string(),
            },
            CoverLeafJson {
                valuation_word: vec![1, 2],
                total_twos: 3,
                starting_residue: "1".to_string(),
                modulus_exponent: 3,
                valuation_semantics: "terminal_at_least".to_string(),
            },
        ];

        assert!(build_cover_manifest(leaves).is_err());
    }
}
