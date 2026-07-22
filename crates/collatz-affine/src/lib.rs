pub mod affine;
pub mod inversion;
pub mod valuation;

pub use affine::{compute_descent_threshold, AffineDiagnostics, AffinePrefix};
pub use inversion::{hensel_inverse_3_pow, modular_inverse_3k_mod_2A, solve_starting_residue};
pub use valuation::ValuationWord;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum AffineError {
    #[error("Empty valuation word passed to affine computation")]
    EmptyValuationWord,

    #[error("Valuation element zero encountered at index {0}")]
    ZeroValuation(usize),

    #[error("Integer overflow encountered during affine calculation")]
    Overflow,

    #[error("Modular inverse does not exist for modulus {0}")]
    NoModularInverse(String),
}
