use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuardedEdge {
    pub source_state: String,
    pub source_subguard: String,
    pub valuation_word: Vec<u32>,
    pub target_state: String,
    pub target_image_base: u64,
    pub target_residue: u64,
    pub target_modulus_exponent: u32,
    pub universal_inclusion: bool,
}

pub struct GraphClosureEngine;

impl GraphClosureEngine {
    /// Evaluates w1 = [1,1,2] for n = r + 1024k
    pub fn eval_w1(n: u128) -> u128 {
        (27 * n + 19) / 16
    }

    /// Evaluates w2 = [1,2,2] for n = r + 2048k
    pub fn eval_w2(n: u128) -> u128 {
        (27 * n + 23) / 32
    }

    /// Evaluates w3 = [1,1,1,2] for n = r + 2048k
    pub fn eval_w3(n: u128) -> u128 {
        (81 * n + 65) / 32
    }

    /// Verifies path realizability & exact path cylinder for uv switching (214759 mod 262144)
    pub fn verify_uv_realizability() -> bool {
        let n0 = 214759u128;
        assert_eq!(n0 % 262144, 214759, "n0 must be in 214759 mod 262144");
        assert_eq!(n0 % 1024, 743, "n0 must be in 743 mod 1024 (Q1 self-loop)");

        let n1 = Self::eval_w1(n0); // 362407
        assert_eq!(n1 % 1024, 935, "n1 must be in 935 mod 1024 (Q1 -> Q2)");

        let n2 = Self::eval_w1(n1); // 611563
        assert_eq!(n2 % 1024, 235, "n2 must be in 235 mod 1024 (Q2 -> Q1)");

        let n3 = Self::eval_w2(n2); // 516007
        assert_eq!(n3 % 32, 7, "n3 must return to Q1 (7 mod 32)");

        // Composed map verification: F_uv(n) = (19683 * n + 27947) / 8192
        let num_uv = 19683 * n0 + 27947;
        assert_eq!(num_uv % 8192, 0, "Numerator must be divisible by 8192");
        assert_eq!(num_uv / 8192, 516007, "Composed map must output 516007");

        true
    }

    /// Verifies path realizability & exact path cylinder for vu switching (1959 mod 262144)
    pub fn verify_vu_realizability() -> bool {
        let n0 = 1959u128;
        assert_eq!(n0 % 262144, 1959, "n0 must be in 1959 mod 262144");
        assert_eq!(n0 % 1024, 935, "n0 must be in 935 mod 1024 (Q1 -> Q2)");

        let n1 = Self::eval_w1(n0); // 3307
        assert_eq!(n1 % 1024, 235, "n1 must be in 235 mod 1024 (Q2 -> Q1)");

        let n2 = Self::eval_w2(n1); // 2791
        assert_eq!(n2 % 1024, 743, "n2 must be in 743 mod 1024 (Q1 self-loop)");

        let n3 = Self::eval_w1(n2); // 4711
        assert_eq!(n3 % 32, 7, "n3 must return to Q1 (7 mod 32)");

        // Composed map verification: F_vu(n) = (19683 * n + 33515) / 8192
        let num_vu = 19683 * n0 + 33515;
        assert_eq!(num_vu % 8192, 0, "Numerator must be divisible by 8192");
        assert_eq!(num_vu / 8192, 4711, "Composed map must output 4711");

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uv_and_vu_switching_realizability() {
        assert!(GraphClosureEngine::verify_uv_realizability());
        assert!(GraphClosureEngine::verify_vu_realizability());
    }
}
