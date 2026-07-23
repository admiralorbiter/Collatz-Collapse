use collatz_affine::{AffineError, Q1Quotient, TwoAdicValuation};
use num_bigint::BigUint;
use num_traits::Zero;

/// Region partition for valuation x = v_2(L_u(n)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValuationRegion {
    X5,
    X6,
    X7,
    X8,
    XGe9,
    Infinity,
}

impl ValuationRegion {
    pub fn from_x(x: TwoAdicValuation) -> Self {
        match x {
            TwoAdicValuation::Infinity => ValuationRegion::Infinity,
            TwoAdicValuation::Finite(v) => match v {
                5 => ValuationRegion::X5,
                6 => ValuationRegion::X6,
                7 => ValuationRegion::X7,
                8 => ValuationRegion::X8,
                _ if v >= 9 => ValuationRegion::XGe9,
                _ => ValuationRegion::X5,
            },
        }
    }
}

/// Ultrametric state (x, U) where L_u(n) = 2^x * U (U odd).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UltrametricState {
    pub x: TwoAdicValuation,
    pub u_odd: BigUint,
    pub region: ValuationRegion,
}

impl UltrametricState {
    pub fn from_q1_quotient(quotient: &Q1Quotient) -> Self {
        let k = quotient.value();
        // L_u(n) = 32 * (11k + 3)
        let expr = (BigUint::from(11u32) * k) + 3u32;
        let trailing = expr.trailing_zeros().unwrap_or(0);
        let x = TwoAdicValuation::Finite(5 + trailing);
        let u_odd = &expr >> trailing;
        let region = ValuationRegion::from_x(x);

        Self { x, u_odd, region }
    }

    pub fn from_n(n: &BigUint) -> Self {
        let q = Q1Quotient::from_integer(n).unwrap();
        Self::from_q1_quotient(&q)
    }

    /// Exact u transition on (x, U): x' = x - 4, U' = 27U.
    pub fn step_u(&self) -> Result<Self, AffineError> {
        match self.x {
            TwoAdicValuation::Infinity => Ok(Self {
                x: TwoAdicValuation::Infinity,
                u_odd: BigUint::zero(),
                region: ValuationRegion::Infinity,
            }),
            TwoAdicValuation::Finite(v) => {
                if v < 4 {
                    return Err(AffineError::Overflow);
                }
                let next_x = TwoAdicValuation::Finite(v - 4);
                let next_u_odd = &self.u_odd * 27u32;
                let region = ValuationRegion::from_x(next_x);
                Ok(Self {
                    x: next_x,
                    u_odd: next_u_odd,
                    region,
                })
            }
        }
    }

    /// Exact resonant v transition on layer x = 6: x' = \gamma - 3, U' = (729U + 87) / 2^\gamma.
    pub fn step_v_resonant(&self) -> Result<Self, AffineError> {
        if self.region != ValuationRegion::X6 {
            return Err(AffineError::Overflow);
        }

        // Check exact v execution condition: U \equiv 1 (mod 16)
        let u_mod16 = (&self.u_odd % 16u32)
            .to_u64_digits()
            .first()
            .copied()
            .unwrap_or(0);
        if u_mod16 != 1 {
            return Err(AffineError::Overflow);
        }

        // Resonant affine transform: 729 * U + 87
        let expr = (BigUint::from(729u32) * &self.u_odd) + 87u32;
        let gamma = expr.trailing_zeros().unwrap_or(0);

        if gamma < 3 {
            return Err(AffineError::Overflow);
        }

        let next_x = TwoAdicValuation::Finite(gamma - 3);
        let next_u_odd = &expr >> gamma;
        let region = ValuationRegion::from_x(next_x);

        Ok(Self {
            x: next_x,
            u_odd: next_u_odd,
            region,
        })
    }
}

/// Differential validator checking ultrametric transitions against exact quotient machine.
pub struct UltrametricMachineValidator;

impl UltrametricMachineValidator {
    pub fn validate_u_conformance(k_val: &BigUint) -> bool {
        let q = Q1Quotient::from_k(k_val.clone());
        let ultra_init = UltrametricState::from_q1_quotient(&q);

        let q_out = collatz_affine::QuotientRegisterMachine::eval_u_transition(&q);
        let ultra_out = ultra_init.step_u().unwrap();

        match q_out {
            collatz_affine::ReturnTransitionOutcome::BasedReturn { next_k, .. } => {
                let ultra_next = UltrametricState::from_q1_quotient(&next_k);
                ultra_out.x == ultra_next.x && ultra_out.u_odd == ultra_next.u_odd
            }
            collatz_affine::ReturnTransitionOutcome::ExactButLeavesBase { image } => {
                // Check if image corresponds to ultra_out in n-space
                let l_img = (BigUint::from(11u32) * &image) + 19u32;
                let trailing = l_img.trailing_zeros().unwrap_or(0);
                let x_expected = TwoAdicValuation::Finite(trailing);
                let u_expected = &l_img >> trailing;

                ultra_out.x == x_expected && ultra_out.u_odd == u_expected
            }
            _ => false,
        }
    }
}
