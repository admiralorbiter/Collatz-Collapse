use crate::{
    CanonicalCoreSelector, CoreTransitionReport, FineWilfBound, PeriodicReturnCore, PrecisionLedger,
    SelectorOutput, SemanticCoreDistanceBridge, TwoAdicValuation, ValuationWord,
};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Parameterized Sturmian Gap Embedding mapping binary alphabet {0, 1} to gap valuations {a, b}.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SturmianGapEmbedding {
    pub gap_a: u8,
    pub gap_b: u8,
}

impl SturmianGapEmbedding {
    pub fn new(gap_a: u8, gap_b: u8) -> Self {
        Self { gap_a, gap_b }
    }

    /// Ordered embedding (1, 2)
    pub fn ordered_1_2() -> Self {
        Self { gap_a: 1, gap_b: 2 }
    }

    /// Ordered embedding (2, 1)
    pub fn ordered_2_1() -> Self {
        Self { gap_a: 2, gap_b: 1 }
    }

    /// Single-step bit cost B_a = 9 + 4a, B_b = 9 + 4b.
    pub fn step_cost(&self, symbol: u8) -> u64 {
        if symbol == 0 {
            9 + 4 * (self.gap_a as u64)
        } else {
            9 + 4 * (self.gap_b as u64)
        }
    }
}

/// Represents a single phase-node in the universal Sturmian candidate core graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SturmianPhaseNode {
    pub node_id: usize,
    pub orbit_id: u64,
    pub primitive_word: ValuationWord,
    pub phase_offset: usize,
    pub phase_word: ValuationWord,
    pub core: PeriodicReturnCore,
    pub d_v: i64,
}

/// Balanced Local-Template Universe & Sound Transition Extractor (Phase H.3B).
pub struct SturmianTemplateExtractor;

impl SturmianTemplateExtractor {
    /// Verifies if a binary slice is balanced: for all subwords of equal length, count of b differs by <= 1.
    pub fn is_balanced(slice: &[u8], gap_b: u8) -> bool {
        let len = slice.len();
        for k in 1..=len {
            let mut min_b = usize::MAX;
            let mut max_b = 0;
            for window in slice.windows(k) {
                let count_b = window.iter().filter(|&&x| x == gap_b).count();
                if count_b < min_b {
                    min_b = count_b;
                }
                if count_b > max_b {
                    max_b = count_b;
                }
            }
            if max_b - min_b > 1 {
                return false;
            }
        }
        true
    }

    /// Generates length-32 balanced binary Sturmian templates.
    pub fn generate_length_32_balanced_templates(gap_a: u8, gap_b: u8) -> Vec<ValuationWord> {
        let length = 32;
        let mut templates = Vec::new();

        for num_b in 0..=length {
            let mut word = Vec::with_capacity(length);
            for i in 0..length {
                let val = if ((i + 1) * num_b) / length > (i * num_b) / length {
                    gap_b
                } else {
                    gap_a
                };
                word.push(val);
            }
            if Self::is_balanced(&word, gap_b) {
                templates.push(ValuationWord::from_slice(&word));
            }
        }
        templates
    }
}

/// Universal 52-Phase Sturmian Transition Graph Engine (Phase H.3B).
pub struct SturmianTransitionGraph {
    pub embedding: SturmianGapEmbedding,
    pub nodes: Vec<SturmianPhaseNode>,
    pub edges: Vec<(usize, usize, i64)>, // (source_node, target_node, worst_case_net_weight)
}

impl SturmianTransitionGraph {
    /// Helper to compute orbit_id hash for a primitive word.
    pub fn compute_orbit_id(primitive_word: &ValuationWord) -> u64 {
        let mut hasher = DefaultHasher::new();
        primitive_word.hash(&mut hasher);
        hasher.finish()
    }

    /// Generates all 14 primitive binary necklaces of lengths 1 through 5.
    pub fn generate_14_primitive_necklaces(embedding: &SturmianGapEmbedding) -> Vec<ValuationWord> {
        let a = embedding.gap_a;
        let b = embedding.gap_b;

        let raw_patterns: Vec<Vec<u8>> = vec![
            // Length 1 (2)
            vec![a],
            vec![b],
            // Length 2 (1)
            vec![a, b],
            // Length 3 (2)
            vec![a, a, b],
            vec![a, b, b],
            // Length 4 (3)
            vec![a, a, a, b],
            vec![a, a, b, b],
            vec![a, b, b, b],
            // Length 5 (6)
            vec![a, a, a, a, b],
            vec![a, a, a, b, b],
            vec![a, a, b, a, b],
            vec![a, a, b, b, b],
            vec![a, b, a, b, b],
            vec![a, b, b, b, b],
        ];

        raw_patterns
            .into_iter()
            .map(|pat| ValuationWord::from_slice(&pat))
            .collect()
    }

    /// Proves period-extension invariance for right-censored target agreement:
    /// g(L + k * |w|) - (r_{target} + k) * B_w == g(L) - r_{target} * B_w.
    pub fn verify_period_extension_invariance(
        word_w: &ValuationWord,
        l_observed: usize,
        k_periods: usize,
    ) -> bool {
        let period_w = word_w.as_slice().len();
        let b_w = SemanticCoreDistanceBridge::weighted_bit_precision(word_w, period_w);

        let r_observed = l_observed / period_w;
        let g_observed = SemanticCoreDistanceBridge::weighted_bit_precision(word_w, l_observed);
        let net_observed = (g_observed as i64) - ((r_observed as u64) * b_w) as i64;

        let l_extended = l_observed + k_periods * period_w;
        let r_extended = l_extended / period_w;
        let g_extended = SemanticCoreDistanceBridge::weighted_bit_precision(word_w, l_extended);
        let net_extended = (g_extended as i64) - ((r_extended as u64) * b_w) as i64;

        net_observed == net_extended
    }

    /// Builds the universal 52-phase Sturmian transition graph with worst-case edge aggregation.
    pub fn build(embedding: SturmianGapEmbedding) -> Self {
        let primitive_necklaces = Self::generate_14_primitive_necklaces(&embedding);
        let mut nodes = Vec::new();
        let mut node_counter = 0;

        for primitive_word in &primitive_necklaces {
            let orbit_id = Self::compute_orbit_id(primitive_word);
            let period = primitive_word.as_slice().len();

            let slice = primitive_word.as_slice();
            for k in 0..period {
                let mut rotated = Vec::with_capacity(period);
                rotated.extend_from_slice(&slice[k..]);
                rotated.extend_from_slice(&slice[0..k]);
                let phase_word = ValuationWord::from_slice(&rotated);

                if let Ok(phase_core) = PeriodicReturnCore::new(phase_word.clone()) {
                    let q_v = phase_core.data().multiplier();
                    let m_v = phase_core.data().denominator();
                    let d_v = (q_v - m_v).to_i64().unwrap_or(1);

                    let node = SturmianPhaseNode {
                        node_id: node_counter,
                        orbit_id,
                        primitive_word: primitive_word.clone(),
                        phase_offset: k,
                        phase_word,
                        core: phase_core,
                        d_v,
                    };
                    nodes.push(node);
                    node_counter += 1;
                }
            }
        }

        let templates = SturmianTemplateExtractor::generate_length_32_balanced_templates(
            embedding.gap_a,
            embedding.gap_b,
        );

        let mut max_edge_weights: HashMap<(usize, usize), i64> = HashMap::new();

        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if i != j {
                    let source = &nodes[i];
                    let target = &nodes[j];

                    if source.orbit_id != target.orbit_id {
                        let out_s = SelectorOutput::StructuredCore(crate::PrimitiveCoreSelection {
                            primitive_word: source.primitive_word.clone(),
                            orbit_id: source.orbit_id,
                            phase_offset: source.phase_offset,
                            period: source.primitive_word.as_slice().len(),
                            repetition_count: 2,
                            core: source.core.clone(),
                        });
                        let out_t = SelectorOutput::StructuredCore(crate::PrimitiveCoreSelection {
                            primitive_word: target.primitive_word.clone(),
                            orbit_id: target.orbit_id,
                            phase_offset: target.phase_offset,
                            period: target.primitive_word.as_slice().len(),
                            repetition_count: 2,
                            core: target.core.clone(),
                        });

                        let report = CanonicalCoreSelector::report_transition(&out_s, &out_t);
                        if matches!(report, CoreTransitionReport::SwitchedCore { .. }) {
                            let source_period = source.primitive_word.as_slice().len();
                            let target_period = target.primitive_word.as_slice().len();

                            let source_b_v = SemanticCoreDistanceBridge::weighted_bit_precision(
                                source.core.data().word(),
                                source_period,
                            );
                            let target_b_w = SemanticCoreDistanceBridge::weighted_bit_precision(
                                target.core.data().word(),
                                target_period,
                            );

                            for template in &templates {
                                let raw_l = SemanticCoreDistanceBridge::longest_common_prefix(
                                    target.core.data().word(),
                                    template,
                                );

                                let max_incompat_symbols =
                                    FineWilfBound::max_incompatible_overlap_symbols(source_period, target_period);

                                let l = raw_l.min(max_incompat_symbols);

                                let r_source = 2u64;
                                let r_target = 2.max((l / target_period) as u64);

                                let source_consumed = r_source * source_b_v;
                                let target_cost = r_target * target_b_w;

                                let mut ledger = PrecisionLedger::new(TwoAdicValuation::Finite(16));
                                let a_v = BigInt::from(64u32);
                                let _switch_res = ledger.record_switch(&source.core, &target.core, &a_v, 2);

                                let reset_losses = ledger.total_reset_losses;

                                let bounded_gain =
                                    SemanticCoreDistanceBridge::weighted_bit_precision(target.core.data().word(), l);

                                let net_weight =
                                    (bounded_gain as i64) - ((source_consumed + target_cost + reset_losses) as i64);

                                max_edge_weights
                                    .entry((i, j))
                                    .and_modify(|w| *w = (*w).max(net_weight))
                                    .or_insert(net_weight);
                            }
                        }
                    }
                }
            }
        }

        let edges = max_edge_weights
            .into_iter()
            .map(|((s, t), w)| (s, t, w))
            .collect();

        Self {
            embedding,
            nodes,
            edges,
        }
    }

    /// Certifies that every reachable directed cycle has a strictly negative net weight
    /// using potential function search \Phi(v) = -dist[v] on negated edge weights -W(e).
    /// Computes exact minimum integer slack \varepsilon^* = \min_{e} [-W(e) - \Phi(t) + \Phi(s)].
    pub fn certify_negative_cycle_potential(&self, max_epsilon: i64) -> Option<(HashMap<usize, i64>, i64)> {
        let num_nodes = self.nodes.len();
        if num_nodes == 0 {
            return None;
        }

        let mut aug_edges = Vec::with_capacity(self.edges.len() + num_nodes);
        for &(s, t, w) in &self.edges {
            aug_edges.push((s, t, -w)); // cost c(e) = -W(e)
        }
        for i in 0..num_nodes {
            aug_edges.push((num_nodes, i, 0));
        }

        let mut dist_aug = vec![i64::MAX / 4; num_nodes + 1];
        dist_aug[num_nodes] = 0;

        for _ in 0..=num_nodes {
            let mut updated = false;
            for &(s, t, cost) in &aug_edges {
                if dist_aug[s] != i64::MAX / 4 && dist_aug[s] + cost < dist_aug[t] {
                    dist_aug[t] = dist_aug[s] + cost;
                    updated = true;
                }
            }
            if !updated {
                break;
            }
        }

        let mut potential = HashMap::new();
        for i in 0..num_nodes {
            potential.insert(i, -dist_aug[i]);
        }

        let mut min_slack = i64::MAX;
        for &(s, t, w) in &self.edges {
            let phi_s = potential.get(&s)?;
            let phi_t = potential.get(&t)?;
            let slack = -w - phi_t + phi_s;
            if slack < min_slack {
                min_slack = slack;
            }
            if w + phi_t - phi_s > -max_epsilon {
                return None;
            }
        }

        Some((potential, min_slack))
    }
}
