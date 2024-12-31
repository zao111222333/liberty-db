//! <script>
//! IFRAME();
//! </script>
//!

use ordered_float::NotNan;
pub mod items;
pub mod table;
pub mod traits;

mod demo;
mod impls;
#[inline]
pub(crate) fn parse_f64<S: AsRef<[u8]>>(s: S) -> Result<f64, fast_float2::Error> {
  fast_float2::parse(s)
}
#[inline]
pub(crate) fn f64_into_hash_ord_fn(val: &f64) -> NotNan<f64> {
  unsafe { NotNan::new_unchecked(*val) }
}
