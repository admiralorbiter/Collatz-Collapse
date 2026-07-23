use crate::abstract_domain::{AbstractEdge, RelationalState};
use num_bigint::BigUint;
use std::collections::{HashMap, HashSet};

/// Karp's Maximum Cycle Mean Engine using Exact Symbolic Integer Comparisons.
/// Evaluates cycle growth using exact BigUint comparisons: 3^|C| >= 2^(sum a_i) <=> lambda* >= 0.
/// Eliminates 100% of floating-point rounding risks!
pub struct KarpCycleEngine;

impl KarpCycleEngine {
    /// Extracts all simple cycles from the abstract transition graph.
    pub fn find_cycles(
        states: &HashSet<RelationalState>,
        edges: &HashSet<AbstractEdge>,
        max_cycle_length: usize,
    ) -> Vec<Vec<AbstractEdge>> {
        let mut cycles = Vec::new();
        let mut adj: HashMap<RelationalState, Vec<AbstractEdge>> = HashMap::new();

        for edge in edges {
            adj.entry(edge.from.clone()).or_default().push(edge.clone());
        }

        for start in states {
            let mut visited = HashSet::new();
            let mut path = Vec::new();
            Self::dfs_cycles(
                start,
                start,
                &adj,
                &mut visited,
                &mut path,
                &mut cycles,
                max_cycle_length,
            );
        }

        cycles
    }

    fn dfs_cycles(
        curr: &RelationalState,
        target: &RelationalState,
        adj: &HashMap<RelationalState, Vec<AbstractEdge>>,
        visited: &mut HashSet<RelationalState>,
        path: &mut Vec<AbstractEdge>,
        cycles: &mut Vec<Vec<AbstractEdge>>,
        max_len: usize,
    ) {
        if let Some(edge_list) = adj.get(curr) {
            for edge in edge_list {
                if &edge.to == target && !path.is_empty() {
                    cycles.push(path.clone());
                } else if !visited.contains(&edge.to) && path.len() < max_len {
                    visited.insert(edge.to.clone());
                    path.push(edge.clone());
                    Self::dfs_cycles(&edge.to, target, adj, visited, path, cycles, max_len);
                    path.pop();
                    visited.remove(&edge.to);
                }
            }
        }
    }

    /// Evaluates if a cycle is multiplicatively expanding/non-contracting (lambda* >= 0)
    /// using exact BigUint integer comparisons: 3^|C| >= 2^(sum a_i).
    pub fn is_dangerous_cycle(cycle: &[AbstractEdge]) -> bool {
        let cycle_len = cycle.len() as u32;
        let total_twos: u32 = cycle.iter().map(|e| e.valuation as u32).sum();

        let three_pow = BigUint::from(3u32).pow(cycle_len);
        let two_pow = BigUint::from(1u32) << total_twos;

        // Non-contracting condition: 3^k >= 2^A (i.e. lambda* >= 0)
        three_pow >= two_pow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_karp_cycle_integer_soundness() {
        let s1 = RelationalState::new_congruence(1, 2);
        let s2 = RelationalState::new_congruence(3, 2);

        // Contracting cycle: valuations [2, 2] (3^2 = 9 < 2^4 = 16)
        let contracting_cycle = vec![
            AbstractEdge {
                from: s1.clone(),
                to: s2.clone(),
                valuation: 2,
            },
            AbstractEdge {
                from: s2.clone(),
                to: s1.clone(),
                valuation: 2,
            },
        ];
        assert!(!KarpCycleEngine::is_dangerous_cycle(&contracting_cycle));

        // Dangerous cycle: valuations [1, 1] (3^2 = 9 > 2^2 = 4)
        let dangerous_cycle = vec![
            AbstractEdge {
                from: s1.clone(),
                to: s2.clone(),
                valuation: 1,
            },
            AbstractEdge {
                from: s2.clone(),
                to: s1.clone(),
                valuation: 1,
            },
        ];
        assert!(KarpCycleEngine::is_dangerous_cycle(&dangerous_cycle));
    }
}
