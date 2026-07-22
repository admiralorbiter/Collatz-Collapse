#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnCycle {
    pub base_state_id: String,
    pub word_sequence: Vec<u32>,
    pub total_valuation_a: u32,
    pub odd_steps_k: u32,
}

pub struct NonCommutingCycleAnalyzer;

impl NonCommutingCycleAnalyzer {
    /// Finite words u and v commute (u * v == v * u) if and only if they are powers of a common primitive word.
    pub fn do_words_commute(u: &[u32], v: &[u32]) -> bool {
        let mut uv = u.to_vec();
        uv.extend_from_slice(v);

        let mut vu = v.to_vec();
        vu.extend_from_slice(u);

        uv == vu
    }

    /// Verifies non-commuting cycle gate for two return cycles based at the same state Q_s.
    pub fn verify_non_commuting_target_gate(
        cycle_u: &ReturnCycle,
        cycle_v: &ReturnCycle,
    ) -> Result<(), String> {
        if cycle_u.base_state_id != cycle_v.base_state_id {
            return Err(format!(
                "Base State Mismatch: cycle_u based at {}, cycle_v based at {}",
                cycle_u.base_state_id, cycle_v.base_state_id
            ));
        }

        if Self::do_words_commute(&cycle_u.word_sequence, &cycle_v.word_sequence) {
            return Err(format!(
                "Commutative Cycle Rejection: u={:?} and v={:?} commute (uv == vu), indicating single periodic word repetition",
                cycle_u.word_sequence, cycle_v.word_sequence
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_words_commute() {
        // [1, 2] and [1, 2, 1, 2] commute (powers of [1, 2])
        assert!(NonCommutingCycleAnalyzer::do_words_commute(
            &[1, 2],
            &[1, 2, 1, 2]
        ));

        // [1, 1, 2] and [1, 2, 2] do NOT commute
        assert!(!NonCommutingCycleAnalyzer::do_words_commute(
            &[1, 1, 2],
            &[1, 2, 2]
        ));
    }

    #[test]
    fn test_non_commuting_target_gate() {
        let u = ReturnCycle {
            base_state_id: "Q1".to_string(),
            word_sequence: vec![1, 1, 2],
            total_valuation_a: 4,
            odd_steps_k: 3,
        };
        let v = ReturnCycle {
            base_state_id: "Q1".to_string(),
            word_sequence: vec![1, 2, 2],
            total_valuation_a: 5,
            odd_steps_k: 3,
        };

        assert!(NonCommutingCycleAnalyzer::verify_non_commuting_target_gate(&u, &v).is_ok());
    }
}
