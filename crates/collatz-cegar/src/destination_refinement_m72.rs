#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefinementRequirement {
    pub current_exponent: u32,
    pub total_valuation: u32,
    pub target_exponent: u32,
    pub required_exponent: u32,
    pub additional_bits: u32,
}

impl RefinementRequirement {
    /// Incremental precision formula: h_add = max(0, A + q_t - M_curr)
    pub fn compute(m_curr: u32, total_valuation_a: u32, target_q: u32) -> Self {
        let required = total_valuation_a + target_q;
        let additional = if required > m_curr {
            required - m_curr
        } else {
            0
        };

        Self {
            current_exponent: m_curr,
            total_valuation: total_valuation_a,
            target_exponent: target_q,
            required_exponent: required,
            additional_bits: additional,
        }
    }
}

pub struct DestinationRefinementEngine;

impl DestinationRefinementEngine {
    /// Refines source residue r mod 2^m into 2^h_add subcells mod 2^(m + h_add)
    pub fn generate_subcells(
        residue: u64,
        current_exp: u32,
        additional_bits: u32,
    ) -> Vec<(u64, u32)> {
        if additional_bits == 0 {
            return vec![(residue, current_exp)];
        }

        let num_subcells = 1u64 << additional_bits;
        let step = 1u64 << current_exp;
        let new_exp = current_exp + additional_bits;
        let mut result = Vec::with_capacity(num_subcells as usize);

        for t in 0..num_subcells {
            let sub_r = residue + t * step;
            result.push((sub_r, new_exp));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremental_precision_formula() {
        // Source M_curr=5 (32), A=4 ([1,1,2]), target q=6 (64)
        // required = 4 + 6 = 10, additional = max(0, 10 - 5) = 5
        let req = RefinementRequirement::compute(5, 4, 6);
        assert_eq!(req.required_exponent, 10);
        assert_eq!(req.additional_bits, 5);

        // If source is already M_curr=12, additional bits = max(0, 10 - 12) = 0
        let req2 = RefinementRequirement::compute(12, 4, 6);
        assert_eq!(req2.additional_bits, 0);
    }

    #[test]
    fn test_subcell_generation() {
        let subcells = DestinationRefinementEngine::generate_subcells(7, 5, 2);
        assert_eq!(subcells.len(), 4);
        assert_eq!(subcells[0], (7, 7));
        assert_eq!(subcells[1], (39, 7));
        assert_eq!(subcells[2], (71, 7));
        assert_eq!(subcells[3], (103, 7));
    }
}
