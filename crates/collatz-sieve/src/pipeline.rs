use crate::traits::{PrefixSieve, PrefixState, SieveResult};

/// Sequential or parallel pipeline of PrefixSieves.
pub struct SievePipeline {
    sieves: Vec<Box<dyn PrefixSieve>>,
}

impl Default for SievePipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl SievePipeline {
    pub fn new() -> Self {
        Self { sieves: Vec::new() }
    }

    pub fn add_sieve<S: PrefixSieve + 'static>(mut self, sieve: S) -> Self {
        self.sieves.push(Box::new(sieve));
        self
    }

    /// Evaluates a PrefixState through all sieves in the pipeline.
    /// Short-circuits on first rejection unless benchmark feature is enabled.
    pub fn evaluate(&self, state: &PrefixState) -> SieveResult {
        #[cfg(feature = "benchmark")]
        {
            // Benchmark mode: evaluate ALL sieves to profile overlapping eliminations
            let mut final_result = SieveResult::Keep;
            for sieve in &self.sieves {
                let res = sieve.evaluate(state);
                if matches!(res, SieveResult::Reject { .. })
                    && matches!(final_result, SieveResult::Keep)
                {
                    final_result = res;
                }
            }
            final_result
        }

        #[cfg(not(feature = "benchmark"))]
        {
            for sieve in &self.sieves {
                let res = sieve.evaluate(state);
                if matches!(res, SieveResult::Reject { .. }) {
                    return res;
                }
            }
            SieveResult::Keep
        }
    }

    pub fn sieve_count(&self) -> usize {
        self.sieves.len()
    }
}
