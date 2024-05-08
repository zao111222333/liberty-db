// pub(crate) type f64=f64;

#[inline]
pub(crate) fn float_hash<H: std::hash::Hasher>(state: &mut H, f: f64) {
  use std::hash::Hash;
  let mut buffer = ryu::Buffer::new();
  buffer.format(f).hash(state);
}

use float_cmp::approx_eq;
#[inline]
pub(crate) fn float_eq(a: f64, b: f64) -> bool {
  approx_eq!(f64, a, b, (0.0000000001, 9))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MaxMin {
  Max,
  Min,
}
