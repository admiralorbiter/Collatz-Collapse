use crate::{PeriodicReturnCore, ValuationWord};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Primitive Core Selection containing full orbit, phase, and arithmetic data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimitiveCoreSelection {
    pub primitive_word: ValuationWord,
    pub orbit_id: u64,
    pub phase_offset: usize,
    pub period: usize,
    pub repetition_count: usize,
    pub core: PeriodicReturnCore,
}

/// Selector Output according to Axiom 6 (Null Output).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SelectorOutput {
    StructuredCore(PrimitiveCoreSelection),
    NoStructuredCore,
}

/// Detailed Extension Stability Report (Axiom 7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreTransitionReport {
    PersistedCore { orbit_id: u64, phase_offset: usize },
    AdvancedPhase { orbit_id: u64, new_phase: usize },
    ExtendedWindow { orbit_id: u64, repetitions: usize },
    SwitchedCore { from_orbit_id: u64, to_orbit_id: u64 },
    LostStructure { previous_orbit_id: u64 },
    InitialSelection { orbit_id: u64 },
}

/// Symbolic-first, prefix-measurable Canonical Core Selector implementing the 7 Selector Axioms.
#[derive(Debug, Clone)]
pub struct CanonicalCoreSelector {
    min_repetitions: usize,
    max_period: usize,
}

impl Default for CanonicalCoreSelector {
    fn default() -> Self {
        Self {
            min_repetitions: 2,
            max_period: 32,
        }
    }
}

impl CanonicalCoreSelector {
    pub fn new(min_repetitions: usize, max_period: usize) -> Self {
        Self {
            min_repetitions,
            max_period,
        }
    }

    /// Reduces a valuation word to its unique primitive root word.
    pub fn primitive_root(word: &ValuationWord) -> ValuationWord {
        let slice = word.as_slice();
        let len = slice.len();
        if len <= 1 {
            return word.clone();
        }

        for p in 1..=len {
            if len % p == 0 {
                let pattern = &slice[0..p];
                let mut is_period = true;
                for chunk in slice.chunks(p) {
                    if chunk != pattern {
                        is_period = false;
                        break;
                    }
                }
                if is_period {
                    return ValuationWord::from_slice(pattern);
                }
            }
        }
        word.clone()
    }

    /// Returns the canonical primitive root (lexicographically minimal cyclic shift of primitive root).
    pub fn canonical_primitive_root(word: &ValuationWord) -> ValuationWord {
        let prim = Self::primitive_root(word);
        let slice = prim.as_slice();
        let n = slice.len();
        if n <= 1 {
            return prim;
        }

        let mut min_shift = slice.to_vec();
        for i in 1..n {
            let mut rotated = slice[i..].to_vec();
            rotated.extend_from_slice(&slice[0..i]);
            if rotated < min_shift {
                min_shift = rotated;
            }
        }
        ValuationWord::from_slice(&min_shift)
    }

    /// Computes canonical orbit ID from primitive root word.
    pub fn canonical_orbit_id(primitive: &ValuationWord) -> u64 {
        let mut hasher = DefaultHasher::new();
        primitive.as_slice().hash(&mut hasher);
        hasher.finish()
    }

    /// Selects the canonical core from an observed valuation history window (Axioms 1-6).
    pub fn select_core(&self, history: &ValuationWord) -> SelectorOutput {
        let slice = history.as_slice();
        let len = slice.len();

        if len < self.min_repetitions {
            return SelectorOutput::NoStructuredCore;
        }

        let mut best_selection: Option<PrimitiveCoreSelection> = None;

        // Search for repeating periods p <= max_period
        for p in 1..=self.max_period.min(len / self.min_repetitions) {
            let suffix_len = (len / p) * p;
            let start = len - suffix_len;
            let window = &slice[start..];

            let pattern = &window[0..p];
            let mut reps = 0;
            let mut valid = true;

            for chunk in window.chunks(p) {
                if chunk.len() == p {
                    if chunk == pattern {
                        reps += 1;
                    } else {
                        valid = false;
                        break;
                    }
                }
            }

            if valid && reps >= self.min_repetitions {
                let candidate_word = ValuationWord::from_slice(pattern);
                let canonical_prim = Self::canonical_primitive_root(&candidate_word);
                let orbit_id = Self::canonical_orbit_id(&canonical_prim);
                let period = canonical_prim.len();

                // Compute phase offset relative to canonical primitive root
                let prim_slice = canonical_prim.as_slice();
                let mut phase_offset = 0;
                for o in 0..period {
                    let mut rotated = prim_slice[o..].to_vec();
                    rotated.extend_from_slice(&prim_slice[0..o]);
                    if rotated.as_slice() == pattern {
                        phase_offset = o;
                        break;
                    }
                }

                if let Ok(core) = PeriodicReturnCore::new(canonical_prim.clone()) {
                    let selection = PrimitiveCoreSelection {
                        primitive_word: canonical_prim,
                        orbit_id,
                        phase_offset,
                        period,
                        repetition_count: reps,
                        core,
                    };

                    match &best_selection {
                        None => best_selection = Some(selection),
                        Some(current) => {
                            // Axiom 5: Deterministic tie-breaking (shorter period first, then higher repetition)
                            if selection.period < current.period
                                || (selection.period == current.period
                                    && selection.repetition_count > current.repetition_count)
                            {
                                best_selection = Some(selection);
                            }
                        }
                    }
                }
            }
        }

        match best_selection {
            Some(sel) => SelectorOutput::StructuredCore(sel),
            None => SelectorOutput::NoStructuredCore,
        }
    }

    /// Evaluates transition stability between previous output and current output (Axiom 7).
    pub fn report_transition(
        prev: &SelectorOutput,
        curr: &SelectorOutput,
    ) -> CoreTransitionReport {
        match (prev, curr) {
            (SelectorOutput::NoStructuredCore, SelectorOutput::StructuredCore(c)) => {
                CoreTransitionReport::InitialSelection { orbit_id: c.orbit_id }
            }
            (SelectorOutput::StructuredCore(p), SelectorOutput::NoStructuredCore) => {
                CoreTransitionReport::LostStructure { previous_orbit_id: p.orbit_id }
            }
            (SelectorOutput::StructuredCore(p), SelectorOutput::StructuredCore(c)) => {
                if p.orbit_id != c.orbit_id {
                    CoreTransitionReport::SwitchedCore {
                        from_orbit_id: p.orbit_id,
                        to_orbit_id: c.orbit_id,
                    }
                } else if c.repetition_count > p.repetition_count {
                    CoreTransitionReport::ExtendedWindow {
                        orbit_id: c.orbit_id,
                        repetitions: c.repetition_count,
                    }
                } else if c.phase_offset != p.phase_offset {
                    CoreTransitionReport::AdvancedPhase {
                        orbit_id: c.orbit_id,
                        new_phase: c.phase_offset,
                    }
                } else {
                    CoreTransitionReport::PersistedCore {
                        orbit_id: c.orbit_id,
                        phase_offset: c.phase_offset,
                    }
                }
            }
            (SelectorOutput::NoStructuredCore, SelectorOutput::NoStructuredCore) => {
                CoreTransitionReport::LostStructure { previous_orbit_id: 0 }
            }
        }
    }
}
