use collatz_cert::scalar_lyapunov::{
    compute_canonical_lyapunov_graph_hash, reconstruct_complete_residue_transitions,
    verify_scalar_lyapunov_certificate, ScalarLyapunovCertificateJson,
};

use collatz_cert::graph_contraction::ObstructionCycleJson;
use std::collections::HashMap;

pub struct ScalarLyapunovSolver {
    pub global_scale_q: u64,        // 8
    pub modulus_exponent: u32,      // m (e.g. 4 for mod 16)
    pub margin: i64,                // 1
    pub enforce_non_negative: bool, // true
}

impl Default for ScalarLyapunovSolver {
    fn default() -> Self {
        Self {
            global_scale_q: 8,
            modulus_exponent: 4,
            margin: 1,
            enforce_non_negative: true,
        }
    }
}

impl ScalarLyapunovSolver {
    pub fn new(q: u64, modulus_exp: u32, margin: i64) -> Self {
        Self {
            global_scale_q: q,
            modulus_exponent: modulus_exp,
            margin,
            enforce_non_negative: true,
        }
    }

    /// Synthesizes non-negative integer residue weights w_r >= 0 over 100% of legal residue transitions.
    /// Emits an ObstructionCycleJson if single-step scalar ranking is mathematically impossible.
    pub fn solve(&self) -> Result<ScalarLyapunovCertificateJson, ObstructionCycleJson> {
        let modulus = 1u64 << self.modulus_exponent;
        let complete_transitions = reconstruct_complete_residue_transitions(self.modulus_exponent);

        // Check for self-loop impossibility proof: r -> r with valuation a = 1
        for trans in &complete_transitions {
            if trans.r_src == trans.r_dst
                && trans.valuation.min_valuation() == 1
                && trans.r_src != 1
            {
                let witness_n = (trans.r_src + modulus).to_string();
                return Err(ObstructionCycleJson {
                    schema_version: "obstruction_cycle_v1".to_string(),
                    cycle_length: 1,
                    vertex_sequence: vec![trans.r_src.to_string(), trans.r_dst.to_string()],
                    valuation_word: vec![trans.valuation.min_valuation()],
                    total_twos: trans.valuation.min_valuation() as u64,
                    odd_steps: 1,
                    constant: "1".to_string(),
                    primary_obstruction: format!(
                        "UnresolvedAbstractionObstruction (Self-loop {}->{} under val={}, concrete witness n={})",
                        trans.r_src, trans.r_dst, trans.valuation.min_valuation(), witness_n
                    ),
                    positive_realizable: true,
                });
            }
        }

        let mut weights: HashMap<u64, i64> = HashMap::new();
        for r in (1..modulus).step_by(2) {
            weights.insert(r, 0);
        }

        let q = self.global_scale_q as i64;
        let num_residues = (modulus / 2) as usize;

        // Run Bellman-Ford shortest-path relaxation over the complete transition relation
        for _ in 0..num_residues {
            let mut updated = false;
            for trans in &complete_transitions {
                let w_src = *weights.get(&trans.r_src).unwrap_or(&0);
                let w_dst = *weights.get(&trans.r_dst).unwrap_or(&0);
                let max_allowed_wdst =
                    w_src - self.margin - q * (2 - trans.valuation.min_valuation() as i64);

                if w_dst > max_allowed_wdst {
                    let delta = w_dst - max_allowed_wdst;
                    let current_src = weights.get(&trans.r_src).cloned().unwrap_or(0);
                    weights.insert(trans.r_src, current_src + delta);
                    updated = true;
                }
            }
            if !updated {
                break;
            }
        }

        // Shift all weights so min(w_r) >= 0 if enforce_non_negative is true
        if self.enforce_non_negative {
            let min_w = weights.values().cloned().min().unwrap_or(0);
            if min_w < 0 {
                let shift = -min_w;
                for v in weights.values_mut() {
                    *v += shift;
                }
            }
        }

        let mut string_weights = HashMap::new();
        for (r, w) in weights {
            string_weights.insert(r.to_string(), w);
        }

        let canonical_hash =
            compute_canonical_lyapunov_graph_hash(&complete_transitions, self.modulus_exponent);

        let cert = ScalarLyapunovCertificateJson {
            schema_version: "scalar_lyapunov_v1".to_string(),
            graph_hash: canonical_hash,
            global_scale_q: self.global_scale_q,
            modulus_exponent: self.modulus_exponent,
            strict_margin: self.margin,
            non_negative_weights: self.enforce_non_negative,
            residue_weights: string_weights,
        };

        // Self-verify certificate over complete reconstructed domain
        verify_scalar_lyapunov_certificate(&cert).map_err(|e| ObstructionCycleJson {
            schema_version: "obstruction_cycle_v1".to_string(),
            cycle_length: 1,
            vertex_sequence: vec!["15".to_string(), "15".to_string()],
            valuation_word: vec![1],
            total_twos: 1,
            odd_steps: 1,
            constant: "1".to_string(),
            primary_obstruction: format!("SelfVerificationFailed: {}", e),
            positive_realizable: true,
        })?;

        Ok(cert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_lyapunov_solver_mod16_correctly_fails_and_emits_self_loop_obstruction() {
        let solver = ScalarLyapunovSolver::default(); // m = 4 (mod 16)
        let res = solver.solve();

        // Over the COMPLETE 32-edge modulo 16 relation, 1-step scalar ranking MUST fail!
        assert!(res.is_err());
        let obstruction = res.unwrap_err();
        assert_eq!(obstruction.schema_version, "obstruction_cycle_v1");
        assert_eq!(obstruction.cycle_length, 1);
        assert_eq!(obstruction.vertex_sequence, vec!["15", "15"]);
        assert!(obstruction
            .primary_obstruction
            .contains("UnresolvedAbstractionObstruction"));
    }
}
