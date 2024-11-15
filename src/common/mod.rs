//! <script>
//! IFRAME();
//! </script>
//!
pub mod items;
pub mod table;
pub mod traits;

mod demo;
mod impls;

#[inline]
pub(crate) fn parse_f64<S: AsRef<[u8]>>(
  s: S,
) -> Result<crate::NotNan<f64>, fast_float2::Error> {
  #[expect(clippy::undocumented_unsafe_blocks)]
  fast_float2::parse(s).map(|f| unsafe { crate::NotNan::new_unchecked(f) })
}
