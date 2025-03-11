//! <script>
//! IFRAME();
//! </script>
//!

use ordered_float::NotNan;
pub mod char_config;
pub mod items;
pub mod traits;

mod demo;
mod impls;
#[inline]
pub(crate) fn parse_f64<S: AsRef<[u8]>>(s: S) -> Result<f64, fast_float2::Error> {
  fast_float2::parse(s)
}
#[inline]
#[expect(clippy::trivially_copy_pass_by_ref)]
pub(crate) const fn f64_into_hash_ord_fn(val: &f64) -> NotNan<f64> {
  // SAFETY: convert float to hash & ord
  unsafe { NotNan::new_unchecked(*val) }
}
