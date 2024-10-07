// #![cfg_attr(not(feature = "full"), allow(unused_macros))]

#[macro_use]
mod cfg;

#[macro_use]
mod thread_local;

#[macro_use]
mod pin;

// Includes re-exports needed to implement macros
#[doc(hidden)]
pub mod support;
