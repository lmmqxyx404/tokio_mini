cfg_rt! {
  mod current;
  pub(crate) use current::{try_set_current, with_current, SetCurrentGuard};
}
