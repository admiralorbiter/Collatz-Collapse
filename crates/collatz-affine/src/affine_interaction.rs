use crate::{AffineError, AffinePrefix, ValuationWord};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Signed, Zero};

/// 2-adic valuation representation supporting finite exponents and infinity (v_2(0) = \infty).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoAdicValuation {
    Finite(u64),
    Infinity,
}

impl TwoAdicValuation {
    /// Computes v_2(n) for a signed BigInt over Z.
    pub fn from_bigint(n: &BigInt) -> Self {
        if n.is_zero() {
            TwoAdicValuation::Infinity
        } else {
            let abs_biguint = n.abs().to_biguint().unwrap();
            let trailing = abs_biguint.trailing_zeros().unwrap_or(0);
            TwoAdicValuation::Finite(trailing)
        }
    }

    /// Checks if valuation is at least the required power-of-two exponent.
    pub fn at_least(&self, required: u64) -> bool {
        match self {
            TwoAdicValuation::Infinity => true,
            TwoAdicValuation::Finite(v) => *v >= required,
        }
    }
}

/// Word-derived macrostep data independent of any specific source cylinder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacrostepData {
    word: ValuationWord,
    odd_steps: usize,
    total_valuation: u64,
    multiplier: BigUint,  // a_p = 3^{K_p}
    denominator: BigUint, // b_p = 2^{A_p}
    constant: BigUint,    // c_p
    constant_int: BigInt, // c_p as BigInt
    d: BigInt,            // d_p = b_p - a_p (signed)
}

impl MacrostepData {
    pub fn from_word(word: ValuationWord) -> Result<Self, AffineError> {
        if word.is_empty() {
            return Err(AffineError::EmptyValuationWord);
        }
        let prefix = AffinePrefix::from_valuation_word(word.clone())?;
        let odd_steps = word.len();
        let total_valuation = word.total_valuation();

        let multiplier = BigUint::from(3u32).pow(odd_steps as u32);
        let denominator = BigUint::one() << total_valuation;
        let constant = prefix.constant.clone();
        let constant_int = BigInt::from_biguint(Sign::Plus, constant.clone());

        let b_int = BigInt::from_biguint(Sign::Plus, denominator.clone());
        let a_int = BigInt::from_biguint(Sign::Plus, multiplier.clone());
        let d = b_int - a_int;

        // For any nonempty valuation word, b_p = 2^A is even while a_p = 3^K is odd.
        // Therefore d_p = b_p - a_p is odd and strictly non-zero over Z.
        assert!(!d.is_zero(), "d_p must be non-zero for nonempty valuation word");

        Ok(Self {
            word,
            odd_steps,
            total_valuation,
            multiplier,
            denominator,
            constant,
            constant_int,
            d,
        })
    }

    pub fn word(&self) -> &ValuationWord {
        &self.word
    }

    pub fn odd_steps(&self) -> usize {
        self.odd_steps
    }

    pub fn total_valuation(&self) -> u64 {
        self.total_valuation
    }

    pub fn multiplier(&self) -> &BigUint {
        &self.multiplier
    }

    pub fn denominator(&self) -> &BigUint {
        &self.denominator
    }

    pub fn constant(&self) -> &BigUint {
        &self.constant
    }

    pub fn constant_int(&self) -> &BigInt {
        &self.constant_int
    }

    pub fn d(&self) -> &BigInt {
        &self.d
    }

    /// Evaluates H_p(n) = d_p * n - c_p over Z.
    pub fn eval_h(&self, n: &BigInt) -> BigInt {
        (&self.d * n) - &self.constant_int
    }
}

/// Generic affine interaction between two macrosteps p and q.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffineInteraction<'a> {
    p: &'a MacrostepData,
    q: &'a MacrostepData,
    delta: BigInt,
    delta_v2: TwoAdicValuation,
}

impl<'a> AffineInteraction<'a> {
    pub fn new(p: &'a MacrostepData, q: &'a MacrostepData) -> Self {
        // \Delta_{p,q} = d_p c_q - d_q c_p
        let delta = (p.d() * q.constant_int()) - (q.d() * p.constant_int());
        let delta_v2 = TwoAdicValuation::from_bigint(&delta);

        Self {
            p,
            q,
            delta,
            delta_v2,
        }
    }

    pub fn p(&self) -> &'a MacrostepData {
        self.p
    }

    pub fn q(&self) -> &'a MacrostepData {
        self.q
    }

    pub fn delta(&self) -> &BigInt {
        &self.delta
    }

    pub fn delta_v2(&self) -> TwoAdicValuation {
        self.delta_v2
    }

    pub fn is_common_center(&self) -> bool {
        self.delta.is_zero()
    }

    /// Division-free verification of same-form eigenidentity:
    /// d_p(a_p n + c_p) - b_p c_p == a_p (d_p n - c_p)
    pub fn same_form_identity_holds(&self) -> bool {
        let b_p = BigInt::from_biguint(Sign::Plus, self.p.denominator().clone());
        let a_p = BigInt::from_biguint(Sign::Plus, self.p.multiplier().clone());
        let c_p = self.p.constant_int();
        let d_p = self.p.d();

        // Check constant term: d_p c_p - b_p c_p == -a_p c_p
        (d_p * c_p) - (&b_p * c_p) == -(&a_p * c_p)
    }

    /// Division-free verification of cross-form identity:
    /// d_p(a_q n + c_q) - b_q c_p == a_q (d_p n - c_p) + \Delta_{p,q}
    pub fn cross_form_identity_holds(&self) -> bool {
        let b_q = BigInt::from_biguint(Sign::Plus, self.q.denominator().clone());
        let c_p = self.p.constant_int();
        let lhs_constant = (self.p.d() * self.q.constant_int()) - (b_q * c_p);

        let a_q = BigInt::from_biguint(Sign::Plus, self.q.multiplier().clone());
        let rhs_constant = (-a_q * c_p) + &self.delta;

        lhs_constant == rhs_constant
    }

    /// Division-free verification of affine commutator identity:
    /// C_{[q,p]} - C_{[p,q]} == -\Delta_{p,q}
    /// where C_{[p,q]} = a_q c_p + b_p c_q and C_{[q,p]} = a_p c_q + b_q c_p
    pub fn commutator_identity_holds(&self) -> bool {
        let a_p = BigInt::from_biguint(Sign::Plus, self.p.multiplier().clone());
        let b_p = BigInt::from_biguint(Sign::Plus, self.p.denominator().clone());
        let c_p = self.p.constant_int();

        let a_q = BigInt::from_biguint(Sign::Plus, self.q.multiplier().clone());
        let b_q = BigInt::from_biguint(Sign::Plus, self.q.denominator().clone());
        let c_q = self.q.constant_int();

        let c_pq = (&a_q * c_p) + (&b_p * c_q);
        let c_qp = (&a_p * c_q) + (&b_q * c_p);

        (c_qp - c_pq) == -&self.delta
    }

    /// Evaluates the 2-adic valuation v_2(a_q * H_p(n) + \Delta_{p,q}) for a given n.
    pub fn eval_cross_form_valuation(&self, n: &BigInt) -> TwoAdicValuation {
        let a_q = BigInt::from_biguint(Sign::Plus, self.q.multiplier().clone());
        let h_p = self.p.eval_h(n);
        let expr = (a_q * h_p) + &self.delta;
        TwoAdicValuation::from_bigint(&expr)
    }

    /// Diagnostic evaluation returning both sides of same-form identity for concrete n.
    pub fn evaluate_same_form_sides(&self, n: &BigInt) -> (BigInt, BigInt) {
        let a_p = BigInt::from_biguint(Sign::Plus, self.p.multiplier().clone());
        let b_p = BigInt::from_biguint(Sign::Plus, self.p.denominator().clone());
        let c_p = self.p.constant_int();

        let lhs = (self.p.d() * ((&a_p * n) + c_p)) - (&b_p * c_p);
        let rhs = a_p * self.p.eval_h(n);
        (lhs, rhs)
    }

    /// Diagnostic evaluation returning both sides of cross-form identity for concrete n.
    pub fn evaluate_cross_form_sides(&self, n: &BigInt) -> (BigInt, BigInt) {
        let a_q = BigInt::from_biguint(Sign::Plus, self.q.multiplier().clone());
        let b_q = BigInt::from_biguint(Sign::Plus, self.q.denominator().clone());
        let c_q = self.q.constant_int();

        let lhs = (self.p.d() * ((&a_q * n) + c_q)) - (&b_q * self.p.constant_int());
        let rhs = (a_q * self.p.eval_h(n)) + &self.delta;
        (lhs, rhs)
    }
}
