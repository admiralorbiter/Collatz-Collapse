pub mod arithmetic;
pub mod odd_map;
pub mod ordinary;

pub use arithmetic::CollatzInt;
pub use odd_map::{odd_step, odd_step_u128, OddStep};
pub use ordinary::{collatz_step, stopping_time, trajectory_prefix};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum CoreError {
    #[error("Even number input passed to odd-only map: {0}")]
    EvenInput(String),

    #[error("Integer overflow encountered during calculation")]
    Overflow,

    #[error("Zero or invalid input for Collatz computation")]
    InvalidInput,
}
