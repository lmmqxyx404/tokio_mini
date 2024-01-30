//! This module abstracts over `loom` and `std::sync` depending on whether we
//! are running tests or not.

#[cfg(not(all(test, loom)))]
mod std;
#[cfg(not(all(test, loom)))]
pub(crate) use self::std::*;
