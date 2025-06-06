//! All item structure inside
//! `Timing`.
#![allow(clippy::multiple_inherent_impl)]
use crate::{
  Ctx,
  ast::{
    self, BuilderScope, GroupComments, GroupFn, ParsingBuilder, fmt_comment_liberty,
  },
  common::f64_into_hash_ord_fn,
  expression::logic,
  table::{DisplayTableLookUp, DisplayValues, TableLookUp2D},
};
use core::ops::{Add, Mul, Not as _, Sub};
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
pub struct CellDegradation<C: Ctx> {
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
impl<C: Ctx> GroupFn<C> for CellDegradation<C> {}

#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TimingTableLookUp<C: Ctx> {
  pub extra_ctx: C::Table,
  pub name: String,
  pub comments: String,
  pub index_1: Vec<f64>,
  pub index_2: Vec<f64>,
  pub size1: usize,
  pub size2: usize,
  pub values: Vec<f64>,
  /// when `!lvf_values.is_empty() && lvf_index_1.is_empty()`
  /// directly use `index_1`
  pub lvf_index_1: Vec<f64>,
  /// when `!lvf_values.is_empty() && lvf_index_2.is_empty()`
  /// directly use `index_1`
  pub lvf_index_2: Vec<f64>,
  pub lvf_values: Vec<LVFValue>,
}
#[expect(
  clippy::similar_names,
  clippy::indexing_slicing,
  clippy::arithmetic_side_effects
)]
impl<C: Ctx> TimingTableLookUp<C> {
  #[inline]
  #[expect(clippy::needless_pass_by_ref_mut)]
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
  fn get_value(&self, ix: usize, iy: usize) -> f64 {
    self.values[ix * self.index_2.len() + iy]
  }
  #[inline]
  fn get_lvf_value(&self, ix: usize, iy: usize) -> LVFValue {
    self.lvf_values[ix * self.index_2.len() + iy]
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
          Ok(i_1) => Some(self.get_value(i1_, i_1)),
          Err(pos2) => Self::find_pos(self.index_2.len(), pos2).map(|(i_1, i_2)| {
            let q_1 = self.get_value(i1_, i_1);
            let q_2 = self.get_value(i1_, i_2);
            let x_1 = self.index_2[i_1];
            let x_2 = self.index_2[i_2];
            // q_1 + (q_2 - q_1) * ((idx2 - x_1) / (x_2 - x_1))
            (q_2 - q_1).mul_add((idx2 - x_1) / (x_2 - x_1), q_1)
          }),
        }
      }
      Err(pos1) => Self::find_pos(self.index_1.len(), pos1).and_then(|(i1_, i2_)| {
        let x1_ = self.index_1[i1_];
        let x2_ = self.index_1[i2_];
        match self.index_2.binary_search_by(|v| f64_into_hash_ord_fn(v).cmp(&idx2_)) {
          Ok(i_1) => {
            let q1_ = self.get_value(i1_, i_1);
            let q2_ = self.get_value(i2_, i_1);
            // Some(q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_)))
            Some((q2_ - q1_).mul_add((idx1 - x1_) / (x2_ - x1_), q1_))
          }
          Err(pos2) => Self::find_pos(self.index_2.len(), pos2).map(|(i_1, i_2)| {
            let q11 = self.get_value(i1_, i_1);
            let q12 = self.get_value(i1_, i_2);
            let q21 = self.get_value(i2_, i_1);
            let q22 = self.get_value(i2_, i_2);
            let x_1 = self.index_2[i_1];
            let x_2 = self.index_2[i_2];
            // let q1_ = q11 + (q12 - q11) * ((idx2 - x_1) / (x_2 - x_1));
            let q1_ = (q12 - q11).mul_add((idx2 - x_1) / (x_2 - x_1), q11);
            // let q2_ = q21 + (q22 - q21) * ((idx2 - x_1) / (x_2 - x_1));
            let q2_ = (q22 - q21).mul_add((idx2 - x_1) / (x_2 - x_1), q21);
            // q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_))
            (q2_ - q1_).mul_add((idx1 - x1_) / (x2_ - x1_), q1_)
          }),
        }
      }),
    }
  }
  #[must_use]
  #[inline]
  #[expect(clippy::float_arithmetic)]
  pub fn lookup_lvf(&self, idx1: &f64, idx2: &f64) -> Option<LVFValue> {
    let idx1_ = f64_into_hash_ord_fn(idx1);
    let idx2_ = f64_into_hash_ord_fn(idx2);
    match self.index_1.binary_search_by(|v| f64_into_hash_ord_fn(v).cmp(&idx1_)) {
      Ok(i1_) => {
        match self.index_2.binary_search_by(|v| f64_into_hash_ord_fn(v).cmp(&idx2_)) {
          Ok(i_1) => Some(self.get_lvf_value(i1_, i_1)),
          Err(pos2) => Self::find_pos(self.index_2.len(), pos2).map(|(i_1, i_2)| {
            let q_1 = self.get_lvf_value(i1_, i_1);
            let q_2 = self.get_lvf_value(i1_, i_2);
            let x_1 = self.index_2[i_1];
            let x_2 = self.index_2[i_2];
            // q_1 + (q_2 - q_1) * ((idx2 - x_1) / (x_2 - x_1))
            (q_2 - q_1).mul_add((idx2 - x_1) / (x_2 - x_1), q_1)
          }),
        }
      }
      Err(pos1) => Self::find_pos(self.index_1.len(), pos1).and_then(|(i1_, i2_)| {
        let x1_ = self.index_1[i1_];
        let x2_ = self.index_1[i2_];
        match self.index_2.binary_search_by(|v| f64_into_hash_ord_fn(v).cmp(&idx2_)) {
          Ok(i_1) => {
            let q1_ = self.get_lvf_value(i1_, i_1);
            let q2_ = self.get_lvf_value(i2_, i_1);
            // Some(q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_)))
            Some((q2_ - q1_).mul_add((idx1 - x1_) / (x2_ - x1_), q1_))
          }
          Err(pos2) => Self::find_pos(self.index_2.len(), pos2).map(|(i_1, i_2)| {
            let q11 = self.get_lvf_value(i1_, i_1);
            let q12 = self.get_lvf_value(i1_, i_2);
            let q21 = self.get_lvf_value(i2_, i_1);
            let q22 = self.get_lvf_value(i2_, i_2);
            let x_1 = self.index_2[i_1];
            let x_2 = self.index_2[i_2];
            // let q1_ = q11 + (q12 - q11) * ((idx2 - x_1) / (x_2 - x_1));
            let q1_ = (q12 - q11).mul_add((idx2 - x_1) / (x_2 - x_1), q11);
            // let q2_ = q21 + (q22 - q21) * ((idx2 - x_1) / (x_2 - x_1));
            let q2_ = (q22 - q21).mul_add((idx2 - x_1) / (x_2 - x_1), q21);
            // q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_))
            (q2_ - q1_).mul_add((idx1 - x1_) / (x2_ - x1_), q1_)
          }),
        }
      }),
    }
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Default, Clone, Copy)]
pub struct LVFValue {
  /// `mean` = `nominal` + `mean_shift`
  pub mean: f64,
  pub std_dev: f64,
  pub skewness: f64,
}
impl LVFValue {
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
impl PartialEq for LVFValue {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    f64_into_hash_ord_fn(&self.mean) == f64_into_hash_ord_fn(&other.mean)
      && f64_into_hash_ord_fn(&self.std_dev) == f64_into_hash_ord_fn(&other.std_dev)
      && f64_into_hash_ord_fn(&self.skewness) == f64_into_hash_ord_fn(&other.skewness)
  }
}
#[expect(clippy::float_arithmetic)]
impl Add for LVFValue {
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
impl Sub for LVFValue {
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
impl Mul<f64> for LVFValue {
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

impl<C: Ctx> ParsingBuilder<C> for Option<TimingTableLookUp<C>> {
  /// `value`, `mean_shift`, `std_dev`, `skewness`
  type Builder = (
    // value
    Option<<TableLookUp2D<C> as ParsingBuilder<C>>::Builder>,
    // mean_shift
    Option<<TableLookUp2D<C> as ParsingBuilder<C>>::Builder>,
    // std_dev
    Option<<TableLookUp2D<C> as ParsingBuilder<C>>::Builder>,
    // skewness
    Option<<TableLookUp2D<C> as ParsingBuilder<C>>::Builder>,
  );
  #[inline]
  #[expect(clippy::float_arithmetic, clippy::arithmetic_side_effects)]
  fn build(builder: Self::Builder, _scope: &mut BuilderScope<C>) -> Self {
    #[inline]
    fn eq_index<C: Ctx>(
      lhs: &<TableLookUp2D<C> as ParsingBuilder<C>>::Builder,
      rhs: &<TableLookUp2D<C> as ParsingBuilder<C>>::Builder,
    ) -> bool {
      lhs.index_1 == rhs.index_1 && lhs.index_2 == rhs.index_2
    }
    let mut out: TimingTableLookUp<C> = match builder {
      (Some(_value), Some(_mean_shift), Some(_std_dev), Some(_skewness)) => {
        let lvf_nomial_same_index = eq_index(&_value, &_mean_shift);
        let valid_lvf_index =
          eq_index(&_mean_shift, &_std_dev) && eq_index(&_std_dev, &_skewness);
        let (lvf_values, comments) = if valid_lvf_index {
          (
            izip!(
              _value.values.inner.iter(),
              _mean_shift.values.inner,
              _std_dev.values.inner,
              _skewness.values.inner
            )
            .map(|(value, mean_shift, std_dev, skewness)| {
              let mean = value + mean_shift;
              LVFValue { mean, std_dev, skewness }
            })
            .collect(),
            String::new(),
          )
        } else {
          crate::error!("LVF LUTs' index mismatch");
          (Vec::new(), String::from("LVF LUTs' index mismatch"))
        };
        TimingTableLookUp {
          extra_ctx: C::Table::default(),
          name: _value.name,
          comments,
          index_1: _value.index_1,
          index_2: _value.index_2,
          size1: _value.values.size1,
          size2: _value.values.size2,
          values: _value.values.inner,
          lvf_index_1: if lvf_nomial_same_index {
            Vec::new()
          } else {
            _mean_shift.index_1
          },
          lvf_index_2: if lvf_nomial_same_index {
            Vec::new()
          } else {
            _mean_shift.index_2
          },
          lvf_values,
        }
      }
      (Some(_value), None, None, None) => TimingTableLookUp {
        extra_ctx: C::Table::default(),
        name: _value.name,
        comments: String::new(),
        index_1: _value.index_1,
        index_2: _value.index_2,
        size1: _value.values.size1,
        size2: _value.values.size2,
        values: _value.values.inner,
        lvf_index_1: Vec::new(),
        lvf_index_2: Vec::new(),
        lvf_values: Vec::new(),
      },
      _ => return None,
    };
    if out.size2 == 1 && out.values.len() == out.index_1.len() * out.index_2.len() {
      out.size1 = out.index_1.len();
      out.size2 = out.index_2.len();
    }
    Some(out)
  }
}
impl<C: Ctx> TimingTableLookUp<C> {
  #[inline]
  #[expect(clippy::float_arithmetic)]
  pub(crate) fn fmt_liberty<T: core::fmt::Write, I: ast::Indentation>(
    &self,
    key: &str,
    f: &mut ast::CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    fmt_comment_liberty(Some(&self.comments), f)?;
    DisplayTableLookUp {
      name: &self.name,
      index_1: &self.index_1,
      index_2: &self.index_2,
      values: DisplayValues {
        len: self.values.len(),
        size1: self.size1,
        inner: self.values.iter().copied(),
      },
    }
    .fmt_self::<_, _, C>("", key, f)?;
    if !self.lvf_values.is_empty() {
      let mismatch_index = !self.lvf_index_1.is_empty();
      DisplayTableLookUp {
        name: &self.name,
        index_1: if mismatch_index { &self.lvf_index_1 } else { &self.index_1 },
        index_2: if mismatch_index { &self.lvf_index_2 } else { &self.index_2 },
        values: DisplayValues {
          len: self.values.len(),
          size1: self.size1,
          inner: izip!(self.values.iter(), self.lvf_values.iter())
            .map(|(value, lvf)| lvf.mean - value),
        },
      }
      .fmt_self::<_, _, C>("ocv_mean_shift_", key, f)?;
      DisplayTableLookUp {
        name: &self.name,
        index_1: if mismatch_index { &self.lvf_index_1 } else { &self.index_1 },
        index_2: if mismatch_index { &self.lvf_index_2 } else { &self.index_2 },
        values: DisplayValues {
          len: self.values.len(),
          size1: self.size1,
          inner: self.lvf_values.iter().map(|lvf| lvf.std_dev),
        },
      }
      .fmt_self::<_, _, C>("ocv_std_dev_", key, f)?;
      DisplayTableLookUp {
        name: &self.name,
        index_1: if mismatch_index { &self.lvf_index_1 } else { &self.index_1 },
        index_2: if mismatch_index { &self.lvf_index_2 } else { &self.index_2 },
        values: DisplayValues {
          len: self.values.len(),
          size1: self.size1,
          inner: self.lvf_values.iter().map(|lvf| lvf.skewness),
        },
      }
      .fmt_self::<_, _, C>("ocv_skewness_", key, f)?;
    }
    Ok(())
  }
}
