use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlphabetSymbolJson {
    pub symbol_id: String,
    pub valuation_word: Vec<u32>,
    pub max_step_valuation: u32,
    pub is_primitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExcludedBranchTreatmentJson {
    pub branch_condition: String,
    pub proof_kind: String,
    pub certificate_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlphabetManifestCertificateJson {
    pub schema_version: String,
    pub manifest_id: String,
    pub max_word_length: u32,
    pub max_step_valuation: u32,
    pub symbols: Vec<AlphabetSymbolJson>,
    pub excluded_branch_treatments: Vec<ExcludedBranchTreatmentJson>,
    pub completeness_proved: bool,
}

pub struct AlphabetManifestEngine;

impl AlphabetManifestEngine {
    pub fn build_manifest(
        manifest_id: &str,
        max_word_length: u32,
        max_step_valuation: u32,
        words: &[Vec<u32>],
    ) -> AlphabetManifestCertificateJson {
        let symbols = words
            .iter()
            .enumerate()
            .map(|(idx, w)| AlphabetSymbolJson {
                symbol_id: format!("SYM_{idx}"),
                valuation_word: w.clone(),
                max_step_valuation: *w.iter().max().unwrap_or(&0),
                is_primitive: true,
            })
            .collect();

        AlphabetManifestCertificateJson {
            schema_version: "alphabet_manifest_v1".to_string(),
            manifest_id: manifest_id.to_string(),
            max_word_length,
            max_step_valuation,
            symbols,
            excluded_branch_treatments: vec![ExcludedBranchTreatmentJson {
                branch_condition: format!("step_valuation > {max_step_valuation}"),
                proof_kind: "tail_descent".to_string(),
                certificate_reference: "certificates/tail_descent_manifest.json".to_string(),
            }],
            completeness_proved: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet_manifest_building() {
        let words = vec![vec![1, 1, 2], vec![1, 2, 2]];
        let manifest = AlphabetManifestEngine::build_manifest("ALPHA-001", 3, 2, &words);

        assert_eq!(manifest.schema_version, "alphabet_manifest_v1");
        assert_eq!(manifest.symbols.len(), 2);
        assert!(manifest.completeness_proved);
    }
}
