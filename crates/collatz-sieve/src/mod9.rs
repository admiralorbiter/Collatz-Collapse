use crate::traits::{PrefixSieve, PrefixState, RejectionReason, SieveResult};
use roaring::RoaringBitmap;

/// Kinematic Sieve inspecting modular constraints between 2^A_k and 3^j = 9.
/// In Collatz odd steps, 3n+1 mod 9 belongs strictly to {1, 4, 7}.
pub struct Mod9PreimageSieve {
    allowed_mod9: RoaringBitmap,
}

impl Default for Mod9PreimageSieve {
    fn default() -> Self {
        let mut bitmap = RoaringBitmap::new();
        // Allowed odd residues mod 9 after 3n+1 odd step
        bitmap.insert(1);
        bitmap.insert(4);
        bitmap.insert(7);
        Self { allowed_mod9: bitmap }
    }
}

impl PrefixSieve for Mod9PreimageSieve {
    fn name(&self) -> &'static str {
        "Mod9PreimageSieve"
    }

    fn evaluate(&self, state: &PrefixState) -> SieveResult {
        // Fast test for residue compatibility modulo 9
        let r_mod_9 = (&state.affine.starting_residue % 9u32).to_u64_digits().first().cloned().unwrap_or(0);
        let odd_step_mod_9 = (3 * r_mod_9 + 1) % 9;

        if !self.allowed_mod9.contains(odd_step_mod_9 as u32) {
            return SieveResult::Reject {
                reason: RejectionReason::Mod9TernaryContradiction {
                    modulus_2: 9,
                    residue_2: odd_step_mod_9,
                },
            };
        }

        SieveResult::Keep
    }
}
