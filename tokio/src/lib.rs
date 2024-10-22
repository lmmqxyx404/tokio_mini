// allow：这个指令告诉编译器忽略指定的警告或错误。
// unknown_lints：忽略未知的 lint 警告。这意味着如果编译器遇到一个它不认识的 lint，它不会报错。这有助于向前兼容性。
// unexpected_cfgs：忽略对未知 cfg 的警告，比如条件编译标志，保证编译过程中不会因为未知的配置条件而报错。
#![allow(unknown_lints, unexpected_cfgs)]
// todo: set cfg_attr for `loom`
// #![cfg_attr(loom, allow(dead_code, unreachable_pub))]

// Includes re-exports used by macros.
//
// This module is not intended to be part of the public API. In general, any
// `doc(hidden)` code is not part of Tokio's public and stable API.
/// 4
#[macro_use]
#[doc(hidden)]
pub mod macros;

// pub mod task;
pub mod runtime;

// 仅仅引入包
mod loom;

mod util;

cfg_sync! {
  pub mod sync;
}

cfg_not_sync! {
  mod sync;
}

cfg_time! {
  pub mod time;
}
