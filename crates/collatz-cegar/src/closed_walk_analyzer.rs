#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportedClosedWalk {
    pub base_state_id: String,
    pub path_a: Vec<Vec<u32>>,
    pub path_b: Vec<Vec<u32>>,
    pub cycle_u: Vec<Vec<u32>>,
    pub cycle_v: Vec<Vec<u32>>,
    pub transported_walk_u: Vec<Vec<u32>>,
    pub transported_walk_v: Vec<Vec<u32>>,
}

pub struct ClosedWalkAnalyzer;

impl ClosedWalkAnalyzer {
    /// Concatenates vector of word slices into a flattened word sequence
    pub fn flatten_words(words: &[Vec<u32>]) -> Vec<u32> {
        words.iter().flat_map(|w| w.iter().copied()).collect()
    }

    /// Tests exact non-commutativity: p_u p_v != p_v p_u
    pub fn do_transported_walks_commute(walk_u: &[Vec<u32>], walk_v: &[Vec<u32>]) -> bool {
        let flat_u = Self::flatten_words(walk_u);
        let flat_v = Self::flatten_words(walk_v);

        let mut uv = flat_u.clone();
        uv.extend_from_slice(&flat_v);

        let mut vu = flat_v.clone();
        vu.extend_from_slice(&flat_u);

        uv == vu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transported_walk_non_commutativity() {
        let walk_u = vec![vec![1, 1, 2]];
        let walk_v = vec![vec![1, 2, 2]];

        let commutes = ClosedWalkAnalyzer::do_transported_walks_commute(&walk_u, &walk_v);
        assert!(!commutes, "Words [1,1,2] and [1,2,2] do not commute!");
    }
}
