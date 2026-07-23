use crate::extremal_source_search::CanonicalGuardedWord;
use num_bigint::BigUint;
use num_traits::Zero;
use serde::{Deserialize, Serialize};

/// Zero-tail profile representing high-order zero bits and lift-block structural metrics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ZeroTailProfile {
    /// Total precision in bits B_w
    pub precision_bits: u64,
    /// Absolute bit length of canonical source residue \ell(\rho_w), 0 if \rho_w == 0
    pub source_bit_length: u64,
    /// High-order zero bits Z(w) = B_w - \ell(\rho_w)
    pub total_high_zero_bits: u64,
    /// Count of consecutive terminal lift blocks with \Lambda_i == 0
    pub terminal_zero_block_count: usize,
    /// Sum of widths of terminal zero lift blocks
    pub terminal_zero_block_width: u64,
    /// Number of leading zero bits inside the last nonzero lift block \Lambda_m
    pub leading_zero_bits_in_last_nonzero_block: u64,
    /// Normalized zero-tail ratio \zeta_w = Z(w) / B_w
    pub normalized_zero_tail: f64,
}

impl ZeroTailProfile {
    /// Verify structural invariant:
    /// total_high_zero_bits == terminal_zero_block_width + leading_zero_bits_in_last_nonzero_block
    pub fn verify_lift_block_invariants(&self) -> bool {
        let expected_zero_bits =
            self.terminal_zero_block_width + self.leading_zero_bits_in_last_nonzero_block;
        self.total_high_zero_bits == expected_zero_bits
    }

    /// Compute ZeroTailProfile from a CanonicalGuardedWord by decomposing \rho_w into lift blocks.
    pub fn from_canonical_word(word: &CanonicalGuardedWord) -> Self {
        let precision_bits = word.affine.denominator.bits() as u64 - 1;
        let rho = &word.source_residue;

        if rho.is_zero() {
            return Self {
                precision_bits,
                source_bit_length: 0,
                total_high_zero_bits: precision_bits,
                terminal_zero_block_count: word.gap_sequence.len(),
                terminal_zero_block_width: precision_bits,
                leading_zero_bits_in_last_nonzero_block: 0,
                normalized_zero_tail: 1.0,
            };
        }

        let source_bit_length = rho.bits() as u64;
        let total_high_zero_bits = precision_bits.saturating_sub(source_bit_length);

        // Decompose \rho_w into lift blocks \Lambda_i
        let mut block_widths = Vec::with_capacity(word.gap_sequence.len());
        for &j in &word.gap_sequence {
            block_widths.push(9 + 4 * j);
        }

        let mut lambda_blocks = Vec::with_capacity(block_widths.len());
        let mut current_shift = 0u64;

        for &width in &block_widths {
            let mask = (BigUint::from(1u32) << width) - 1u32;
            let block_val = (rho >> current_shift) & mask;
            lambda_blocks.push(block_val);
            current_shift += width;
        }

        // Find last nonzero block index m (1-based index)
        let mut m_idx = 0;
        for (idx, block) in lambda_blocks.iter().enumerate().rev() {
            if !block.is_zero() {
                m_idx = idx + 1;
                break;
            }
        }

        let terminal_zero_block_count = lambda_blocks.len() - m_idx;
        let mut terminal_zero_block_width = 0u64;
        for i in m_idx..lambda_blocks.len() {
            terminal_zero_block_width += block_widths[i];
        }

        let leading_zero_bits_in_last_nonzero_block = if m_idx > 0 {
            let m = m_idx - 1;
            let lambda_m_bits = lambda_blocks[m].bits() as u64;
            block_widths[m].saturating_sub(lambda_m_bits)
        } else {
            0
        };

        let normalized_zero_tail = if precision_bits > 0 {
            total_high_zero_bits as f64 / precision_bits as f64
        } else {
            0.0
        };

        let profile = Self {
            precision_bits,
            source_bit_length,
            total_high_zero_bits,
            terminal_zero_block_count,
            terminal_zero_block_width,
            leading_zero_bits_in_last_nonzero_block,
            normalized_zero_tail,
        };

        debug_assert!(
            profile.verify_lift_block_invariants(),
            "Lift-block zero-tail invariant failure: {:?}",
            profile
        );

        profile
    }
}
