use crate::{
    AffineError, MacrostepData, Q1Quotient, QuotientRegisterMachine, ReturnTransitionOutcome,
};
use num_bigint::BigUint;
use num_traits::Zero;

/// Deterministic 9-region valuation partition for x = v_2(L_u(n)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValuationRegion {
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    XGe13,
    Infinity,
}

impl ValuationRegion {
    pub fn from_x(x_val: u64) -> Self {
        match x_val {
            5 => ValuationRegion::X5,
            6 => ValuationRegion::X6,
            7 => ValuationRegion::X7,
            8 => ValuationRegion::X8,
            9 => ValuationRegion::X9,
            10 => ValuationRegion::X10,
            11 => ValuationRegion::X11,
            12 => ValuationRegion::X12,
            _ => ValuationRegion::XGe13,
        }
    }
}

/// Concrete 2-adic ultrametric state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConcreteUltrametricState {
    Finite { x: u64, unit: BigUint },
    Infinity,
}

impl ConcreteUltrametricState {
    pub fn from_q1_quotient(quotient: &Q1Quotient) -> Self {
        let k = quotient.value();
        // L_u(n) = 32 * (11k + 3)
        let expr = (BigUint::from(11u32) * k) + 3u32;
        let trailing = expr.trailing_zeros().unwrap_or(0);
        let x = 5 + trailing;
        let unit = &expr >> trailing;

        Self::Finite { x, unit }
    }

    pub fn from_n(n: &BigUint) -> Result<Self, AffineError> {
        let q = Q1Quotient::from_integer(n)?;
        Ok(Self::from_q1_quotient(&q))
    }
}

/// Maps concrete ultrametric state back to nonnegative integer quotient k if realizable over N.
pub fn positive_integer_realization(state: &ConcreteUltrametricState) -> Option<BigUint> {
    match state {
        ConcreteUltrametricState::Infinity => None, // k = -3/11 \in Z_2 \setminus N
        ConcreteUltrametricState::Finite { x, unit } => {
            if *x < 5 {
                return None;
            }
            let shift = x - 5;
            let term = unit << shift;
            if term < BigUint::from(3u32) {
                return None;
            }
            let num = term - 3u32;
            if (&num % 11u32).is_zero() {
                Some(num / 11u32)
            } else {
                None
            }
        }
    }
}

/// Unit residue constraint U mod 2^exponent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitResidue {
    pub residue: BigUint,
    pub exponent: u32,
}

/// Abstract 2-adic ultrametric state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractUltrametricState {
    pub valuation_region: ValuationRegion,
    pub unit_constraint: Option<UnitResidue>,
}

/// Three-valued abstract transition enabledness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbstractEnabledness {
    Impossible,
    Guaranteed,
    Mixed,
}

/// Exact transition outcome for concrete ultrametric step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UltrametricStepOutcome {
    NonIntegral,
    IntegralEvenOutsideQ1 {
        image_x: u64,
    },
    ExactButLeavesQ1 {
        image_x: u64,
        image_unit: BigUint,
    },
    BasedReturn {
        next_state: ConcreteUltrametricState,
    },
}

/// Ultrametric state machine evaluator.
pub struct UltrametricMachine;

impl UltrametricMachine {
    /// Exact u transition on concrete state: x' = x - 4, U' = 27U.
    pub fn step_u(state: &ConcreteUltrametricState) -> UltrametricStepOutcome {
        match state {
            ConcreteUltrametricState::Infinity => UltrametricStepOutcome::BasedReturn {
                next_state: ConcreteUltrametricState::Infinity,
            },
            ConcreteUltrametricState::Finite { x, unit } => {
                if *x < 4 {
                    return UltrametricStepOutcome::NonIntegral;
                }
                let image_x = x - 4;
                let image_unit = unit * 27u32;

                if image_x < 5 {
                    UltrametricStepOutcome::ExactButLeavesQ1 {
                        image_x,
                        image_unit,
                    }
                } else {
                    UltrametricStepOutcome::BasedReturn {
                        next_state: ConcreteUltrametricState::Finite {
                            x: image_x,
                            unit: image_unit,
                        },
                    }
                }
            }
        }
    }

    /// Exact resonant v transition on layer x = 6.
    pub fn step_v_resonant(state: &ConcreteUltrametricState) -> UltrametricStepOutcome {
        match state {
            ConcreteUltrametricState::Infinity => UltrametricStepOutcome::NonIntegral,
            ConcreteUltrametricState::Finite { x, unit } => {
                if *x != 6 {
                    return UltrametricStepOutcome::NonIntegral;
                }

                // 729 * U + 87
                let expr = (BigUint::from(729u32) * unit) + 87u32;
                let gamma = expr.trailing_zeros().unwrap_or(0);

                if gamma < 3 {
                    // U mod 8 != 1: non-integral
                    UltrametricStepOutcome::NonIntegral
                } else if gamma == 3 {
                    // U mod 16 != 1: integral but even (x' = 0)
                    UltrametricStepOutcome::IntegralEvenOutsideQ1 { image_x: 0 }
                } else {
                    let image_x = gamma - 3;
                    let image_unit = &expr >> gamma;

                    if image_x < 5 {
                        // U mod 256 != 81: exact v execution but leaves Q_1 (1 <= x' <= 4)
                        UltrametricStepOutcome::ExactButLeavesQ1 {
                            image_x,
                            image_unit,
                        }
                    } else {
                        // U mod 256 == 81: based return to Q_1 (x' >= 5)
                        UltrametricStepOutcome::BasedReturn {
                            next_state: ConcreteUltrametricState::Finite {
                                x: image_x,
                                unit: image_unit,
                            },
                        }
                    }
                }
            }
        }
    }

    /// Verifies the precise commuting diagram \Phi(T_p(k)) == T_p(\Phi(k)).
    pub fn verify_commuting_diagram(
        quotient: &Q1Quotient,
        macrostep: &MacrostepData,
    ) -> Result<bool, AffineError> {
        let q_outcome = QuotientRegisterMachine::eval_transition(macrostep, quotient)?;
        let ultra_init = ConcreteUltrametricState::from_q1_quotient(quotient);

        let ultra_outcome = if macrostep.odd_steps() == 3 {
            Self::step_u(&ultra_init)
        } else {
            Self::step_v_resonant(&ultra_init)
        };

        match (q_outcome, ultra_outcome) {
            (ReturnTransitionOutcome::NotExactWord, UltrametricStepOutcome::NonIntegral) => {
                Ok(true)
            }
            (
                ReturnTransitionOutcome::NotExactWord,
                UltrametricStepOutcome::IntegralEvenOutsideQ1 { .. },
            ) => Ok(true),
            (
                ReturnTransitionOutcome::ExactButLeavesBase { image },
                UltrametricStepOutcome::ExactButLeavesQ1 {
                    image_x,
                    image_unit,
                },
            ) => {
                let l_img = (BigUint::from(11u32) * &image) + 19u32;
                let trailing = l_img.trailing_zeros().unwrap_or(0);
                let x_expected = trailing;
                let u_expected = &l_img >> trailing;

                Ok(image_x == x_expected && image_unit == u_expected)
            }
            (
                ReturnTransitionOutcome::BasedReturn { next_k, .. },
                UltrametricStepOutcome::BasedReturn { next_state },
            ) => {
                let ultra_expected = ConcreteUltrametricState::from_q1_quotient(&next_k);
                Ok(next_state == ultra_expected)
            }
            _ => Ok(false),
        }
    }
}
