use crate::{
    CoreInteractionKernel, CoreSwitchResult, CoreSwitchType, PeriodicReturnCore,
    TwoAdicValuation,
};
use num_bigint::BigInt;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Fine–Wilf precision bound for distinct primitive periodic cores.
pub struct FineWilfBound;

impl FineWilfBound {
    /// Computes Fine-Wilf maximum incompatible overlap symbols T - 1 = p + q - gcd(p, q) - 1.
    pub fn max_incompatible_overlap_symbols(period_v: usize, period_w: usize) -> usize {
        let g = gcd(period_v, period_w);
        period_v + period_w - g - 1
    }

    /// Computes Fine-Wilf threshold T = p + q - gcd(p, q).
    pub fn fine_wilf_threshold(period_v: usize, period_w: usize) -> usize {
        let g = gcd(period_v, period_w);
        period_v + period_w - g
    }

    /// Computes upper bound on 2-adic core distance valuation \kappa(v,w) = v_2(\Gamma_{v,w})
    /// under maximum single-step valuation B_max: \kappa(v,w) <= B_max * (p + q - gcd(p,q) - 1).
    pub fn max_precision_bound(period_v: usize, period_w: usize, b_max: u64) -> u64 {
        let max_symbols = Self::max_incompatible_overlap_symbols(period_v, period_w) as u64;
        max_symbols * b_max
    }
}

/// Single core switch record in the 2-adic precision ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchLedgerEntry {
    pub step_index: usize,
    pub incoming_depth_s: TwoAdicValuation,
    pub shadowed_cycles_r: usize,
    pub bits_consumed_c: u64,
    pub pre_switch_depth_t: TwoAdicValuation,
    pub core_distance_kappa: TwoAdicValuation,
    pub switch_type: CoreSwitchType,
    pub outgoing_depth_next_s: TwoAdicValuation,
    pub reset_loss: Option<u64>,
    pub resonance_gain_g: Option<u64>,
}

/// Pathwise 2-adic Precision Ledger tracking exact telescoping depth and conditional budget identity.
#[derive(Debug, Clone, Default)]
pub struct PrecisionLedger {
    pub entries: Vec<SwitchLedgerEntry>,
    pub initial_depth_s1: TwoAdicValuation,
    pub total_bits_consumed: u64,
    pub total_reset_losses: u64,
    pub total_resonance_gain: u64,
}

impl PrecisionLedger {
    pub fn new(initial_depth_s1: TwoAdicValuation) -> Self {
        Self {
            entries: Vec::new(),
            initial_depth_s1,
            total_bits_consumed: 0,
            total_reset_losses: 0,
            total_resonance_gain: 0,
        }
    }

    /// Records a core switch step from core v to core w with incoming primitive value A_v(D).
    ///
    /// Four-case non-resonant switch law:
    /// - t < \kappa \implies s_{next} = t (Inherited, delta = 0)
    /// - t > \kappa \implies s_{next} = \kappa (Reset, delta = - (t - \kappa))
    /// - t = \kappa \implies s_{next} = \kappa + g = t + g (Resonant, delta = +g, g >= 1)
    pub fn record_switch(
        &mut self,
        v: &PeriodicReturnCore,
        w: &PeriodicReturnCore,
        a_v: &BigInt,
        shadowed_cycles_r: usize,
    ) -> CoreSwitchResult {
        let kernel = CoreInteractionKernel::new(v, w);
        let switch_res = kernel.evaluate_integer_switch(a_v);

        let incoming_depth_s = switch_res.incoming_depth;
        let bits_consumed_c = (shadowed_cycles_r as u64) * v.data().total_valuation();

        let pre_switch_depth_t = match incoming_depth_s {
            TwoAdicValuation::Infinity => TwoAdicValuation::Infinity,
            TwoAdicValuation::Finite(s) => {
                if s >= bits_consumed_c {
                    TwoAdicValuation::Finite(s - bits_consumed_c)
                } else {
                    TwoAdicValuation::Finite(0)
                }
            }
        };

        let core_distance_kappa = kernel.kappa();
        let outgoing_depth_next_s = switch_res.outgoing_depth;

        let mut reset_loss = None;
        let mut resonance_gain_g = None;

        match (pre_switch_depth_t, core_distance_kappa, outgoing_depth_next_s) {
            (TwoAdicValuation::Finite(t), TwoAdicValuation::Finite(k), TwoAdicValuation::Finite(s_next)) => {
                if t > k {
                    // Case 2: Reset (s_next = k, loss = t - k)
                    reset_loss = Some(t - k);
                } else if t == k {
                    // Case 3: Resonant (s_next = t + g)
                    if s_next > t {
                        resonance_gain_g = Some(s_next - t);
                    } else {
                        resonance_gain_g = Some(0);
                    }
                }
            }
            _ => {}
        }

        let entry = SwitchLedgerEntry {
            step_index: self.entries.len(),
            incoming_depth_s,
            shadowed_cycles_r,
            bits_consumed_c,
            pre_switch_depth_t,
            core_distance_kappa,
            switch_type: switch_res.switch_type,
            outgoing_depth_next_s,
            reset_loss,
            resonance_gain_g,
        };

        self.total_bits_consumed += bits_consumed_c;
        if let Some(loss) = reset_loss {
            self.total_reset_losses += loss;
        }
        if let Some(g) = resonance_gain_g {
            self.total_resonance_gain += g;
        }

        self.entries.push(entry);
        switch_res
    }

    /// Evaluates conditional resonance budget identity:
    /// \sum_{Resonant} g_i >= \sum c_i + \sum_{Reset} (t_i - \kappa_i) - s_1
    pub fn required_resonance_budget(&self) -> u64 {
        let initial_s1 = match self.initial_depth_s1 {
            TwoAdicValuation::Finite(s) => s,
            TwoAdicValuation::Infinity => 0,
        };
        let total_required = self.total_bits_consumed + self.total_reset_losses;
        if total_required >= initial_s1 {
            total_required - initial_s1
        } else {
            0
        }
    }

    /// Verifies if cumulative resonance gains satisfy the conditional resonance budget identity.
    pub fn satisfies_conditional_resonance_budget(&self) -> bool {
        self.total_resonance_gain >= self.required_resonance_budget()
    }
}
