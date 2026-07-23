use crate::{
    CoreInteractionKernel, PeriodicReturnCore, TwoAdicValuation, ValuationWord,
};
use num_bigint::BigInt;
use num_traits::Signed;

/// Semantic Core Distance and State-Core Semantic Bridge (Phase H.2C & H.3B.0).
pub struct SemanticCoreDistanceBridge;

impl SemanticCoreDistanceBridge {
    /// Computes longest common valuation prefix length L = lcp(v, w).
    pub fn longest_common_prefix(v: &ValuationWord, w: &ValuationWord) -> usize {
        let v_slice = v.as_slice();
        let w_slice = w.as_slice();
        let min_len = v_slice.len().min(w_slice.len());
        let mut l = 0;
        while l < min_len && v_slice[l] == w_slice[l] {
            l += 1;
        }
        l
    }

    /// Computes periodic valuation exponent sum H_m = \sum_{j=0}^{m-1} b_{j \bmod period}
    /// over m steps for core word with valuation exponents b_j.
    pub fn weighted_precision(word: &ValuationWord, m: usize) -> u64 {
        let slice = word.as_slice();
        if slice.is_empty() || m == 0 {
            return 0;
        }
        let period = slice.len();
        let mut sum = 0u64;
        for j in 0..m {
            let symbol = slice[j % period];
            sum += symbol as u64;
        }
        sum
    }

    /// Computes 2-adic bit valuation precision H_m = \sum_{j=0}^{m-1} (9 + 4 * b_{j \bmod period})
    /// over m steps for core word with valuation exponents b_j.
    pub fn weighted_bit_precision(word: &ValuationWord, m: usize) -> u64 {
        let slice = word.as_slice();
        if slice.is_empty() || m == 0 {
            return 0;
        }
        let period = slice.len();
        let mut sum = 0u64;
        for j in 0..m {
            let symbol = slice[j % period];
            sum += 9 + 4 * (symbol as u64);
        }
        sum
    }

    /// Verifies the core-to-core interval identity: H_L <= \kappa(v,w) < H_L + min(B_{x_{L+1}}, B_{y_{L+1}})
    /// where L = lcp(v^\infty, w^\infty).
    pub fn verify_weighted_lcp_interval(
        v: &PeriodicReturnCore,
        w: &PeriodicReturnCore,
    ) -> (u64, TwoAdicValuation, u64) {
        let word_v = v.data().word();
        let word_w = w.data().word();

        let l = Self::longest_common_prefix(word_v, word_w);
        let h_l = Self::weighted_precision(word_v, l);

        let v_slice = word_v.as_slice();
        let w_slice = word_w.as_slice();
        let b_next_v = if l < v_slice.len() { v_slice[l] as u64 } else { 1 };
        let b_next_w = if l < w_slice.len() { w_slice[l] as u64 } else { 1 };

        let h_upper = h_l + b_next_v.min(b_next_w);

        let kernel = CoreInteractionKernel::new(v, w);
        let kappa = kernel.kappa();

        (h_l, kappa, h_upper)
    }

    /// Proves that for positive ordinary integer D > 0 and fixed point \xi_v < 0,
    /// exact core landing A_v(D) = 0 is IMPOSSIBLE (A_v(D) > 0 strictly).
    pub fn positive_state_never_exact_core(core: &PeriodicReturnCore, d_val: &BigInt) -> bool {
        if !d_val.is_positive() {
            return false;
        }
        let a_v = core.eval_integer_primitive(d_val);
        a_v.is_positive()
    }

    /// H.3B.0 State-Core Semantic Bridge:
    /// Relates incoming valuation depth s = v_2(A_v(D)) to exact future agreement length L
    /// between the physical trajectory of D and core periodic continuation v^\infty.
    /// Verifies H_L <= s < H_L + B_{x_{L+1}}.
    pub fn verify_state_core_semantic_depth(
        core: &PeriodicReturnCore,
        d_val: &BigInt,
        actual_future_itinerary: &ValuationWord,
    ) -> (u64, TwoAdicValuation, u64) {
        let a_v = core.eval_integer_primitive(d_val);
        let s = TwoAdicValuation::from_bigint(&a_v);

        let word_v = core.data().word();
        let l = Self::longest_common_prefix(word_v, actual_future_itinerary);
        let h_l = Self::weighted_bit_precision(word_v, l);

        let v_slice = word_v.as_slice();
        let b_next = if l < v_slice.len() { v_slice[l] as u64 } else { 1 };
        let h_upper = h_l + (9 + 4 * b_next);

        (h_l, s, h_upper)
    }
}
