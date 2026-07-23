use crate::{AffineError, AffinePrefix, ValuationWord};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Signed, Zero};

/// 2-adic valuation representation supporting finite exponents and infinity (v_2(0) = \infty).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TwoAdicValuation {
    Finite(u64),
    #[default]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        assert!(
            !d.is_zero(),
            "d_p must be non-zero for nonempty valuation word"
        );

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

/// Canonical 2-adic odd rational fixed point representation: \xi_v = -\beta_v / (Q_v - M_v) < 0.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OddRational2Adic {
    numerator: BigInt,   // -\beta_v < 0
    denominator: BigInt, // d_v = Q_v - M_v > 0 (odd)
}

impl OddRational2Adic {
    /// Canonical fixed-point constructor enforcing \xi_v = -\beta_v / (Q_v - M_v).
    pub fn fixed_point(m_v: &BigUint, q_v: &BigUint, beta_v: &BigUint) -> Result<Self, AffineError> {
        let m_int = BigInt::from_biguint(Sign::Plus, m_v.clone());
        let q_int = BigInt::from_biguint(Sign::Plus, q_v.clone());
        let beta_int = BigInt::from_biguint(Sign::Plus, beta_v.clone());

        let d_v = &q_int - &m_int;
        if d_v <= BigInt::zero() {
            return Err(AffineError::Overflow);
        }

        let numerator = -&beta_int;
        let denominator = d_v;

        Ok(Self {
            numerator,
            denominator,
        })
    }

    pub fn numerator(&self) -> &BigInt {
        &self.numerator
    }

    pub fn denominator(&self) -> &BigInt {
        &self.denominator
    }

    pub fn is_negative(&self) -> bool {
        self.numerator.is_negative()
    }
}

/// Canonical return core data holding M_v = 2^{B_v}, Q_v = 3^{E_v}, \beta_v = c_v, and d_v = Q_v - M_v.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PeriodicReturnCore {
    data: MacrostepData,
    fixed_point: OddRational2Adic,
}

impl PeriodicReturnCore {
    pub fn new(word: ValuationWord) -> Result<Self, AffineError> {
        let data = MacrostepData::from_word(word)?;
        // Canonical return convention: M_v = 2^{B_v} (denominator), Q_v = 3^{E_v} (multiplier)
        let m_v = data.denominator().clone();
        let q_v = data.multiplier().clone();
        let beta_v = data.constant().clone();

        let fixed_point = OddRational2Adic::fixed_point(&m_v, &q_v, &beta_v)?;
        Ok(Self { data, fixed_point })
    }

    pub fn data(&self) -> &MacrostepData {
        &self.data
    }

    pub fn fixed_point(&self) -> &OddRational2Adic {
        &self.fixed_point
    }

    pub fn m_v(&self) -> &BigUint {
        self.data.denominator()
    }

    pub fn q_v(&self) -> &BigUint {
        self.data.multiplier()
    }

    pub fn beta_v(&self) -> &BigUint {
        self.data.constant()
    }

    /// Returns d_v = Q_v - M_v > 0.
    pub fn d_v(&self) -> BigInt {
        let q_int = BigInt::from_biguint(Sign::Plus, self.q_v().clone());
        let m_int = BigInt::from_biguint(Sign::Plus, self.m_v().clone());
        q_int - m_int
    }

    /// Evaluates canonical return map F_v(D) = (Q_v * D + beta_v) / M_v over Z.
    pub fn eval_map(&self, d: &BigInt) -> Option<BigInt> {
        let q_int = BigInt::from_biguint(Sign::Plus, self.q_v().clone());
        let m_int = BigInt::from_biguint(Sign::Plus, self.m_v().clone());
        let beta_int = self.data.constant_int();

        let num = (q_int * d) + beta_int;
        if (&num % &m_int).is_zero() {
            Some(num / m_int)
        } else {
            None
        }
    }

    /// Evaluates the Integer Primitive Form A_v(D) = d_v * D + \beta_v.
    pub fn eval_integer_primitive(&self, d: &BigInt) -> BigInt {
        (self.d_v() * d) + self.data.constant_int()
    }

    /// Returns the authoritative CANONICAL_RETURN_CONVENTION_V1 ASCII formula.
    pub fn canonical_ascii_definition(&self) -> &'static str {
        "F_v(D) = (Q_v * D + beta_v) / M_v"
    }
}

/// The 4-case Core Switch Classification Type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreSwitchType {
    SameCore,
    Inherited,
    Reset,
    Resonant,
}

/// The outcome of unit cancellation in a resonant core switch (s = \kappa).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResonanceOutcome {
    FiniteGain { extra_bits: u64 },
    ExactCore,
}

/// Detailed evaluation result for an integer core switch transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreSwitchResult {
    pub switch_type: CoreSwitchType,
    pub incoming_depth: TwoAdicValuation,
    pub core_separation_kappa: TwoAdicValuation,
    pub outgoing_depth: TwoAdicValuation,
    pub resonance_unit_cancellation: Option<u64>,
    pub resonance_outcome: Option<ResonanceOutcome>,
}

/// Core interaction kernel providing exact arithmetic for core separation, commutators, and switch budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreInteractionKernel<'a> {
    v: &'a PeriodicReturnCore,
    w: &'a PeriodicReturnCore,
    gamma: BigInt,
    kappa: TwoAdicValuation,
}

impl<'a> CoreInteractionKernel<'a> {
    pub fn new(v: &'a PeriodicReturnCore, w: &'a PeriodicReturnCore) -> Self {
        // \Gamma_{v,w} = d_v \beta_w - d_w \beta_v where d_v = Q_v - M_v > 0
        let gamma = (v.d_v() * w.data().constant_int()) - (w.d_v() * v.data().constant_int());
        let kappa = TwoAdicValuation::from_bigint(&gamma);
        Self { v, w, gamma, kappa }
    }

    pub fn v(&self) -> &'a PeriodicReturnCore {
        self.v
    }

    pub fn w(&self) -> &'a PeriodicReturnCore {
        self.w
    }

    pub fn gamma(&self) -> &BigInt {
        &self.gamma
    }

    pub fn kappa(&self) -> TwoAdicValuation {
        self.kappa
    }

    pub fn are_cores_commuting(&self) -> bool {
        self.gamma.is_zero()
    }

    /// Evaluates the 4-case exact integer core switch law: d_v A_w(D) = d_w A_v(D) + \Gamma_{v,w}.
    pub fn evaluate_integer_switch(&self, a_v: &BigInt) -> CoreSwitchResult {
        let s = TwoAdicValuation::from_bigint(a_v);
        let kappa = self.kappa;

        match (s, kappa) {
            (_, TwoAdicValuation::Infinity) => CoreSwitchResult {
                switch_type: CoreSwitchType::SameCore,
                incoming_depth: s,
                core_separation_kappa: TwoAdicValuation::Infinity,
                outgoing_depth: s,
                resonance_unit_cancellation: None,
                resonance_outcome: None,
            },
            (TwoAdicValuation::Finite(s_val), TwoAdicValuation::Finite(k_val)) => {
                if s_val < k_val {
                    CoreSwitchResult {
                        switch_type: CoreSwitchType::Inherited,
                        incoming_depth: s,
                        core_separation_kappa: kappa,
                        outgoing_depth: s,
                        resonance_unit_cancellation: None,
                        resonance_outcome: None,
                    }
                } else if s_val > k_val {
                    CoreSwitchResult {
                        switch_type: CoreSwitchType::Reset,
                        incoming_depth: s,
                        core_separation_kappa: kappa,
                        outgoing_depth: TwoAdicValuation::Finite(k_val),
                        resonance_unit_cancellation: None,
                        resonance_outcome: None,
                    }
                } else {
                    // Resonant case: s_val == k_val
                    let d_w = self.w.d_v();
                    let u = (d_w * a_v) >> k_val;
                    let g = &self.gamma >> k_val;
                    let sum = u + g;
                    let cancellation = TwoAdicValuation::from_bigint(&sum);

                    let (outgoing, extra_bits, outcome) = match cancellation {
                        TwoAdicValuation::Finite(c) => (
                            TwoAdicValuation::Finite(k_val + c),
                            Some(c),
                            Some(ResonanceOutcome::FiniteGain { extra_bits: c }),
                        ),
                        TwoAdicValuation::Infinity => (
                            TwoAdicValuation::Infinity,
                            None,
                            Some(ResonanceOutcome::ExactCore),
                        ),
                    };

                    CoreSwitchResult {
                        switch_type: CoreSwitchType::Resonant,
                        incoming_depth: s,
                        core_separation_kappa: kappa,
                        outgoing_depth: outgoing,
                        resonance_unit_cancellation: extra_bits,
                        resonance_outcome: outcome,
                    }
                }
            }
            (TwoAdicValuation::Infinity, TwoAdicValuation::Finite(k_val)) => CoreSwitchResult {
                switch_type: CoreSwitchType::Reset,
                incoming_depth: s,
                core_separation_kappa: kappa,
                outgoing_depth: TwoAdicValuation::Finite(k_val),
                resonance_unit_cancellation: None,
                resonance_outcome: None,
            },
        }
    }
}

