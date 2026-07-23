use crate::affine::AffinePrefix;
use crate::valuation::ValuationWord;
use crate::AffineError;
use num_bigint::BigUint;

/// Directional wrapper representing left-to-right sequence execution.
/// `steps[0]` is applied first, then `steps[1]`, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExecutionSequence<T> {
    steps: Vec<T>,
}

impl<T> ExecutionSequence<T> {
    pub fn new(steps: Vec<T>) -> Self {
        Self { steps }
    }

    pub fn steps(&self) -> &[T] {
        &self.steps
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }
}

impl ExecutionSequence<ValuationWord> {
    /// Combines the valuation words left-to-right into a single flattened ValuationWord.
    pub fn flatten_valuation_word(&self) -> ValuationWord {
        let mut flattened = Vec::new();
        for word in &self.steps {
            flattened.extend_from_slice(word.as_slice());
        }
        ValuationWord::from_slice(&flattened)
    }

    /// Derives the composite AffinePrefix corresponding to the left-to-right execution of steps.
    pub fn combined_affine_prefix(&self) -> Result<AffinePrefix, AffineError> {
        let flattened = self.flatten_valuation_word();
        AffinePrefix::from_valuation_word(flattened)
    }
}

impl ExecutionSequence<AffinePrefix> {
    /// Applies steps left-to-right to a starting integer n:
    /// n_1 = step_0(n_0), n_2 = step_1(n_1), ...
    pub fn apply_left_to_right(&self, n: &BigUint) -> Result<BigUint, AffineError> {
        let mut curr = n.clone();
        for step in &self.steps {
            curr = step.apply(&curr)?;
        }
        Ok(curr)
    }
}

/// Directional combinator trait for pairwise composition `u.then(v)`.
pub trait ThenSequence<Rhs = Self> {
    type Output;
    fn then(self, rhs: Rhs) -> Self::Output;
}

impl ThenSequence for ValuationWord {
    type Output = ExecutionSequence<ValuationWord>;
    fn then(self, rhs: Self) -> Self::Output {
        ExecutionSequence::new(vec![self, rhs])
    }
}

impl ThenSequence<ValuationWord> for ExecutionSequence<ValuationWord> {
    type Output = ExecutionSequence<ValuationWord>;
    fn then(mut self, rhs: ValuationWord) -> Self::Output {
        self.steps.push(rhs);
        self
    }
}
