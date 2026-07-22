use collatz_affine::{AffineDiagnostics, AffinePrefix, ValuationWord};
use collatz_sieve::{DescentSieve, PrefixSieve, PrefixState, SieveResult};
use smallvec::SmallVec;

/// Candidate path in beam search with multi-objective scoring.
#[derive(Debug, Clone)]
pub struct BeamCandidate {
    pub word: ValuationWord,
    pub prefix: AffinePrefix,
    pub growth_debt: f64,
    pub entropy: f64,
    pub pole_distance_bits: u32,
    pub combined_score: f64,
}

/// Multi-Objective Diversity-Preserving Beam Search Engine.
/// Score = alpha * growth_debt + beta * entropy + gamma * pole_distance_bits
pub struct DiversityBeamSearch {
    pub beam_width: usize,
    pub max_depth: usize,
    pub alpha_growth: f64,
    pub beta_entropy: f64,
    pub gamma_pole: f64,
}

impl Default for DiversityBeamSearch {
    fn default() -> Self {
        Self {
            beam_width: 100,
            max_depth: 20,
            alpha_growth: 1.0,
            beta_entropy: 0.5,
            gamma_pole: 0.2,
        }
    }
}

impl DiversityBeamSearch {
    pub fn new(beam_width: usize, max_depth: usize) -> Self {
        Self {
            beam_width,
            max_depth,
            ..Default::default()
        }
    }

    /// Computes multi-objective combined score for a valuation prefix.
    pub fn score_candidate(&self, word: &ValuationWord, prefix: &AffinePrefix) -> f64 {
        let diag = AffineDiagnostics::from_prefix(prefix);
        let debt = diag.growth_debt;

        // Entropy heuristic based on valuation frequencies
        let counts = word.as_slice().iter().fold([0u32; 16], |mut acc, &val| {
            let idx = (val as usize).min(15);
            acc[idx] += 1;
            acc
        });

        let total = word.len() as f64;
        let entropy = counts.iter().filter(|&&c| c > 0).map(|&c| {
            let p = c as f64 / total;
            -p * p.log2()
        }).sum::<f64>();

        let pole_bits = diag.pole_distance_bits as f64;

        (self.alpha_growth * debt) + (self.beta_entropy * entropy) + (self.gamma_pole * pole_bits)
    }

    /// Runs adversarial beam search starting from initial root word.
    pub fn search(&self, initial_word: ValuationWord) -> Vec<BeamCandidate> {
        let mut current_beam = Vec::new();
        if let Ok(prefix) = AffinePrefix::from_valuation_word(initial_word.clone()) {
            let score = self.score_candidate(&initial_word, &prefix);
            let diag = AffineDiagnostics::from_prefix(&prefix);
            current_beam.push(BeamCandidate {
                word: initial_word,
                prefix,
                growth_debt: diag.growth_debt,
                entropy: 0.0,
                pole_distance_bits: diag.pole_distance_bits,
                combined_score: score,
            });
        }

        let descent_sieve = DescentSieve;

        for _depth in 0..self.max_depth {
            let mut next_candidates = Vec::new();

            for parent in &current_beam {
                // Expand child valuations a_k in 1..=8
                for a_k in 1..=8u32 {
                    let mut child_vec = parent.word.as_slice().to_vec();
                    child_vec.push(a_k as u8);

                    if let Ok(child_word) = ValuationWord::new(child_vec.clone()) {
                        if let Ok(child_prefix) = AffinePrefix::from_valuation_word(child_word.clone()) {
                            let state = PrefixState {
                                valuations: SmallVec::from_slice(&child_vec),
                                affine: child_prefix.clone(),
                                growth_debt: parent.growth_debt,
                            };

                            // Prune if certified descending
                            if matches!(descent_sieve.evaluate(&state), SieveResult::Reject { .. }) {
                                continue;
                            }

                            let score = self.score_candidate(&child_word, &child_prefix);
                            let diag = AffineDiagnostics::from_prefix(&child_prefix);

                            next_candidates.push(BeamCandidate {
                                word: child_word,
                                prefix: child_prefix,
                                growth_debt: diag.growth_debt,
                                entropy: 0.0,
                                pole_distance_bits: diag.pole_distance_bits,
                                combined_score: score,
                            });
                        }
                    }
                }
            }

            if next_candidates.is_empty() {
                break;
            }

            // Sort by combined score descending and retain top beam_width
            next_candidates.sort_by(|a, b| b.combined_score.partial_cmp(&a.combined_score).unwrap_or(std::cmp::Ordering::Equal));
            next_candidates.truncate(self.beam_width);
            current_beam = next_candidates;
        }

        current_beam
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diversity_beam_search() {
        let searcher = DiversityBeamSearch::new(10, 5);
        let word = ValuationWord::new(vec![1]).unwrap();
        let candidates = searcher.search(word);

        assert!(!candidates.is_empty());
        assert!(candidates.len() <= 10);
    }
}
