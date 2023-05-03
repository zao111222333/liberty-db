pub(crate) type Float=f64;


#[inline]
pub(crate) fn float_hash<H: std::hash::Hasher>(state: &mut H, f: Float) {
    use std::hash::Hash;
    format!("{:.10E}",f).hash(state);
}

use float_cmp::approx_eq;
#[inline]
pub(crate) fn float_eq(a: Float, b: Float) -> bool {
    approx_eq!(Float, a, b, (0.0000000001,9))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaxMin {
    Max,
    Min,
}