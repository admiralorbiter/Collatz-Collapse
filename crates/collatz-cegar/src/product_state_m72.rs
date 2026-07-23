use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct ProductState {
    pub control_state_s: u32,
    pub residue_r: u64,
    pub modulus_exponent_m: u32,
    pub history_provenance: String,
}

impl PartialEq for ProductState {
    fn eq(&self, other: &Self) -> bool {
        self.control_state_s == other.control_state_s
            && self.residue_r == other.residue_r
            && self.modulus_exponent_m == other.modulus_exponent_m
    }
}

impl Eq for ProductState {}

impl Hash for ProductState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.control_state_s.hash(state);
        self.residue_r.hash(state);
        self.modulus_exponent_m.hash(state);
    }
}

impl ProductState {
    pub fn new(s: u32, r: u64, m: u32, provenance: &str) -> Self {
        let mask = (1u64 << m) - 1;
        Self {
            control_state_s: s,
            residue_r: r & mask,
            modulus_exponent_m: m,
            history_provenance: provenance.to_string(),
        }
    }

    pub fn canonical_label(&self) -> String {
        format!(
            "s{}_r{}_m{}",
            self.control_state_s, self.residue_r, self.modulus_exponent_m
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_product_state_hashing_and_canonical_label() {
        let p1 = ProductState::new(0, 7, 5, "seed");
        let p2 = ProductState::new(0, 7, 5, "refinement_depth_1");

        // Canonical equality ignores history provenance string
        assert_eq!(p1, p2);

        let mut set = HashSet::new();
        set.insert(p1);
        assert!(set.contains(&p2));
        assert_eq!(p2.canonical_label(), "s0_r7_m5");
    }
}
