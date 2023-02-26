pub(crate) type HashMap<K, V> = hashbrown::HashMap<K, V>;
pub(crate) type HashSet<K> = hashbrown::HashSet<K>;

// cfg_if::cfg_if! {
//     if #[cfg(feature = "f32")] {
//         pub(crate) type Float = f32;
//     } else {
//         pub(crate) type Float = f64;
//     }
// }

// #[cfg(feature = "f32")]
pub(crate) type Float = f32;
#[cfg(feature = "f32")]
pub(crate) use std::f32 as floats;
#[cfg(not(feature = "f32"))]
pub(crate) type Float = f64;
#[cfg(not(feature = "f32"))]
pub(crate) use std::f64 as floats;
// pub type Complex = num_complex::Complex<Float>;

use std::hash::Hash;

#[inline]
pub(crate) fn float_hash<H: std::hash::Hasher>(state: &mut H, f: Float) {
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