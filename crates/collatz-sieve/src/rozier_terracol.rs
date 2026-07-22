use collatz_affine::ValuationWord;

/// Rozier-Terracol (2025) Paradoxical Sequence Benchmark Suite.
/// Profiles valuation words exhibiting multiplicative expansion (growth debt D_k > 0)
/// but possessing localized descent thresholds or additive contractions.
pub struct RozierTerracolBenchmarkSuite;

impl RozierTerracolBenchmarkSuite {
    /// Known paradoxical valuation word sequences from literature.
    pub fn benchmark_words() -> Vec<ValuationWord> {
        vec![
            // High-debt expansion prefixes (a_i = 1 dominated)
            ValuationWord::new(vec![1, 1, 1, 1, 1]).unwrap(),
            ValuationWord::new(vec![1, 1, 1, 1, 2]).unwrap(),
            ValuationWord::new(vec![1, 1, 2, 1, 1, 1]).unwrap(),
            ValuationWord::new(vec![1, 1, 1, 2, 1, 1]).unwrap(),
        ]
    }

    /// Evaluates whether a valuation word exhibits paradoxical growth behavior:
    /// Multiplicative expansion 3^k > 2^{A_k} (debt D_k > 0).
    pub fn is_paradoxical(word: &ValuationWord) -> bool {
        let k = word.len();
        let a_k = word.total_valuation();
        let k_float = k as f64;
        let a_float = a_k as f64;

        k_float * 3.0f64.log2() > a_float
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rozier_terracol_suite() {
        let words = RozierTerracolBenchmarkSuite::benchmark_words();
        assert!(!words.is_empty());
        // All-ones sequence (1, 1, 1, 1, 1) has 3^5 = 243 > 2^5 = 32 -> paradoxical expansion!
        assert!(RozierTerracolBenchmarkSuite::is_paradoxical(&words[0]));
    }
}
