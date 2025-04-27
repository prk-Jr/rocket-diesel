#[cfg(feature = "postgresql")]
pub mod pg;

#[cfg(test)]
pub mod test_utils;

#[cfg(feature = "postgresql")]
pub use pg::*;

#[cfg(test)]
pub use test_utils::*;

/// Always export a `establish_connection_pool()`
/// that uses the correct backend depending on context.
#[cfg(not(test))]
pub use pg::establish_connection_pool;

#[cfg(test)]
pub use test_utils::establish_connection_pool;
