//! All item structure inside
//! `Timing`.
#![allow(clippy::multiple_inherent_impl)]
use core::ops::{Add, Mul, Not as _, Sub};

use crate::{
  ast::{
    self, fmt_comment_liberty, BuilderScope, GroupComments, GroupFn, ParseScope,
    ParsingBuilder, SimpleAttri,
  },
  common::table::{DisplayTableLookUp, DisplayValues, TableLookUp},
  expression::logic,
  ArcStr, Ctx, NotNan,
};

use itertools::izip;
use strum_macros::{Display, EnumString};
/// The `timing_sense` attribute describes the way an input pin logically affects an output pin.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =213.11
/// &end
/// =214.6
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =203.55
/// &end
/// =203.55
/// ">Reference-Instance</a>
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
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =213.11
/// &end
/// =214.6
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =203.55
/// &end
/// =203.55
/// ">Reference-Instance</a>
#[derive(
  Debug, Clone, Copy, PartialEq, Display, EnumString, Default, Hash, Eq, PartialOrd, Ord
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TimingSenseType {
  /// Combines incoming `rise` delays with local `rise` delays
  /// and compares incoming `fall` delays with local `fall` delays.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
  /// ?field=test
  /// &bgn
  /// =t.m0.x45.ha.y2b10.ffc.fs2.fc2.sc0.ls0.ws0
  /// &end
  /// =t.m0.x37.h4.y2b12.ff1.fs2.fc2.sc0.ls0.ws0
  /// ">Reference</a>
  #[strum(serialize = "positive_unate")]
  PositiveUnate,
  /// Combines incoming `rise` delays with local `fall` delays
  /// and compares incoming `fall` delays with local `rise` delays.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
  /// ?field=test
  /// &bgn
  /// =t.m0.x45.ha.y2b13.ffc.fs2.fc2.sc0.ls0.ws0
  /// &end
  /// =t.m0.x37.h4.y2b15.ff1.fs2.fc2.sc0.ls0.ws0
  /// ">Reference</a>
  #[strum(serialize = "negative_unate")]
  NegativeUnate,
  /// Combines local delays with the `worst-case` incoming delay value.
  /// The non-unate timing sense represents a function whose
  /// output value change cannot be determined from the direction
  /// of the change in the input value.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
  /// ?field=test
  /// &bgn
  /// =t.m0.x45.ha.y2b16.ffc.fs2.fc2.sc0.ls0.ws0
  /// &end
  /// =t.m0.x37.h4.y2b19.ff1.fs2.fc2.sc0.ls0.ws0
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
impl SimpleAttri for TimingSenseType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::nom_parse_from_str(i, scope)
  }
}
/// You define the mode attribute within a timing group.
///
/// A mode attribute pertains to an individual timing arc.
/// The timing arc is active when mode is instantiated with a name and a value.
/// You can specify multiple instances of the mode attribute,
/// but only one instance for each timing arc.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =219.39
/// +220.11
/// &end
/// =220.9
/// +222.73
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =204.5
/// &end
/// =204.5
/// ">Reference-Instance</a>
///
// #[derive(Debug, Clone, Copy, Default)]
pub type Mode = [ArcStr; 2];

/// The `cell_degradation`  group describes a cell performance degradation
/// design rule for compiling a design.
///
/// A cell degradation design rule specifies the maximum capacitive load
/// a cell can drive without causing cell performance degradation during the fall transition.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=347.33&end=347.35
/// ">Reference</a>
///
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct CellDegradation<C: Ctx> {
  /// name
  #[size = 8]
  #[liberty(name)]
  #[id(borrow = "&str", with_ref = false)]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  #[size = 0]
  #[liberty(extra_ctx)]
  extra_ctx: C::Other,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: ast::Attributes,
  /// /* lookup table */
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=348.6&end=348.7
  /// ">Reference</a>
  #[size = 24]
  #[liberty(complex)]
  pub index_1: Vec<NotNan<f64>>,
  /// /* lookup table */
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=348.6&end=348.7
  /// ">Reference</a>
  #[size = 24]
  #[liberty(complex)]
  pub values: Vec<NotNan<f64>>,
}
impl<C: Ctx> GroupFn for CellDegradation<C> {}

#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TimingTableLookUp<C: Ctx> {
  pub extra_ctx: C::Table,
  pub name: Option<ArcStr>,
  pub comments: String,
  pub index_1: Vec<NotNan<f64>>,
  pub index_2: Vec<NotNan<f64>>,
  pub size1: usize,
  pub size2: usize,
  pub values: Vec<NotNan<f64>>,
  pub lvf_values: Vec<LVFValue>,
}
#[expect(
  clippy::similar_names,
  clippy::indexing_slicing,
  clippy::arithmetic_side_effects
)]
impl<C: Ctx> TimingTableLookUp<C> {
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
  fn get_value(&self, ix: usize, iy: usize) -> NotNan<f64> {
    self.values[ix * self.index_2.len() + iy]
  }
  #[inline]
  fn get_lvf_value(&self, ix: usize, iy: usize) -> LVFValue {
    self.lvf_values[ix * self.index_2.len() + iy]
  }
  #[must_use]
  #[inline]
  pub fn lookup(&self, idx1: &NotNan<f64>, idx2: &NotNan<f64>) -> Option<NotNan<f64>> {
    match self.index_1.binary_search(idx1) {
      Ok(i1_) => match self.index_2.binary_search(idx2) {
        Ok(i_1) => Some(self.get_value(i1_, i_1)),
        Err(pos2) => Self::find_pos(self.index_2.len(), pos2).map(|(i_1, i_2)| {
          let q_1 = self.get_value(i1_, i_1);
          let q_2 = self.get_value(i1_, i_2);
          let x_1 = self.index_2[i_1];
          let x_2 = self.index_2[i_2];
          q_1 + (q_2 - q_1) * ((idx2 - x_1) / (x_2 - x_1))
        }),
      },
      Err(pos1) => Self::find_pos(self.index_1.len(), pos1).and_then(|(i1_, i2_)| {
        let x1_ = self.index_1[i1_];
        let x2_ = self.index_1[i2_];
        match self.index_2.binary_search(idx2) {
          Ok(i_1) => {
            let q1_ = self.get_value(i1_, i_1);
            let q2_ = self.get_value(i2_, i_1);
            Some(q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_)))
          }
          Err(pos2) => Self::find_pos(self.index_2.len(), pos2).map(|(i_1, i_2)| {
            let q11 = self.get_value(i1_, i_1);
            let q12 = self.get_value(i1_, i_2);
            let q21 = self.get_value(i2_, i_1);
            let q22 = self.get_value(i2_, i_2);
            let x_1 = self.index_2[i_1];
            let x_2 = self.index_2[i_2];
            let q1_ = q11 + (q12 - q11) * ((idx2 - x_1) / (x_2 - x_1));
            let q2_ = q21 + (q22 - q21) * ((idx2 - x_1) / (x_2 - x_1));
            q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_))
          }),
        }
      }),
    }
  }
  #[must_use]
  #[inline]
  pub fn lookup_lvf(&self, idx1: &NotNan<f64>, idx2: &NotNan<f64>) -> Option<LVFValue> {
    match self.index_1.binary_search(idx1) {
      Ok(i1_) => match self.index_2.binary_search(idx2) {
        Ok(i_1) => Some(self.get_lvf_value(i1_, i_1)),
        Err(pos2) => Self::find_pos(self.index_2.len(), pos2).map(|(i_1, i_2)| {
          let q_1 = self.get_lvf_value(i1_, i_1);
          let q_2 = self.get_lvf_value(i1_, i_2);
          let x_1 = self.index_2[i_1];
          let x_2 = self.index_2[i_2];
          q_1 + (q_2 - q_1) * ((idx2 - x_1) / (x_2 - x_1))
        }),
      },
      Err(pos1) => Self::find_pos(self.index_1.len(), pos1).and_then(|(i1_, i2_)| {
        let x1_ = self.index_1[i1_];
        let x2_ = self.index_1[i2_];
        match self.index_2.binary_search(idx2) {
          Ok(i_1) => {
            let q1_ = self.get_lvf_value(i1_, i_1);
            let q2_ = self.get_lvf_value(i2_, i_1);
            Some(q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_)))
          }
          Err(pos2) => Self::find_pos(self.index_2.len(), pos2).map(|(i_1, i_2)| {
            let q11 = self.get_lvf_value(i1_, i_1);
            let q12 = self.get_lvf_value(i1_, i_2);
            let q21 = self.get_lvf_value(i2_, i_1);
            let q22 = self.get_lvf_value(i2_, i_2);
            let x_1 = self.index_2[i_1];
            let x_2 = self.index_2[i_2];
            let q1_ = q11 + (q12 - q11) * ((idx2 - x_1) / (x_2 - x_1));
            let q2_ = q21 + (q22 - q21) * ((idx2 - x_1) / (x_2 - x_1));
            q1_ + (q2_ - q1_) * ((idx1 - x1_) / (x2_ - x1_))
          }),
        }
      }),
    }
  }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LVFValue {
  /// `mean` = `nominal` + `mean_shift`
  pub mean: NotNan<f64>,
  pub std_dev: NotNan<f64>,
  pub skewness: NotNan<f64>,
}
#[expect(clippy::arithmetic_side_effects)]
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
#[expect(clippy::arithmetic_side_effects)]
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
#[expect(clippy::arithmetic_side_effects)]
impl Mul<NotNan<f64>> for LVFValue {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: NotNan<f64>) -> Self::Output {
    Self {
      mean: self.mean * rhs,
      std_dev: self.std_dev * rhs,
      skewness: self.skewness * rhs,
    }
  }
}

impl<C: Ctx> ParsingBuilder for Option<TimingTableLookUp<C>> {
  /// `value`, `mean_shift`, `std_dev`, `skewness`
  type Builder = (
    // value
    Option<<TableLookUp<C> as ParsingBuilder>::Builder>,
    // mean_shift
    Option<<TableLookUp<C> as ParsingBuilder>::Builder>,
    // std_dev
    Option<<TableLookUp<C> as ParsingBuilder>::Builder>,
    // skewness
    Option<<TableLookUp<C> as ParsingBuilder>::Builder>,
  );
  #[inline]
  #[expect(clippy::arithmetic_side_effects)]
  fn build(builder: Self::Builder, _scope: &mut BuilderScope) -> Self {
    #[inline]
    fn eq_index<C: Ctx>(
      lhs: &<TableLookUp<C> as ParsingBuilder>::Builder,
      rhs: &<TableLookUp<C> as ParsingBuilder>::Builder,
    ) -> bool {
      lhs.index_1 == rhs.index_1 && lhs.index_2 == rhs.index_2
    }
    match builder {
      (Some(_value), Some(_mean_shift), Some(_std_dev), Some(_skewness)) => {
        let (lvf_values, comments) = if eq_index(&_value, &_mean_shift)
          && eq_index(&_mean_shift, &_std_dev)
          && eq_index(&_std_dev, &_skewness)
        {
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
          log::error!("LVF LUTs' index mismatch");
          (Vec::new(), String::from("LVF LUTs' index mismatch"))
        };
        Some(TimingTableLookUp {
          name: _value.name,
          comments,
          index_1: _value.index_1,
          index_2: _value.index_2,
          size1: _value.values.size1,
          size2: _value.values.size2,
          values: _value.values.inner,
          lvf_values,
          extra_ctx: _value.extra_ctx,
        })
      }
      (Some(_value), None, None, None) => Some(TimingTableLookUp {
        name: _value.name,
        comments: String::new(),
        index_1: _value.index_1,
        index_2: _value.index_2,
        size1: _value.values.size1,
        size2: _value.values.size2,
        values: _value.values.inner,
        lvf_values: Vec::new(),
        extra_ctx: _value.extra_ctx,
      }),
      _ => None,
    }
  }
}
impl<C: Ctx> TimingTableLookUp<C> {
  #[inline]
  #[expect(clippy::arithmetic_side_effects)]
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
        size1: self.size1,
        inner: self.values.iter().copied(),
      },
    }
    .fmt_self("", key, f)?;
    if !self.lvf_values.is_empty() {
      DisplayTableLookUp {
        name: &self.name,
        index_1: &self.index_1,
        index_2: &self.index_2,
        values: DisplayValues {
          size1: self.size1,
          inner: izip!(self.values.iter(), self.lvf_values.iter())
            .map(|(value, lvf)| lvf.mean - value),
        },
      }
      .fmt_self("ocv_mean_shift_", key, f)?;
      DisplayTableLookUp {
        name: &self.name,
        index_1: &self.index_1,
        index_2: &self.index_2,
        values: DisplayValues {
          size1: self.size1,
          inner: self.lvf_values.iter().map(|lvf| lvf.std_dev),
        },
      }
      .fmt_self("ocv_std_dev_", key, f)?;
      DisplayTableLookUp {
        name: &self.name,
        index_1: &self.index_1,
        index_2: &self.index_2,
        values: DisplayValues {
          size1: self.size1,
          inner: self.lvf_values.iter().map(|lvf| lvf.skewness),
        },
      }
      .fmt_self("ocv_skewness_", key, f)?;
    }
    Ok(())
  }
}
