use collatz_affine::ValuationWord;
use std::collections::{HashMap, HashSet};

/// Minimal Deterministic Finite Automaton (DFA) representing regular language of unresolved valuation words.
#[derive(Debug, Clone)]
pub struct UnresolvedAutomaton {
    pub num_states: usize,
    pub transitions: HashMap<(usize, u8), usize>,
    pub accepting_states: HashSet<usize>,
}

#[derive(Debug, Clone)]
pub struct PumpableCycle {
    pub state_cycle: Vec<usize>,
    pub valuation_cycle: Vec<u8>,
    pub total_valuation: u32,
    pub cycle_length: usize,
    pub average_valuation: f64,
    pub is_expansion_cycle: bool,
}

impl UnresolvedAutomaton {
    /// Builds a minimal DFA from a collection of unresolved valuation words.
    pub fn build_from_words(words: &[ValuationWord]) -> Self {
        let mut state_counter = 0;
        let mut transitions = HashMap::new();
        let mut accepting_states = HashSet::new();

        // Root state = 0
        let root = 0;
        state_counter += 1;

        let mut prefix_map: HashMap<Vec<u8>, usize> = HashMap::new();
        prefix_map.insert(Vec::new(), root);

        for word in words {
            let slice = word.as_slice();
            let mut curr_prefix = Vec::new();
            let mut curr_state = root;

            for &val in slice {
                curr_prefix.push(val);

                if let Some(&next_state) = prefix_map.get(&curr_prefix) {
                    curr_state = next_state;
                } else {
                    let next_state = state_counter;
                    state_counter += 1;
                    transitions.insert((curr_state, val), next_state);
                    prefix_map.insert(curr_prefix.clone(), next_state);
                    curr_state = next_state;
                }
            }

            accepting_states.insert(curr_state);
        }

        Self {
            num_states: state_counter,
            transitions,
            accepting_states,
        }
    }

    /// Detects pumpable cycles (SCCs) in the automaton and calculates their average valuation.
    pub fn detect_pumpable_cycles(&self) -> Vec<PumpableCycle> {
        let mut cycles = Vec::new();
        let log2_3 = 3.0f64.log2();

        // Simple DFS cycle detector
        for &start_state in &self.accepting_states {
            let mut path_states = vec![start_state];
            let mut path_vals = Vec::new();
            let mut visited = HashSet::new();
            visited.insert(start_state);

            self.dfs_cycles(
                start_state,
                start_state,
                &mut path_states,
                &mut path_vals,
                &mut visited,
                &mut cycles,
                log2_3,
            );
        }

        cycles
    }

    fn dfs_cycles(
        &self,
        curr_state: usize,
        target_state: usize,
        path_states: &mut Vec<usize>,
        path_vals: &mut Vec<u8>,
        visited: &mut HashSet<usize>,
        cycles: &mut Vec<PumpableCycle>,
        log2_3: f64,
    ) {
        for val in 1..=8u8 {
            if let Some(&next_state) = self.transitions.get(&(curr_state, val)) {
                if next_state == target_state && !path_vals.is_empty() {
                    // Cycle detected!
                    let total_a: u32 = path_vals.iter().map(|&v| v as u32).sum();
                    let len = path_vals.len();
                    let avg_a = total_a as f64 / len as f64;
                    let is_expansion = avg_a < log2_3;

                    cycles.push(PumpableCycle {
                        state_cycle: path_states.clone(),
                        valuation_cycle: path_vals.clone(),
                        total_valuation: total_a,
                        cycle_length: len,
                        average_valuation: avg_a,
                        is_expansion_cycle: is_expansion,
                    });
                } else if !visited.contains(&next_state) && path_states.len() < 10 {
                    visited.insert(next_state);
                    path_states.push(next_state);
                    path_vals.push(val);

                    self.dfs_cycles(
                        next_state,
                        target_state,
                        path_states,
                        path_vals,
                        visited,
                        cycles,
                        log2_3,
                    );

                    path_vals.pop();
                    path_states.pop();
                    visited.remove(&next_state);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automata_extraction() {
        let words = vec![
            ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap(),
            ValuationWord::new(vec![2, 3, 3, 2, 1]).unwrap(),
        ];

        let dfa = UnresolvedAutomaton::build_from_words(&words);
        assert!(dfa.num_states > 0);
        assert_eq!(dfa.accepting_states.len(), 2);
    }
}
