use collatz_affine::{SymbolicWordData, ValuationWord};
use std::collections::HashMap;

/// Realizability status for infinite symbolic streams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfiniteRealizationStatus {
    UltimatelyPeriodicNonpositive,
    NonordinaryTwoAdicWitness,
    PositiveInfiniteWitness,
    AperiodicUnresolved,
}

/// Zero-lift continuation chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroLiftChain {
    pub start_word: ValuationWord,
    pub chain_length: usize,
    pub words: Vec<ValuationWord>,
}

/// Lift-digit analysis engine.
pub struct LiftDigitEngine;

impl LiftDigitEngine {
    /// Analyzes a set of enumerated symbolic words and builds zero-lift continuation chains.
    pub fn analyze_zero_lift_chains(
        word_data: &[SymbolicWordData],
    ) -> (Vec<ZeroLiftChain>, HashMap<usize, (usize, usize)>) {
        // Map word -> SymbolicWordData
        let mut map: HashMap<Vec<u32>, &SymbolicWordData> = HashMap::new();
        for data in word_data {
            map.insert(data.word.elements().to_vec(), data);
        }

        // Depth -> (total_children, zero_lift_children)
        let mut depth_stats: HashMap<usize, (usize, usize)> = HashMap::new();

        for data in word_data {
            if let Some(is_zero) = data.is_zero_lift_from_parent {
                let depth = data.word.elements().len() / 3; // depth in u/v blocks
                let entry = depth_stats.entry(depth).or_insert((0, 0));
                entry.0 += 1;
                if is_zero {
                    entry.1 += 1;
                }
            }
        }

        // Find maximal zero-lift chains
        let mut chains = Vec::new();
        let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
        let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();

        for data in word_data {
            if data.is_zero_lift_from_parent == Some(true) || data.word.elements().len() <= 6 {
                // Explore forward zero-lift chain
                let mut current_words = vec![data.word.clone()];
                let mut curr_data = data;

                loop {
                    let mut found_zero_child = None;
                    for child_symbol in &[u_word.clone(), v_word.clone()] {
                        let mut child_elems = curr_data.word.elements();
                        child_elems.extend_from_slice(&child_symbol.elements());
                        if let Some(child_data) = map.get(&child_elems) {
                            if child_data.is_zero_lift_from_parent == Some(true) {
                                found_zero_child = Some(*child_data);
                                break;
                            }
                        }
                    }

                    if let Some(child) = found_zero_child {
                        current_words.push(child.word.clone());
                        curr_data = child;
                    } else {
                        break;
                    }
                }

                if current_words.len() >= 2 {
                    chains.push(ZeroLiftChain {
                        start_word: data.word.clone(),
                        chain_length: current_words.len(),
                        words: current_words,
                    });
                }
            }
        }

        (chains, depth_stats)
    }
}
