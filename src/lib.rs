pub mod domain;
pub mod plan;
pub mod problem;
mod token;

/// Denotes the error msg and the location at which it was encountered
pub type Error = (&'static str, std::ops::Range<usize>);
pub type Result<T> = std::result::Result<T, Error>;
