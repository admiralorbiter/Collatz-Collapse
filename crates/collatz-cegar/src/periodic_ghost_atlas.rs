use crate::extremal_source_search::ExtremalSourceSearchEngine;
use crate::zero_tail_stress_audit::ZeroTailProfile;
use num_bigint::BigInt;

/// Phase 7.3S.1C: Periodic & Eventually Periodic Ghost Source Density Engine.
#[derive(Debug, Clone)]
pub struct PeriodicGhostAtlas {
    pub period_sequence: Vec<u64>,
}

impl PeriodicGhostAtlas {
    pub fn new(period_sequence: Vec<u64>) -> Self {
        Self { period_sequence }
    }

    /// Compute pure periodic ghost fixed point z_w^* = -p_w / q_w in 2-adic rationals.
    pub fn pure_periodic_ghost(&self) -> (BigInt, BigInt) {
        let word = ExtremalSourceSearchEngine::sequence_guarded_word(&self.period_sequence);
        let m_big = BigInt::from(word.affine.denominator.clone());
        let q_big = BigInt::from(word.affine.multiplier.clone());
        let beta_big = word.affine.intercept.clone();

        // F_w(x) = (Q_w x + beta_w) / M_w = x => (M_w - Q_w) x = beta_w
        // x = - beta_w / (Q_w - M_w) = - p_w / q_w where q_w = Q_w - M_w
        let p_w = beta_big;
        let q_w = &q_big - &m_big;
        (p_w, q_w)
    }

    /// Compute eventually periodic ghost source residue x = ( - Q_u q_w p_w + beta_u q_w ) / ( M_u q_w ).
    pub fn eventually_periodic_ghost(&self, prefix: &[u64]) -> ZeroTailProfile {
        let mut word = ExtremalSourceSearchEngine::sequence_guarded_word(prefix);
        let rep_count = 10;
        for _ in 0..rep_count {
            for &g in &self.period_sequence {
                word = ExtremalSourceSearchEngine::extend_guarded_word(&word, g);
            }
        }
        ZeroTailProfile::from_canonical_word(&word)
    }
}
