//! All item structure inside
//! `Timing`.
#![allow(clippy::multiple_inherent_impl)]
use crate::{
  Ctx, Group,
  ast::{
    self, BuilderScope, GroupComments, GroupFn, ParsingBuilder, fmt_comment_liberty,
  },
  common::f64_into_hash_ord_fn,
  expression::logic,
  table::{
    DisplayTableLookUp, DisplayValues, OcvSigmaTable, OcvSigmaTableBuilder, SigmaType,
    TableLookUp2D, TableLookUp2DBuilder,
  },
};
use core::iter::zip;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Not as _, Sub};
use itertools::izip;
use strum::{Display, EnumString};
/// The `timing_sense` attribute describes the way an input pin logically affects an output pin.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=328.32+329.2+330.2&end=328.33+329.39+330.6
/// ">Reference</a>
///
/// #### Syntax
/// `timing_sense : positive_unate | negative_unate | non_unate ;`
///
/// `positive_unate`: Combines incoming rise delays with local rise delays and
/// compares incoming fall delays with local fall delays.
///
/// `negative_unate`: Combines incoming rise delays with local fall delays and
/// compares incoming fall delays with local rise delays.
///
/// `non_unate`: Combines local delays with the worst-case incoming delay value.
/// The non-unate timing sense represents a function whose output value change cannot
/// be determined from the direction of the change in the input value.
///
/// Timing sense is derived from the logic function of a pin. For example, the value derived for
/// an AND gate is `positive_unate`, the value for a NAND gate is `negative_unate`, and the value
/// for an XOR gate is `non_unate`.
///
/// A function is said to be unate if a rising (falling) change on a positive (negative) unate
/// input variable causes the output function variable to rise (fall) or not change.
/// For a non-unate variable, further state information is required to determine the effects of
/// a particular state transition.
///
/// You can specify half-unate sequential timing arcs if the `timing_type` value is either
/// `rising_edge` or `falling_edge` and the `timing_sense` value is either `positive_unate`
/// or `negative_unate`.
/// + In the case of `rising_edge` and `positive_unate` values, only the `cell_rise` and `rise_transition` information is required.
/// + In the case of `rising_edge` and `negative_unate` values, only the `cell_fall` and `fall_transition` information is required.
/// + In the case of `falling_edge` and `positive_unate` values, only the `cell_rise` and `rise_transition` information is required.
/// + In the case of `falling_edge` and `negative_unate` values, only the `cell_fall` and `fall_transition` information is required.
///
/// Do not define the `timing_sense` value of a pin, except when you need to override the derived value
/// or when you are characterizing a noncombinational gate such as a three-state component. For example,
/// you might want to define the timing sense manually when you model multiple paths between
/// an input pin and an output pin, such as in an XOR gate.
///
/// It is possible that one path is positive unate while another is negative unate. In this case,
/// the first timing arc is given a `positive_unate` designation and the second is given a `negative_unate`
/// designation.
///
/// Timing arcs with a timing type of `clear` or `preset` require a `timing_sense` attribute.
/// If `related_pin` is an output pin, you must define a `timing_sense` attribute for that pin.
#[derive(
  Debug, Clone, Copy, PartialEq, Display, EnumString, Default, Hash, Eq, PartialOrd, Ord
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TimingSenseType {
  /// Combines incoming `rise` delays with local `rise` delays
  /// and compares incoming `fall` delays with local `fall` delays.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=329.5&end=329.6
  /// ">Reference</a>
  #[strum(serialize = "positive_unate")]
  PositiveUnate,
  /// Combines incoming `rise` delays with local `fall` delays
  /// and compares incoming `fall` delays with local `rise` delays.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=329.8&end=329.9
  /// ">Reference</a>
  #[strum(serialize = "negative_unate")]
  NegativeUnate,
  /// Combines local delays with the `worst-case` incoming delay value.
  /// The non-unate timing sense represents a function whose
  /// output value change cannot be determined from the direction
  /// of the change in the input value.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=329.11&end=329.13
  /// ">Reference</a>
  #[strum(serialize = "non_unate")]
  #[default]
  NonUnate,
}

impl TimingSenseType {
  #[must_use]
  #[inline]
  pub fn compute_edge(&self, pin_edge: &logic::Edge) -> Option<logic::Edge> {
    match self {
      Self::PositiveUnate => Some(*pin_edge),
      Self::NegativeUnate => Some(pin_edge.not()),
      Self::NonUnate => None,
    }
  }
}
crate::ast::impl_self_builder!(TimingSenseType);
crate::ast::impl_simple!(TimingSenseType);

/// The `cell_degradation`  group describes a cell performance degradation
/// design rule for compiling a design.
///
/// A cell degradation design rule specifies the maximum capacitive load
/// a cell can drive without causing cell performance degradation during the fall transition.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=347.33&end=347.35
/// ">Reference</a>
///
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct CellDegradation<C: 'static + Ctx> {
  /// name
  #[liberty(name)]
  #[id(borrow = str)]
  pub name: String,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: ast::Attributes,
  /// /* lookup table */
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=348.6&end=348.7
  /// ">Reference</a>
  #[liberty(complex)]
  pub index_1: Vec<f64>,
  /// /* lookup table */
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=348.6&end=348.7
  /// ">Reference</a>
  #[liberty(complex)]
  pub values: Vec<f64>,
}
impl<C: 'static + Ctx> GroupFn<C> for CellDegradation<C> {}

#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TimingTableLookUp<C: 'static + Ctx> {
  pub extra_ctx: C::Table,
  pub name: String,
  pub comments: String,
  pub index_1: Vec<f64>,
  pub index_2: Vec<f64>,
  pub values: Vec<f64>,
  pub lvf_moments_values: Vec<LVFMoments>,
  pub lvf_early_late_values: Vec<LVFEarlyLate>,
}
#[expect(clippy::indexing_slicing, clippy::arithmetic_side_effects)]
impl<C: 'static + Ctx> TimingTableLookUp<C> {
  #[inline]
  pub(crate) fn use_common_template(
    table: &mut Option<Self>,
    scope: &mut BuilderScope<C>,
  ) {
    if let Some(t) = table {
      #[cfg(feature = "lut_template")]
      crate::table::TableCtx::set_lut_template(
        &mut t.extra_ctx,
        scope.lu_table_template.get(&t.name),
      );
    }
  }
  #[inline]
  const fn find_pos(len: usize, pos: usize) -> Option<(usize, usize)> {
    if len <= 1 {
      None
    } else {
      Some(if pos == 0 {
        (0, 1)
      } else if pos == len {
        (len - 2, len - 1)
      } else {
        (pos - 1, pos)
      })
    }
  }
  #[inline]
  fn get_value(&self, ix: usize, iy: usize) -> Option<f64> {
    self.values.get(ix * self.index_2.len() + iy).copied()
  }
  #[inline]
  fn get_lvf_moments_value(&self, ix: usize, iy: usize) -> Option<LVFMoments> {
    self.lvf_moments_values.get(ix * self.index_2.len() + iy).copied()
  }
  /// The linear interpolation & extrapolation
  #[must_use]
  #[inline]
  #[expect(clippy::float_arithmetic)]
  pub fn lookup(&self, idx1: &f64, idx2: &f64) -> Option<f64> {
    let idx1_ = f64_into_hash_ord_fn(idx1);
    let idx2_ = f64_into_hash_ord_fn(idx2);
    match self.index_1.binary_search_by(|v| f64_into_hash_ord_fn(v).cmp(&idx1_)) {
      Ok(i1_) => {
        match self.index_2.binary_search_by(|v| f64_into_hash_ord_fn(v).cmp(&idx2_)) {
          Ok(i_1) => self.get_value(i1_, i_1),
          Err(pos2) => {
            let (i_1, i_2) = Self::find_pos(self.index_2.len(), pos2)?;
            let q_1 = self.get_value(i1_, i_1)?;
            let q_2 = self.get_value(i1_, i_2)?;
            let x_1 = self.index_2[i_1];
            let x_2 = self.index_2[i_2];
            // q_1 + (q_2 - q_1) * ((idx2 - x_1) / (x_2 - x_1))
            Some((q_2 - q_1).mul_add((idx2 - x_1) / (x_2 - x_1), q_1))
          }
        }
      }
      Err(pos1) => {
        let (i1_, i2_) = Self::find_pos(self.index_1.len(), pos1)?;
        let x1_ = self.index_1[i1_];
        let x2_ = self.index_1[i2_];
        match self.index_2.binary_search_by(|v| f64_into_hash_ord_fn(v).cmp(&idx2_)) {
          Ok(i_1) => {
            let q1_ = self.get_value(i1_, i_1)?;
            let q2_ = self.get_value(i2_, i_1)?;
            // Some(q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_)))
            Some((q2_ - q1_).mul_add((idx1 - x1_) / (x2_ - x1_), q1_))
          }
          Err(pos2) => {
            let (i_1, i_2) = Self::find_pos(self.index_2.len(), pos2)?;
            let q11 = self.get_value(i1_, i_1)?;
            let q12 = self.get_value(i1_, i_2)?;
            let q21 = self.get_value(i2_, i_1)?;
            let q22 = self.get_value(i2_, i_2)?;
            let x_1 = self.index_2[i_1];
            let x_2 = self.index_2[i_2];
            // let q1_ = q11 + (q12 - q11) * ((idx2 - x_1) / (x_2 - x_1));
            let q1_ = (q12 - q11).mul_add((idx2 - x_1) / (x_2 - x_1), q11);
            // let q2_ = q21 + (q22 - q21) * ((idx2 - x_1) / (x_2 - x_1));
            let q2_ = (q22 - q21).mul_add((idx2 - x_1) / (x_2 - x_1), q21);
            // q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_))
            Some((q2_ - q1_).mul_add((idx1 - x1_) / (x2_ - x1_), q1_))
          }
        }
      }
    }
  }
  #[must_use]
  #[inline]
  #[expect(clippy::float_arithmetic)]
  pub fn lookup_lvf_moments(&self, idx1: &f64, idx2: &f64) -> Option<LVFMoments> {
    let idx1_ = f64_into_hash_ord_fn(idx1);
    let idx2_ = f64_into_hash_ord_fn(idx2);
    match self.index_1.binary_search_by(|v| f64_into_hash_ord_fn(v).cmp(&idx1_)) {
      Ok(i1_) => {
        match self.index_2.binary_search_by(|v| f64_into_hash_ord_fn(v).cmp(&idx2_)) {
          Ok(i_1) => self.get_lvf_moments_value(i1_, i_1),
          Err(pos2) => {
            let (i_1, i_2) = Self::find_pos(self.index_2.len(), pos2)?;
            let q_1 = self.get_lvf_moments_value(i1_, i_1)?;
            let q_2 = self.get_lvf_moments_value(i1_, i_2)?;
            let x_1 = self.index_2[i_1];
            let x_2 = self.index_2[i_2];
            // q_1 + (q_2 - q_1) * ((idx2 - x_1) / (x_2 - x_1))
            Some((q_2 - q_1).mul_add((idx2 - x_1) / (x_2 - x_1), q_1))
          }
        }
      }
      Err(pos1) => {
        let (i1_, i2_) = Self::find_pos(self.index_1.len(), pos1)?;
        let x1_ = self.index_1[i1_];
        let x2_ = self.index_1[i2_];
        match self.index_2.binary_search_by(|v| f64_into_hash_ord_fn(v).cmp(&idx2_)) {
          Ok(i_1) => {
            let q1_ = self.get_lvf_moments_value(i1_, i_1)?;
            let q2_ = self.get_lvf_moments_value(i2_, i_1)?;
            // Some(q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_)))
            Some((q2_ - q1_).mul_add((idx1 - x1_) / (x2_ - x1_), q1_))
          }
          Err(pos2) => {
            let (i_1, i_2) = Self::find_pos(self.index_2.len(), pos2)?;
            let q11 = self.get_lvf_moments_value(i1_, i_1)?;
            let q12 = self.get_lvf_moments_value(i1_, i_2)?;
            let q21 = self.get_lvf_moments_value(i2_, i_1)?;
            let q22 = self.get_lvf_moments_value(i2_, i_2)?;
            let x_1 = self.index_2[i_1];
            let x_2 = self.index_2[i_2];
            // let q1_ = q11 + (q12 - q11) * ((idx2 - x_1) / (x_2 - x_1));
            let q1_ = (q12 - q11).mul_add((idx2 - x_1) / (x_2 - x_1), q11);
            // let q2_ = q21 + (q22 - q21) * ((idx2 - x_1) / (x_2 - x_1));
            let q2_ = (q22 - q21).mul_add((idx2 - x_1) / (x_2 - x_1), q21);
            // q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_))
            Some((q2_ - q1_).mul_add((idx1 - x1_) / (x2_ - x1_), q1_))
          }
        }
      }
    }
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Default, Clone, Copy)]
pub struct LVFMoments {
  /// `mean` = `nominal` + `mean_shift`
  pub mean: f64,
  pub std_dev: f64,
  pub skewness: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Default, Clone, Copy)]
pub struct LVFEarlyLate {
  pub early_sigma: f64,
  pub late_sigma: f64,
}
#[expect(clippy::float_arithmetic)]
impl LVFMoments {
  /// Cornish–Fisher:
  /// `quantile ≈ μ + σ[ z + γ*​​(z^2−1)/6 − γ^2*​​(2*z^3−5*z)/36 ]`
  const Z: f64 = 3.0;
  const TMP1: f64 = (Self::Z * Self::Z - 1.0) / 6.0;
  const TMP2: f64 = (2.0 * Self::Z * Self::Z * Self::Z - 5.0 * Self::Z) / 36.0;
  #[inline]
  #[must_use]
  #[deprecated = "TODO"]
  /// `q+ ​​= μ + σ[ z + γ*TMP1 − γ^2*TMP2 ] = z*late_sigma - nominal`
  ///
  /// `​q− = μ + σ[−z + γ*TMP1 + γ^2*TMP2 ] = nominal - z*early_sigma`
  pub const fn to_early_late(&self, nominal: f64) -> Option<LVFEarlyLate> {
    if !(self.std_dev >= 0.0
      && nominal.is_finite()
      && self.mean.is_finite()
      && self.std_dev.is_finite()
      && self.skewness.is_finite())
    {
      return None;
    }
    let quantile_plus_3sigma = self.mean
      + self.std_dev
        * (self.skewness * Self::TMP1 + Self::Z
          - self.skewness * self.skewness * Self::TMP2);
    let quantile_minus_3sigma = self.mean
      + self.std_dev
        * (self.skewness * Self::TMP1 - Self::Z
          + self.skewness * self.skewness * Self::TMP2);
    Some(LVFEarlyLate {
      early_sigma: (nominal - quantile_minus_3sigma) / Self::Z,
      late_sigma: (quantile_plus_3sigma - nominal) / Self::Z,
    })
  }
}

#[expect(clippy::float_arithmetic)]
impl LVFEarlyLate {
  /// `q+ ​​= μ + σ[ z + γ*TMP1 − γ^2*TMP2 ] = z*late_sigma - nominal`
  ///
  /// `​q− = μ + σ[−z + γ*TMP1 + γ^2*TMP2 ] = nominal - z*early_sigma`
  ///
  /// `Δ = q+ - q- = z*(early_sigma+late_sigma) - 2*nominal`
  ///
  /// `a = (q+ + q-)/2 = z*(late_sigma-early_sigma)/2`
  #[inline]
  #[must_use]
  #[expect(clippy::suboptimal_flops)]
  #[deprecated = "TODO"]
  pub fn to_moments(&self, nominal: f64, mean: f64) -> Option<LVFMoments> {
    if !(self.early_sigma >= 0.0
      && self.late_sigma >= 0.0
      && nominal.is_finite()
      && mean.is_finite()
      && self.early_sigma.is_finite()
      && self.late_sigma.is_finite())
    {
      return None;
    }

    let delta = LVFMoments::Z * (self.early_sigma + self.late_sigma) - 2.0 * nominal;
    let a = LVFMoments::Z * (self.late_sigma - self.early_sigma) / 2.0;
    let a_minus_mean = a - mean;

    // K = 2*TMP2*(a-mean)^2 / TMP1^2
    let k = 2.0 * LVFMoments::TMP2 * a_minus_mean * a_minus_mean
      / (LVFMoments::TMP1 * LVFMoments::TMP1);

    // disc = delta^2 + 8*Z*k
    let disc = delta * delta + 8.0 * LVFMoments::Z * k;
    if !disc.is_finite() {
      return None;
    }
    let std_dev = (delta + disc.sqrt()) / (4.0 * LVFMoments::Z);
    if !(std_dev.is_finite() && std_dev >= 0.0) {
      return None;
    } // skewness=0 when std_dev==0
    let skewness =
      if std_dev > 0.0 { a_minus_mean / (std_dev * LVFMoments::TMP1) } else { 0.0 };

    Some(LVFMoments { mean, std_dev, skewness })
  }
}

impl LVFMoments {
  /// PeBay/West one-pass estimations
  /// <https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Online>
  #[inline]
  #[expect(
    clippy::as_conversions,
    clippy::float_arithmetic,
    clippy::cast_precision_loss,
    clippy::arithmetic_side_effects
  )]
  pub fn estimate<E, I: Iterator<Item = Result<f64, E>>>(
    data: I,
  ) -> Result<Option<Self>, E> {
    let mut n: usize = 0;
    let mut mean = 0.0;
    let mut m2 = 0.0; // sum (x - mean)^2
    let mut m3 = 0.0; // sum (x - mean)^3

    for x in data {
      n += 1;
      let n_f = n as f64;

      // PeBay/West one-pass
      let delta = x? - mean;
      let delta_n = delta / n_f;
      let term1 = delta * delta_n * (n_f - 1.0);

      m3 += (term1 * delta_n).mul_add(n_f - 2.0, -(3.0 * delta_n * m2));
      // m3 += term1 * delta_n * (n_f - 2.0) - 3.0 * delta_n * m2;
      m2 += term1;
      mean += delta_n;
    }

    if n == 0 {
      return Ok(None);
    }

    let n_f = n as f64;
    let std_dev = (m2 / n_f).sqrt();
    let skewness = if n >= 2 && std_dev > 0.0 {
      m3 / ((n_f - 1.0) * std_dev * std_dev * std_dev)
    } else {
      0.0
    };

    Ok(Some(Self { mean, std_dev, skewness }))
  }
  #[inline]
  #[must_use]
  /// self * a + b
  pub fn mul_add(self, a: f64, b: Self) -> Self {
    Self {
      mean: self.mean.mul_add(a, b.mean),
      std_dev: self.std_dev.mul_add(a, b.std_dev),
      skewness: self.skewness.mul_add(a, b.skewness),
    }
  }
}
impl PartialEq for LVFMoments {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    f64_into_hash_ord_fn(&self.mean) == f64_into_hash_ord_fn(&other.mean)
      && f64_into_hash_ord_fn(&self.std_dev) == f64_into_hash_ord_fn(&other.std_dev)
      && f64_into_hash_ord_fn(&self.skewness) == f64_into_hash_ord_fn(&other.skewness)
  }
}
#[expect(clippy::float_arithmetic)]
impl Add for LVFMoments {
  type Output = Self;
  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self {
      mean: self.mean + rhs.mean,
      std_dev: self.std_dev + rhs.std_dev,
      skewness: self.skewness + rhs.skewness,
    }
  }
}
#[expect(clippy::float_arithmetic)]
impl AddAssign for LVFMoments {
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
    self.mean += rhs.mean;
    self.std_dev += rhs.std_dev;
    self.skewness += rhs.skewness;
  }
}
#[expect(clippy::float_arithmetic)]
impl AddAssign for LVFEarlyLate {
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
    self.early_sigma += rhs.early_sigma;
    self.late_sigma += rhs.late_sigma;
  }
}
#[expect(clippy::float_arithmetic)]
impl Sub for LVFMoments {
  type Output = Self;
  #[inline]
  /// TODO: check
  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      mean: self.mean - rhs.mean,
      // TODO: check - or + ?
      std_dev: self.std_dev - rhs.std_dev,
      skewness: self.skewness - rhs.skewness,
    }
  }
}
#[expect(clippy::float_arithmetic)]
impl Mul<f64> for LVFMoments {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: f64) -> Self::Output {
    Self {
      mean: self.mean * rhs,
      std_dev: self.std_dev * rhs,
      skewness: self.skewness * rhs,
    }
  }
}
#[expect(clippy::float_arithmetic)]
impl MulAssign<f64> for LVFMoments {
  #[inline]
  fn mul_assign(&mut self, rhs: f64) {
    self.mean *= rhs;
    self.std_dev *= rhs;
    self.skewness *= rhs;
  }
}
#[expect(clippy::float_arithmetic)]
impl Div<f64> for LVFMoments {
  type Output = Self;
  #[inline]
  fn div(self, rhs: f64) -> Self::Output {
    Self {
      mean: self.mean / rhs,
      std_dev: self.std_dev / rhs,
      skewness: self.skewness / rhs,
    }
  }
}
#[expect(clippy::float_arithmetic)]
impl DivAssign<f64> for LVFMoments {
  #[inline]
  fn div_assign(&mut self, rhs: f64) {
    self.mean /= rhs;
    self.std_dev /= rhs;
    self.skewness /= rhs;
  }
}
#[expect(clippy::float_arithmetic)]
impl DivAssign<f64> for LVFEarlyLate {
  #[inline]
  fn div_assign(&mut self, rhs: f64) {
    self.early_sigma /= rhs;
    self.late_sigma /= rhs;
  }
}

impl<C: 'static + Ctx> ParsingBuilder<C> for Option<TimingTableLookUp<C>> {
  /// `value`, `mean_shift`, `std_dev`, `skewness`, `early/late-sigma`
  type Builder = (
    // value
    Option<<TableLookUp2D<C> as ParsingBuilder<C>>::Builder>,
    // mean_shift
    Option<<TableLookUp2D<C> as ParsingBuilder<C>>::Builder>,
    // std_dev
    Option<<TableLookUp2D<C> as ParsingBuilder<C>>::Builder>,
    // skewness
    Option<<TableLookUp2D<C> as ParsingBuilder<C>>::Builder>,
    // early/late sigma
    Vec<<OcvSigmaTable<C> as ParsingBuilder<C>>::Builder>,
  );
  #[inline]
  #[expect(clippy::float_arithmetic, clippy::too_many_lines)]
  fn build(builder: Self::Builder, _scope: &mut BuilderScope<C>) -> Self {
    #[inline]
    fn eq_index<C: 'static + Ctx>(
      lhs: &<TableLookUp2D<C> as ParsingBuilder<C>>::Builder,
      rhs: &<TableLookUp2D<C> as ParsingBuilder<C>>::Builder,
    ) -> bool {
      lhs.index_1 == rhs.index_1
        && lhs.index_2 == rhs.index_2
        && lhs.values.inner.len() == rhs.values.inner.len()
    }
    #[inline]
    fn ocv_eq_index<C: 'static + Ctx>(
      lhs: &<TableLookUp2D<C> as ParsingBuilder<C>>::Builder,
      rhs: &<OcvSigmaTable<C> as ParsingBuilder<C>>::Builder,
    ) -> bool {
      lhs.index_1 == rhs.index_1
        && lhs.index_2 == rhs.index_2
        && lhs.values.inner.len() == rhs.values.inner.len()
    }
    fn obtain_ocv_sigma<C: 'static + Ctx>(
      value: &TableLookUp2DBuilder<C>,
      comments: &mut String,
      ocv_sigma: Vec<OcvSigmaTableBuilder<C>>,
    ) -> Vec<LVFEarlyLate> {
      let mut early = None;
      let mut late = None;
      let mut early_late = None;
      for table in ocv_sigma {
        match table.sigma_type {
          SigmaType::Early => early = Some(table),
          SigmaType::Late => late = Some(table),
          SigmaType::EarlyAndLate => early_late = Some(table),
        }
      }
      if let (Some(early_table), Some(late_table)) = (early, late) {
        if ocv_eq_index(value, &early_table) && ocv_eq_index(value, &late_table) {
          zip(early_table.values.inner, late_table.values.inner)
            .map(|(early_sigma, late_sigma)| LVFEarlyLate { early_sigma, late_sigma })
            .collect()
        } else {
          crate::error!("LVF early_late LUTs' index mismatch");
          comments.push_str("LVF early_late LUTs' index mismatch");
          Vec::new()
        }
      } else if let Some(early_late_table) = early_late {
        if ocv_eq_index(value, &early_late_table) {
          early_late_table
            .values
            .inner
            .into_iter()
            .map(|sigma| LVFEarlyLate { early_sigma: sigma, late_sigma: sigma })
            .collect()
        } else {
          crate::error!("LVF early_late LUTs' index mismatch");
          comments.push_str("LVF early_late LUTs' index mismatch");
          Vec::new()
        }
      } else {
        Vec::new()
      }
    }
    let mut comments = String::new();
    match builder {
      (Some(_value), Some(_mean_shift), Some(_std_dev), Some(_skewness), ocv_sigma) => {
        let lvf_moments_values = if eq_index(&_value, &_mean_shift)
          && eq_index(&_mean_shift, &_std_dev)
          && eq_index(&_std_dev, &_skewness)
        {
          izip!(
            _value.values.inner.iter(),
            _mean_shift.values.inner,
            _std_dev.values.inner,
            _skewness.values.inner
          )
          .map(|(value, mean_shift, std_dev, skewness)| {
            let mean = value + mean_shift;
            LVFMoments { mean, std_dev, skewness }
          })
          .collect()
        } else {
          crate::error!("LVF moments LUTs' index mismatch");
          comments.push_str("LVF moments LUTs' index mismatch");
          Vec::new()
        };
        let lvf_early_late_values = obtain_ocv_sigma(&_value, &mut comments, ocv_sigma);
        Some(TimingTableLookUp {
          extra_ctx: C::Table::default(),
          name: _value.name,
          comments,
          index_1: _value.index_1,
          index_2: _value.index_2,
          values: _value.values.inner,
          lvf_moments_values,
          lvf_early_late_values,
        })
      }
      (Some(_value), None, None, None, ocv_sigma) => {
        let lvf_early_late_values = obtain_ocv_sigma(&_value, &mut comments, ocv_sigma);
        Some(TimingTableLookUp {
          extra_ctx: C::Table::default(),
          name: _value.name,
          comments,
          index_1: _value.index_1,
          index_2: _value.index_2,
          values: _value.values.inner,
          lvf_moments_values: Vec::new(),
          lvf_early_late_values,
        })
      }
      _ => None,
    }
  }
}
impl<C: 'static + Ctx> ParsingBuilder<C> for TimingTableLookUp<C> {
  type Builder = ();
  fn build(_: Self::Builder, _: &mut BuilderScope<C>) -> Self {
    unreachable!()
  }
}
impl<C: 'static + Ctx> ast::GroupAttri<C> for TimingTableLookUp<C> {
  #[inline]
  #[expect(clippy::float_arithmetic)]
  fn fmt_liberty<T: core::fmt::Write, I: ast::Indentation, K: core::fmt::Display>(
    &self,
    key: K,
    f: &mut ast::CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    let chunk_size =
      if self.index_2.is_empty() { self.values.len() } else { self.index_2.len() };
    let len = self.values.len();
    fmt_comment_liberty(Some(&self.comments), f)?;
    DisplayTableLookUp {
      name: &self.name,
      index_1: &self.index_1,
      index_2: &self.index_2,
      sigma_type: None,
      values: DisplayValues {
        len,
        chunk_size,
        inner: self.values.iter().copied(),
      },
    }
    .fmt_self::<_, _, C, _, _>("", &key, f)?;
    if !self.lvf_moments_values.is_empty() {
      DisplayTableLookUp {
        name: &self.name,
        index_1: &self.index_1,
        index_2: &self.index_2,
        sigma_type: None,
        values: DisplayValues {
          len,
          chunk_size,
          inner: izip!(self.values.iter(), self.lvf_moments_values.iter())
            .map(|(value, lvf)| lvf.mean - value),
        },
      }
      .fmt_self::<_, _, C, _, _>("ocv_mean_shift_", &key, f)?;
      DisplayTableLookUp {
        name: &self.name,
        index_1: &self.index_1,
        index_2: &self.index_2,
        sigma_type: None,
        values: DisplayValues {
          len,
          chunk_size,
          inner: self.lvf_moments_values.iter().map(|lvf| lvf.std_dev),
        },
      }
      .fmt_self::<_, _, C, _, _>("ocv_std_dev_", &key, f)?;
      DisplayTableLookUp {
        name: &self.name,
        index_1: &self.index_1,
        index_2: &self.index_2,
        sigma_type: None,
        values: DisplayValues {
          len,
          chunk_size,
          inner: self.lvf_moments_values.iter().map(|lvf| lvf.skewness),
        },
      }
      .fmt_self::<_, _, C, _, _>("ocv_skewness_", &key, f)?;
    }
    if !self.lvf_early_late_values.is_empty() {
      DisplayTableLookUp {
        name: &self.name,
        index_1: &self.index_1,
        index_2: &self.index_2,
        sigma_type: Some(SigmaType::Early),
        values: DisplayValues {
          len,
          chunk_size,
          inner: self.lvf_early_late_values.iter().map(|lvf| lvf.early_sigma),
        },
      }
      .fmt_self::<_, _, C, _, _>("ocv_sigma_", &key, f)?;
      DisplayTableLookUp {
        name: &self.name,
        index_1: &self.index_1,
        index_2: &self.index_2,
        sigma_type: Some(SigmaType::Late),
        values: DisplayValues {
          len,
          chunk_size,
          inner: self.lvf_early_late_values.iter().map(|lvf| lvf.late_sigma),
        },
      }
      .fmt_self::<_, _, C, _, _>("ocv_sigma_", &key, f)?;
    }
    Ok(())
  }
  fn nom_parse<'a, const IS_INCLUDED: bool>(
    _: &mut Self::Builder,
    _: &'a str,
    _: &str,
    _: &mut ast::ParseScope<'_>,
  ) -> nom::IResult<&'a str, Result<(), ast::IdError>, nom::error::Error<&'a str>> {
    unreachable!()
  }
}

impl<C: 'static + Ctx> Group<C> for TimingTableLookUp<C> {}

#[cfg(test)]
mod test {
  use super::{LVFEarlyLate, LVFMoments};

  #[test]
  #[expect(deprecated)]
  fn lvf_convert() {
    let nominal = 0.5065;
    let mean_shift = 0.005352;
    let std_dev = 0.04952;
    let skewness = 0.03483;
    let early_sigma = 0.0422;
    let late_sigma = 0.0624;
    let mean = nominal + mean_shift;
    let moments = LVFMoments { mean, std_dev, skewness };
    let early_late = LVFEarlyLate { early_sigma, late_sigma };
    moments.to_early_late(nominal);
    moments.to_early_late(nominal).unwrap().to_moments(nominal, mean);
    early_late.to_moments(nominal, mean);
  }
}
