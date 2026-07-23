use num_bigint::BigUint;
use num_rational::BigRational;
use num_traits::{One, Zero};

/// LSB-first Binary Trie node tracking 2-adic residue class coverage over odd integers.
#[derive(Debug, Clone, Default)]
struct TrieNode {
    is_covered: bool,
    left: Option<Box<TrieNode>>,  // bit 0
    right: Option<Box<TrieNode>>, // bit 1
}

impl TrieNode {
    /// Inserts a residue class r mod 2^M for odd integers.
    /// Bit 0 of r is 1; bits 1..(M-1) form the (M-1)-bit LSB key.
    fn insert(&mut self, residue: &BigUint, modulus_exponent: u64, depth: u64) -> bool {
        if self.is_covered {
            return false; // Already covered by a broader cylinder
        }

        if depth + 1 >= modulus_exponent {
            self.is_covered = true;
            self.left = None;
            self.right = None;
            return true;
        }

        // Bit at position (depth + 1)
        let bit = (residue >> (depth + 1)) & BigUint::one();

        if bit.is_zero() {
            let left_child = self.left.get_or_insert_with(Default::default);
            left_child.insert(residue, modulus_exponent, depth + 1);
        } else {
            let right_child = self.right.get_or_insert_with(Default::default);
            right_child.insert(residue, modulus_exponent, depth + 1);
        }

        // Canonical collapse if both children are fully covered
        let left_full = self.left.as_ref().map_or(false, |c| c.is_covered);
        let right_full = self.right.as_ref().map_or(false, |c| c.is_covered);

        if left_full && right_full {
            self.is_covered = true;
            self.left = None;
            self.right = None;
        }

        true
    }

    /// Computes canonical union measure as BigRational.
    fn compute_measure(&self, depth: u64) -> BigRational {
        if self.is_covered {
            let denom = BigUint::one() << depth;
            return BigRational::new(BigUint::one().into(), denom.into());
        }

        let mut sum = BigRational::zero();
        if let Some(ref l) = self.left {
            sum += l.compute_measure(depth + 1);
        }
        if let Some(ref r) = self.right {
            sum += r.compute_measure(depth + 1);
        }
        sum
    }
}

/// 2-Adic Measure Trie for tracking residue coverage and exact canonical union measure over odd integers.
#[derive(Debug, Clone, Default)]
pub struct MeasureTrie {
    root: TrieNode,
    pub count: usize,
}

impl MeasureTrie {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts residue class r mod 2^modulus_exponent (over odd integers).
    pub fn insert(&mut self, residue: &BigUint, modulus_exponent: u64) -> bool {
        if modulus_exponent == 0 {
            return false;
        }
        self.count += 1;
        self.root.insert(residue, modulus_exponent, 0)
    }

    /// Returns the exact canonical union measure as a BigRational in [0, 1].
    pub fn canonical_union_measure(&self) -> BigRational {
        let m = self.root.compute_measure(0);
        debug_assert!(
            m <= BigRational::one(),
            "Canonical union measure cannot exceed 1.0"
        );
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disjoint_residues() {
        let mut trie = MeasureTrie::new();
        // 3 mod 8 (011_2): bits 1..2 are 0,1.
        // 7 mod 8 (111_2): bits 1..2 are 1,1.
        trie.insert(&BigUint::from(3u32), 3);
        trie.insert(&BigUint::from(7u32), 3);

        // Each covers 1/4 of odd integers. Total = 1/2.
        assert_eq!(
            trie.canonical_union_measure(),
            BigRational::new(1u32.into(), 2u32.into())
        );
    }

    #[test]
    fn test_overlapping_residues_absorbed() {
        let mut trie = MeasureTrie::new();
        // 3 mod 4 (11_2): covers bits 1..1 = 1. Measure = 1/2.
        // 7 mod 8 (111_2): subset of 3 mod 4.
        trie.insert(&BigUint::from(3u32), 2);
        trie.insert(&BigUint::from(7u32), 3);

        // Union measure should be exactly 1/2.
        assert_eq!(
            trie.canonical_union_measure(),
            BigRational::new(1u32.into(), 2u32.into())
        );
    }

    #[test]
    fn test_sibling_merge() {
        let mut trie = MeasureTrie::new();
        // 3 mod 8 (011_2) and 7 mod 8 (111_2) merge to 3 mod 4 (11_2)
        trie.insert(&BigUint::from(3u32), 3);
        trie.insert(&BigUint::from(7u32), 3);
        assert_eq!(
            trie.canonical_union_measure(),
            BigRational::new(1u32.into(), 2u32.into())
        );
    }
}
