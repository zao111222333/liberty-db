//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

use super::{BooleanExpression, BooleanExpressionLike, UNKNOWN};
use crate::{
  ast::{Attributes, GroupComments, GroupFn, IdError, NamedGroup, ParseScope},
  ArcStr,
};
use biodivine_lib_bdd::boolean_expression::BooleanExpression as Expr;
use core::fmt::Write;
/// The `ff` group describes either a single-stage or a master-slave flip-flop
/// in a cell or test cell. The syntax for a cell is shown here.
/// TODO: For information about the `test_cell` group, see [test_cell](crate::test_cell) Group
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=157.22&end=157.40
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone, Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FF {
  /// The `variable1` (`variable[0]`) value is the state of the
  /// noninverting output of the flip-flop;
  /// the `variable2` (`variable[1]`) value is the state of the inverting output.
  /// The `variable1` value can be considered the 1-bit storage of the flip-flop.
  /// Valid values for `variable1`  and `variable2` are
  /// anything except a pin name used in the cell being described.
  /// Both of these variables must be assigned,
  /// even if one of them is not connected to a primary output pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=158.2&end=158.6
  /// ">Reference-Definition</a>
  #[liberty(name)]
  #[id]
  pub variable1: ArcStr,
  /// The `variable1` (`variable[0]`) value is the state of the
  /// noninverting output of the flip-flop;
  /// the `variable2` (`variable[1]`) value is the state of the inverting output.
  /// The `variable1` value can be considered the 1-bit storage of the flip-flop.
  /// Valid values for `variable1`  and `variable2` are
  /// anything except a pin name used in the cell being described.
  /// Both of these variables must be assigned,
  /// even if one of them is not connected to a primary output pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=158.2&end=158.6
  /// ">Reference-Definition</a>
  #[liberty(name)]
  #[id]
  pub variable2: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// The clear  attribute gives the active value for the clear input.
  #[liberty(simple(type = Option))]
  pub clear: Option<BooleanExpression>,
  /// The `clear_preset_var1` attribute gives the value that `variable1`
  ///  has when `clear` and `preset` are both active at the same time.
  #[liberty(simple(type = Option))]
  pub clear_preset_var1: Option<ClearPresetState>,
  /// The `clear_preset_var2` attribute gives the value that `variable2`
  ///  has when `clear` and `preset` are both active at the same time.
  #[liberty(simple(type = Option))]
  pub clear_preset_var2: Option<ClearPresetState>,
  /// The `clocked_on`  and `clocked_on_also`  attributes identify
  /// the active edge of the clock signals and are required in all `ff`  groups.
  /// For example, use `clocked_on : "CP"`  to describe a rising-edge-triggered device
  /// and use  `clocked_on_also : "CP"`  for a falling-edge-triggered device.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=159.24&end=159.27
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub clocked_on: Option<BooleanExpression>,
  /// The `clocked_on`  and `clocked_on_also`  attributes identify
  /// the active edge of the clock signals and are required in all `ff`  groups.
  /// For example, use `clocked_on : "CP"`  to describe a rising-edge-triggered device
  /// and use  `clocked_on_also : "CP"`  for a falling-edge-triggered device.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=159.24&end=159.27
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub clocked_on_also: Option<BooleanExpression>,
  /// The value of `variable1` after the active edge.
  #[liberty(simple(type = Option))]
  pub next_state: Option<BooleanExpression>,
  /// The `preset` attribute gives the active value for the preset input.
  #[liberty(simple(type = Option))]
  pub preset: Option<BooleanExpression>,
}
/// The `ff` group describes either a single-stage or a master-slave flip-flop
/// in a cell or test cell. The syntax for a cell is shown here.
/// TODO: For information about the `test_cell` group, see [test_cell](crate::test_cell) Group
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=157.22&end=157.40
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone, Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FFBank {
  /// The `variable1` (`variable[0]`) value is the state of the
  /// noninverting output of the flip-flop;
  /// the `variable2` (`variable[1]`) value is the state of the inverting output.
  /// The `variable1` value can be considered the 1-bit storage of the flip-flop.
  /// Valid values for `variable1`  and `variable2` are
  /// anything except a pin name used in the cell being described.
  /// Both of these variables must be assigned,
  /// even if one of them is not connected to a primary output pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=158.2&end=158.6
  /// ">Reference-Definition</a>
  #[liberty(name)]
  #[id]
  pub variable1: ArcStr,
  /// The `variable1` (`variable[0]`) value is the state of the
  /// noninverting output of the flip-flop;
  /// the `variable2` (`variable[1]`) value is the state of the inverting output.
  /// The `variable1` value can be considered the 1-bit storage of the flip-flop.
  /// Valid values for `variable1`  and `variable2` are
  /// anything except a pin name used in the cell being described.
  /// Both of these variables must be assigned,
  /// even if one of them is not connected to a primary output pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=158.2&end=158.6
  /// ">Reference-Definition</a>
  #[liberty(name)]
  #[id]
  pub variable2: ArcStr,
  /// bits
  #[liberty(name)]
  pub bits: usize,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// The clear  attribute gives the active value for the clear input.
  #[liberty(simple(type = Option))]
  pub clear: Option<BooleanExpression>,
  /// The `clear_preset_var1` attribute gives the value that `variable1`
  ///  has when `clear` and `preset` are both active at the same time.
  #[liberty(simple(type = Option))]
  pub clear_preset_var1: Option<ClearPresetState>,
  /// The `clear_preset_var2` attribute gives the value that `variable2`
  ///  has when `clear` and `preset` are both active at the same time.
  #[liberty(simple(type = Option))]
  pub clear_preset_var2: Option<ClearPresetState>,
  /// The `clocked_on`  and `clocked_on_also`  attributes identify
  /// the active edge of the clock signals and are required in all `ff`  groups.
  /// For example, use `clocked_on : "CP"`  to describe a rising-edge-triggered device
  /// and use  `clocked_on_also : "CP"`  for a falling-edge-triggered device.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=159.24&end=159.27
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub clocked_on: Option<BooleanExpression>,
  /// The `clocked_on`  and `clocked_on_also`  attributes identify
  /// the active edge of the clock signals and are required in all `ff`  groups.
  /// For example, use `clocked_on : "CP"`  to describe a rising-edge-triggered device
  /// and use  `clocked_on_also : "CP"`  for a falling-edge-triggered device.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=159.24&end=159.27
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub clocked_on_also: Option<BooleanExpression>,
  /// The value of `variable1` after the active edge.
  #[liberty(simple(type = Option))]
  pub next_state: Option<BooleanExpression>,
  /// The `preset` attribute gives the active value for the preset input.
  #[liberty(simple(type = Option))]
  pub preset: Option<BooleanExpression>,
}

/// A `latch` group is defined within a `cell`, `model`, or `test_cell` group to describe a levelsensitive memory device.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=183.3&end=183.5
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone, Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Latch {
  /// The `variable1` (`variable[0]`) value is the state of the
  /// noninverting output of the flip-flop;
  /// the `variable2` (`variable[1]`) value is the state of the inverting output.
  /// The `variable1` value can be considered the 1-bit storage of the flip-flop.
  /// Valid values for `variable1`  and `variable2` are
  /// anything except a pin name used in the cell being described.
  /// Both of these variables must be assigned,
  /// even if one of them is not connected to a primary output pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=158.2&end=158.6
  /// ">Reference-Definition</a>
  #[id]
  #[liberty(name)]
  pub variable1: ArcStr,
  /// The `variable1` (`variable[0]`) value is the state of the
  /// noninverting output of the flip-flop;
  /// the `variable2` (`variable[1]`) value is the state of the inverting output.
  /// The `variable1` value can be considered the 1-bit storage of the flip-flop.
  /// Valid values for `variable1`  and `variable2` are
  /// anything except a pin name used in the cell being described.
  /// Both of these variables must be assigned,
  /// even if one of them is not connected to a primary output pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=158.2&end=158.6
  /// ">Reference-Definition</a>
  #[id]
  #[liberty(name)]
  pub variable2: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// The clear  attribute gives the active value for the clear input.
  #[liberty(simple(type = Option))]
  pub clear: Option<BooleanExpression>,
  /// The `clear_preset_var1` attribute gives the value that `variable1`
  ///  has when `clear` and `preset` are both active at the same time.
  #[liberty(simple(type = Option))]
  pub clear_preset_var1: Option<ClearPresetState>,
  /// The `clear_preset_var2` attribute gives the value that `variable2`
  ///  has when `clear` and `preset` are both active at the same time.
  #[liberty(simple(type = Option))]
  pub clear_preset_var2: Option<ClearPresetState>,
  /// The `enable`  attribute gives the state of the enable input,
  /// and `data_in`  attribute gives the state of the data input.
  /// The `enable`  and `data_in`  attributes are optional,
  /// but if you use one of them, you must also use the other.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=185.11&end=185.13
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub enable: Option<BooleanExpression>,
  /// The `enable_also`  attribute gives the state of the `enable`
  /// input when you are describing master and slave cells.
  /// The `enable_also`  attribute is optional.
  /// If you use `enable_also`, you must also use the enable  and data_in  attributes
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=185.23&end=185.25
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub enable_also: Option<BooleanExpression>,
  /// The value of `variable1` after the active edge.
  #[liberty(simple(type = Option))]
  pub data_in: Option<BooleanExpression>,
  /// The `preset` attribute gives the active value for the preset input.
  #[liberty(simple(type = Option))]
  pub preset: Option<BooleanExpression>,
}

/// A `latch` group is defined within a `cell`, `model`, or `test_cell` group to describe a levelsensitive memory device.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=183.3&end=183.5
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone, Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LatchBank {
  /// The `variable1` (`variable[0]`) value is the state of the
  /// noninverting output of the flip-flop;
  /// the `variable2` (`variable[1]`) value is the state of the inverting output.
  /// The `variable1` value can be considered the 1-bit storage of the flip-flop.
  /// Valid values for `variable1`  and `variable2` are
  /// anything except a pin name used in the cell being described.
  /// Both of these variables must be assigned,
  /// even if one of them is not connected to a primary output pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=158.2&end=158.6
  /// ">Reference-Definition</a>
  #[id]
  #[liberty(name)]
  pub variable1: ArcStr,
  /// The `variable1` (`variable[0]`) value is the state of the
  /// noninverting output of the flip-flop;
  /// the `variable2` (`variable[1]`) value is the state of the inverting output.
  /// The `variable1` value can be considered the 1-bit storage of the flip-flop.
  /// Valid values for `variable1`  and `variable2` are
  /// anything except a pin name used in the cell being described.
  /// Both of these variables must be assigned,
  /// even if one of them is not connected to a primary output pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=158.2&end=158.6
  /// ">Reference-Definition</a>
  #[id]
  #[liberty(name)]
  pub variable2: ArcStr,
  /// bits
  #[liberty(name)]
  pub bits: usize,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// The clear  attribute gives the active value for the clear input.
  #[liberty(simple(type = Option))]
  pub clear: Option<BooleanExpression>,
  /// The `clear_preset_var1` attribute gives the value that `variable1`
  ///  has when `clear` and `preset` are both active at the same time.
  #[liberty(simple(type = Option))]
  pub clear_preset_var1: Option<ClearPresetState>,
  /// The `clear_preset_var2` attribute gives the value that `variable2`
  ///  has when `clear` and `preset` are both active at the same time.
  #[liberty(simple(type = Option))]
  pub clear_preset_var2: Option<ClearPresetState>,
  /// The `enable`  attribute gives the state of the enable input,
  /// and `data_in`  attribute gives the state of the data input.
  /// The `enable`  and `data_in`  attributes are optional,
  /// but if you use one of them, you must also use the other.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=185.11&end=185.13
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub enable: Option<BooleanExpression>,
  /// The `enable_also`  attribute gives the state of the `enable`
  /// input when you are describing master and slave cells.
  /// The `enable_also`  attribute is optional.
  /// If you use `enable_also`, you must also use the enable  and data_in  attributes
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=185.23&end=185.25
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub enable_also: Option<BooleanExpression>,
  /// The value of `variable1` after the active edge.
  #[liberty(simple(type = Option))]
  pub data_in: Option<BooleanExpression>,
  /// The `preset` attribute gives the active value for the preset input.
  #[liberty(simple(type = Option))]
  pub preset: Option<BooleanExpression>,
}

#[duplicate::duplicate_item(
  LatchFF_type;
  [Latch];
  [FF];
)]
impl NamedGroup for LatchFF_type {
  #[inline]
  fn parse_set_name(&mut self, mut v: Vec<&str>) -> Result<(), IdError> {
    let l = v.len();
    if l != 2 {
      return Err(IdError::length_dismatch(2, l, v));
    }
    v.pop()
      .map_or(Err(IdError::Other("Unkown pop error".into())), |variable2| {
        v.pop()
          .map_or(Err(IdError::Other("Unkown pop error".into())), |variable1| {
            self.variable1 = variable1.into();
            self.variable2 = variable2.into();
            Ok(())
          })
      })
  }
  #[inline]
  fn fmt_name<T: Write, I: crate::ast::Indentation>(
    &self,
    f: &mut crate::ast::CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    write!(f, "{}, {}", self.variable1, self.variable2)
  }
}

#[duplicate::duplicate_item(
  LatchFFBank_type;
  [LatchBank];
  [FFBank];
)]
impl NamedGroup for LatchFFBank_type {
  #[inline]
  fn parse_set_name(&mut self, mut v: Vec<&str>) -> Result<(), IdError> {
    let l = v.len();
    if l != 3 {
      return Err(IdError::length_dismatch(3, l, v));
    }
    v.pop()
      .map_or(Err(IdError::Other("Unkown pop error".into())), |bits_str| {
        match bits_str.parse::<usize>() {
          Ok(bits) => {
            v.pop()
              .map_or(Err(IdError::Other("Unkown pop error".into())), |variable2| {
                v.pop().map_or(
                  Err(IdError::Other("Unkown pop error".into())),
                  |variable1| {
                    self.variable1 = variable1.into();
                    self.variable2 = variable2.into();
                    self.bits = bits;
                    Ok(())
                  },
                )
              })
          }
          Err(e) => Err(IdError::Int(e)),
        }
      })
  }
  #[inline]
  fn fmt_name<T: Write, I: crate::ast::Indentation>(
    &self,
    f: &mut crate::ast::CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    write!(f, "{}, {}, {}", self.variable1, self.variable2, self.bits)
  }
}

#[duplicate::duplicate_item(
  Latch_type;
  [LatchBank];
  [Latch];
)]
impl __LatchFF for Latch_type {
  #[inline]
  fn variable1(&self) -> &ArcStr {
    &self.variable1
  }
  #[inline]
  fn variable2(&self) -> &ArcStr {
    &self.variable2
  }
  #[inline]
  fn clear(&self) -> &Option<BooleanExpression> {
    &self.clear
  }
  #[inline]
  fn clear_preset_var1(&self) -> &Option<ClearPresetState> {
    &self.clear_preset_var1
  }
  #[inline]
  fn clear_preset_var2(&self) -> &Option<ClearPresetState> {
    &self.clear_preset_var2
  }
  #[inline]
  fn active(&self) -> Option<Box<Expr>> {
    self.enable.as_ref().map(|e| Box::new(e.expr.clone()))
  }
  #[inline]
  fn active_also(&self) -> Option<Box<Expr>> {
    self.enable_also.as_ref().map(|e| Box::new(e.expr.clone()))
  }
  #[inline]
  fn next_state(&self) -> &Option<BooleanExpression> {
    &self.data_in
  }
  #[inline]
  fn preset(&self) -> &Option<BooleanExpression> {
    &self.preset
  }
}

#[duplicate::duplicate_item(
  FF_type;
  [FFBank];
  [FF];
)]
impl __LatchFF for FF_type {
  #[inline]
  fn variable1(&self) -> &ArcStr {
    &self.variable1
  }
  #[inline]
  fn variable2(&self) -> &ArcStr {
    &self.variable2
  }
  #[inline]
  fn clear(&self) -> &Option<BooleanExpression> {
    &self.clear
  }
  #[inline]
  fn clear_preset_var1(&self) -> &Option<ClearPresetState> {
    &self.clear_preset_var1
  }
  #[inline]
  fn clear_preset_var2(&self) -> &Option<ClearPresetState> {
    &self.clear_preset_var2
  }
  #[inline]
  fn active(&self) -> Option<Box<Expr>> {
    self.clocked_on.as_ref().map(|clocked_on| {
      let previous_clocked_on = clocked_on.previous();
      Box::new(Expr::And(
        Box::new(Expr::Not(Box::new(previous_clocked_on))),
        Box::new(clocked_on.expr.clone()),
      ))
    })
  }
  #[inline]
  fn active_also(&self) -> Option<Box<Expr>> {
    self.clocked_on_also.as_ref().map(|clocked_on_also| {
      let previous_clocked_on_also = clocked_on_also.previous();
      Box::new(Expr::And(
        Box::new(Expr::Not(Box::new(previous_clocked_on_also))),
        Box::new(clocked_on_also.expr.clone()),
      ))
    })
  }
  #[inline]
  fn next_state(&self) -> &Option<BooleanExpression> {
    &self.next_state
  }
  #[inline]
  fn preset(&self) -> &Option<BooleanExpression> {
    &self.preset
  }
}

trait __LatchFF {
  fn variable1(&self) -> &ArcStr;
  fn variable2(&self) -> &ArcStr;
  fn clear(&self) -> &Option<BooleanExpression>;
  fn clear_preset_var1(&self) -> &Option<ClearPresetState>;
  fn clear_preset_var2(&self) -> &Option<ClearPresetState>;
  fn active(&self) -> Option<Box<Expr>>;
  fn active_also(&self) -> Option<Box<Expr>>;
  fn next_state(&self) -> &Option<BooleanExpression>;
  fn preset(&self) -> &Option<BooleanExpression>;
}

#[duplicate::duplicate_item(
  AllTypes;
  [FFBank];
  [FF];
  [LatchBank];
  [Latch];
)]
impl LatchFF for AllTypes {}
#[duplicate::duplicate_item(
  AllTypes;
  [FFBank];
  [FF];
  [LatchBank];
  [Latch];
)]
impl GroupFn for AllTypes {}
/// trait for `FF` and `FFBank`
#[allow(private_bounds)]
pub trait LatchFF: __LatchFF {
  /// Get the `BooleanExpression` of variable1
  #[inline]
  fn variable1_expr(&self) -> BooleanExpression {
    let present_state = Box::new(Expr::Variable(self.variable1().to_string()));
    let active_edge_variable = self.next_state().as_ref().map_or(
      Expr::Variable(self.variable1().to_string()),
      |_next_state| {
        let next_state = Box::new(_next_state.expr.clone());
        match (self.active(), self.active_also()) {
          (None, None) => Expr::Variable(self.variable1().to_string()),
          (None, Some(active_also)) => {
            Expr::Cond(active_also, next_state, present_state.clone())
          }
          (Some(active), None) => Expr::Cond(active, next_state, present_state.clone()),
          (Some(active), Some(active_also)) => {
            // clocked_on? (clocked_on_also? unknown : next_state) : (clocked_on_also? next_state : present_state)
            Expr::Cond(
              active,
              Box::new(Expr::Cond(
                active_also.clone(),
                UNKNOWN.clone(),
                next_state.clone(),
              )),
              Box::new(Expr::Cond(active_also, next_state, present_state.clone())),
            )
          }
        }
      },
    );

    let expr = match (self.preset(), self.clear()) {
      (None, None) => active_edge_variable,
      (None, Some(clear)) => Expr::Cond(
        Box::new(clear.expr.clone()),
        Box::new(Expr::Const(false)),
        Box::new(active_edge_variable),
      ),
      (Some(preset), None) => Expr::Cond(
        Box::new(preset.expr.clone()),
        Box::new(Expr::Const(true)),
        Box::new(active_edge_variable),
      ),
      (Some(_preset), Some(_clear)) => {
        let clear_preset = match self.clear_preset_var1() {
          Some(ClearPresetState::L) => Box::new(Expr::Const(false)),
          Some(ClearPresetState::H) => Box::new(Expr::Const(true)),
          Some(ClearPresetState::N) => present_state,
          Some(ClearPresetState::T) => Box::new(Expr::Not(present_state)),
          //   Some(ClearPresetState::X) => UNKNOWN.clone(),
          _ => UNKNOWN.clone(),
        };
        let preset = Box::new(_preset.expr.clone());
        let clear = Box::new(_clear.expr.clone());
        // clear? (preset? clear_preset : 0) : (preset? 1 : active_edge_variable)
        Expr::Cond(
          clear,
          Box::new(Expr::Cond(
            preset.clone(),
            clear_preset,
            Box::new(Expr::Const(false)),
          )),
          Box::new(Expr::Cond(
            preset,
            Box::new(Expr::Const(true)),
            Box::new(active_edge_variable),
          )),
        )
      }
    };
    BooleanExpression { expr }
  }

  /// Get the `BooleanExpression` of variable2
  #[inline]
  fn variable2_expr(&self) -> BooleanExpression {
    let present_state = Box::new(Expr::Variable(self.variable2().to_string()));
    let active_edge_variable = self.next_state().as_ref().map_or(
      Expr::Variable(self.variable2().to_string()),
      |_next_state| {
        let next_state = Box::new(Expr::Not(Box::new(_next_state.expr.clone())));
        match (self.active(), self.active_also()) {
          (None, None) => Expr::Variable(self.variable2().to_string()),
          (None, Some(active_also)) => {
            Expr::Cond(active_also, next_state, present_state.clone())
          }
          (Some(active), None) => Expr::Cond(active, next_state, present_state.clone()),
          (Some(active), Some(active_also)) => {
            // clocked_on? (clocked_on_also? unknown : next_state) : (clocked_on_also? next_state : present_state)
            Expr::Cond(
              active,
              Box::new(Expr::Cond(
                active_also.clone(),
                UNKNOWN.clone(),
                next_state.clone(),
              )),
              Box::new(Expr::Cond(active_also, next_state, present_state.clone())),
            )
          }
        }
      },
    );
    let expr = match (self.preset(), self.clear()) {
      (None, None) => active_edge_variable,
      (None, Some(clear)) => Expr::Cond(
        Box::new(clear.expr.clone()),
        Box::new(Expr::Const(true)),
        Box::new(active_edge_variable),
      ),
      (Some(preset), None) => Expr::Cond(
        Box::new(preset.expr.clone()),
        Box::new(Expr::Const(false)),
        Box::new(active_edge_variable),
      ),
      (Some(_preset), Some(_clear)) => {
        let clear_preset = match self.clear_preset_var2() {
          Some(ClearPresetState::L) => Box::new(Expr::Const(false)),
          Some(ClearPresetState::H) => Box::new(Expr::Const(true)),
          Some(ClearPresetState::N) => present_state,
          Some(ClearPresetState::T) => Box::new(Expr::Not(present_state)),
          //   Some(ClearPresetState::X) => UNKNOWN.clone(),
          _ => UNKNOWN.clone(),
        };
        let preset = Box::new(_preset.expr.clone());
        let clear = Box::new(_clear.expr.clone());
        // clear? (preset? clear_preset : 1) : (preset? 0 : active_edge_variable)
        Expr::Cond(
          clear,
          Box::new(Expr::Cond(preset.clone(), clear_preset, Box::new(Expr::Const(true)))),
          Box::new(Expr::Cond(
            preset,
            Box::new(Expr::Const(false)),
            Box::new(active_edge_variable),
          )),
        )
      }
    };
    BooleanExpression { expr }
  }
  /// Get the `BooleanExpression` of (variable1,variable2)
  #[allow(clippy::too_many_lines)]
  #[inline]
  fn variable_expr(&self) -> (BooleanExpression, BooleanExpression) {
    let present_state1 = Box::new(Expr::Variable(self.variable1().to_string()));
    let present_state2 = Box::new(Expr::Variable(self.variable2().to_string()));
    let (active_edge_variable1, active_edge_variable2) =
      self.next_state().as_ref().map_or(
        (
          Expr::Variable(self.variable1().to_string()),
          Expr::Variable(self.variable2().to_string()),
        ),
        |next_state| {
          let next_state1 = Box::new(next_state.expr.clone());
          let next_state2 = Box::new(Expr::Not(next_state1.clone()));
          match (self.active(), self.active_also()) {
            (None, None) => (
              Expr::Variable(self.variable1().to_string()),
              Expr::Variable(self.variable2().to_string()),
            ),
            (None, Some(active_also)) => (
              Expr::Cond(active_also.clone(), next_state1, present_state1.clone()),
              Expr::Cond(active_also, next_state2, present_state2.clone()),
            ),
            (Some(active), None) => (
              Expr::Cond(active.clone(), next_state1, present_state1.clone()),
              Expr::Cond(active, next_state2, present_state2.clone()),
            ),
            (Some(active), Some(active_also)) => {
              // clocked_on? (clocked_on_also? unknown : next_state) : (clocked_on_also? next_state : present_state)
              (
                Expr::Cond(
                  active.clone(),
                  Box::new(Expr::Cond(
                    active_also.clone(),
                    UNKNOWN.clone(),
                    next_state1.clone(),
                  )),
                  Box::new(Expr::Cond(
                    active_also.clone(),
                    next_state1,
                    present_state1.clone(),
                  )),
                ),
                Expr::Cond(
                  active,
                  Box::new(Expr::Cond(
                    active_also.clone(),
                    UNKNOWN.clone(),
                    next_state2.clone(),
                  )),
                  Box::new(Expr::Cond(active_also, next_state2, present_state2.clone())),
                ),
              )
            }
          }
        },
      );
    let (expr1, expr2) = match (self.preset(), self.clear()) {
      (None, None) => (active_edge_variable1, active_edge_variable2),
      (None, Some(clear)) => (
        Expr::Cond(
          Box::new(clear.expr.clone()),
          Box::new(Expr::Const(false)),
          Box::new(active_edge_variable1),
        ),
        Expr::Cond(
          Box::new(clear.expr.clone()),
          Box::new(Expr::Const(true)),
          Box::new(active_edge_variable2),
        ),
      ),
      (Some(preset), None) => (
        Expr::Cond(
          Box::new(preset.expr.clone()),
          Box::new(Expr::Const(true)),
          Box::new(active_edge_variable1),
        ),
        Expr::Cond(
          Box::new(preset.expr.clone()),
          Box::new(Expr::Const(false)),
          Box::new(active_edge_variable2),
        ),
      ),
      (Some(_preset), Some(_clear)) => {
        let clear_preset1 = match self.clear_preset_var1() {
          Some(ClearPresetState::L) => Box::new(Expr::Const(false)),
          Some(ClearPresetState::H) => Box::new(Expr::Const(true)),
          Some(ClearPresetState::N) => present_state1,
          Some(ClearPresetState::T) => Box::new(Expr::Not(present_state1)),
          //   Some(ClearPresetState::X) => UNKNOWN.clone(),
          _ => UNKNOWN.clone(),
        };
        let clear_preset2 = match self.clear_preset_var1() {
          Some(ClearPresetState::L) => Box::new(Expr::Const(false)),
          Some(ClearPresetState::H) => Box::new(Expr::Const(true)),
          Some(ClearPresetState::N) => present_state2,
          Some(ClearPresetState::T) => Box::new(Expr::Not(present_state2)),
          //   Some(ClearPresetState::X) => UNKNOWN.clone(),
          _ => UNKNOWN.clone(),
        };
        let preset = Box::new(_preset.expr.clone());
        let clear = Box::new(_clear.expr.clone());
        // clear? (preset? clear_preset : 0) : (preset? 1 : active_edge_variable)
        (
          Expr::Cond(
            clear.clone(),
            Box::new(Expr::Cond(
              preset.clone(),
              clear_preset1,
              Box::new(Expr::Const(false)),
            )),
            Box::new(Expr::Cond(
              preset.clone(),
              Box::new(Expr::Const(true)),
              Box::new(active_edge_variable1),
            )),
          ),
          Expr::Cond(
            clear,
            Box::new(Expr::Cond(
              preset.clone(),
              clear_preset2,
              Box::new(Expr::Const(true)),
            )),
            Box::new(Expr::Cond(
              preset,
              Box::new(Expr::Const(false)),
              Box::new(active_edge_variable2),
            )),
          ),
        )
      }
    };
    (BooleanExpression { expr: expr1 }, BooleanExpression { expr: expr2 })
  }
}

/// `L | H | N | T | X`
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=158.29&end=159.10
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClearPresetState {
  /// 0
  #[strum(serialize = "L")]
  L,
  /// 1
  #[strum(serialize = "H")]
  H,
  /// No Change
  #[strum(serialize = "N")]
  N,
  /// Toggle the current value from `1` to `0`, `0` to `1`, or `X` to `X`
  #[strum(serialize = "T")]
  T,
  /// Unknown
  #[strum(serialize = "X")]
  X,
}

impl crate::ast::SimpleAttri for ClearPresetState {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

#[cfg(test)]
mod test {
  #[allow(unused_imports)]
  use crate::expression::{FFBank, IdBooleanExpression, Latch, LatchBank, LatchFF, FF};
  #[test]
  fn special_boolean_expression() {
    let ff = crate::ast::test_parse_fmt::<FF>(
      r#"(IQ,IQN) {
        next_state : "(J K IQ') + (J K') + (J' K' IQ)";
        clocked_on : "\"1A\" + \"1B\"";
      }
    "#,
      r#"
liberty_db::expression::boolean_expression::latch_ff::FF (IQ, IQN) {
| clocked_on : "\"1A\"+\"1B\"";
| next_state : "J*K*!IQ+J*!K+!J*!K*IQ";
}"#,
    );
    let var1_expr = ff.variable1_expr();
    let var2_expr = ff.variable2_expr();
    println!("{var1_expr}");
    println!("{var2_expr}");
    let id_var1_expr: IdBooleanExpression = var1_expr.into();
    let id_var2_expr: IdBooleanExpression = var2_expr.into();
    let (var1_expr_, var2_expr_) = ff.variable_expr();
    let id_var1_expr_: IdBooleanExpression = var1_expr_.into();
    let id_var2_expr_: IdBooleanExpression = var2_expr_.into();
    assert_eq!(id_var1_expr_, id_var1_expr);
    assert_eq!(id_var2_expr_, id_var2_expr);
  }
  /// In some flip-flops, the next state depends on the current state.
  /// In this case, the first state variable (IQ  in the example)
  /// can be used in the `next_state`  statement;
  /// the second state variable, IQN, cannot.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=160.25&end=160.29
  /// ">Reference</a>
  #[test]
  fn jk_flip_flop() {
    let ff = crate::ast::test_parse_fmt::<FF>(
      r#"(IQ,IQN) {
        next_state : "(J K IQ') + (J K') + (J' K' IQ)";
        clocked_on : "CP";
      }
    "#,
      r#"
liberty_db::expression::boolean_expression::latch_ff::FF (IQ, IQN) {
| clocked_on : "CP";
| next_state : "J*K*!IQ+J*!K+!J*!K*IQ";
}"#,
    );
    let var1_expr = ff.variable1_expr();
    let var2_expr = ff.variable2_expr();
    println!("{var1_expr}");
    println!("{var2_expr}");
    let id_var1_expr: IdBooleanExpression = var1_expr.into();
    let id_var2_expr: IdBooleanExpression = var2_expr.into();
    let (var1_expr_, var2_expr_) = ff.variable_expr();
    let id_var1_expr_: IdBooleanExpression = var1_expr_.into();
    let id_var2_expr_: IdBooleanExpression = var2_expr_.into();
    assert_eq!(id_var1_expr_, id_var1_expr);
    assert_eq!(id_var2_expr_, id_var2_expr);
  }

  /// Example 19 is an ff  group for a single-stage D flip-flop with
  /// rising-edge sampling, negative clear and preset, and output pins
  /// set to 0 when both clear and preset are active (low).
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=162.2&end=162.4
  /// ">Reference</a>
  #[test]
  fn example19() {
    let ff = crate::ast::test_parse_fmt::<FF>(
      r#"(IQ, IQN) {  
        next_state : "D" ;  
        clocked_on : "CP" ;  
        clear : "CD’" ;  
        preset : "PD’" ;  
        clear_preset_var1 : L ;  
        clear_preset_var2 : L ;
    }
    "#,
      r#"
liberty_db::expression::boolean_expression::latch_ff::FF (IQ, IQN) {
| clear : "!CD";
| clear_preset_var1 : L;
| clear_preset_var2 : L;
| clocked_on : "CP";
| next_state : "D";
| preset : "!PD";
}"#,
    );
    let var1_expr = ff.variable1_expr();
    let var2_expr = ff.variable2_expr();
    println!("{var1_expr}");
    println!("{var2_expr}");
    let id_var1_expr: IdBooleanExpression = var1_expr.into();
    let id_var2_expr: IdBooleanExpression = var2_expr.into();
    let (var1_expr_, var2_expr_) = ff.variable_expr();
    let id_var1_expr_: IdBooleanExpression = var1_expr_.into();
    let id_var2_expr_: IdBooleanExpression = var2_expr_.into();
    assert_eq!(id_var1_expr_, id_var1_expr);
    assert_eq!(id_var2_expr_, id_var2_expr);
  }

  /// Example 20 is an ff group for a single-stage, rising-edge-triggered
  /// JK flip-flop with scan input, negative clear and preset, and
  /// output pins set to 0 when clear and preset are both active.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=160.25&end=160.29
  /// ">Reference</a>
  #[test]
  fn example20() {
    let ff = crate::ast::test_parse_fmt::<FF>(
      r#"(IQ, IQN) {  
        next_state : "(TE*TI)+(TE’*J*K’)+(TE’*J’*K’*IQ)+(TE’*J*K*IQ’)" ;  
        clocked_on : "CP" ;  
        clear : "CD’" ;  
        preset : "PD’" ;  
        clear_preset_var1 : L ; 
        clear_preset_var2 : L ;
      }
    "#,
      r#"
liberty_db::expression::boolean_expression::latch_ff::FF (IQ, IQN) {
| clear : "!CD";
| clear_preset_var1 : L;
| clear_preset_var2 : L;
| clocked_on : "CP";
| next_state : "TE*TI+!TE*J*!K+!TE*!J*!K*IQ+!TE*J*K*!IQ";
| preset : "!PD";
}"#,
    );
    let var1_expr = ff.variable1_expr();
    let var2_expr = ff.variable2_expr();
    println!("{var1_expr}");
    println!("{var2_expr}");
    let id_var1_expr: IdBooleanExpression = var1_expr.into();
    let id_var2_expr: IdBooleanExpression = var2_expr.into();
    let (var1_expr_, var2_expr_) = ff.variable_expr();
    let id_var1_expr_: IdBooleanExpression = var1_expr_.into();
    let id_var2_expr_: IdBooleanExpression = var2_expr_.into();
    assert_eq!(id_var1_expr_, id_var1_expr);
    assert_eq!(id_var2_expr_, id_var2_expr);
  }

  /// Example 21 is an ff group for a D flip-flop with synchronous negative clear.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=162.26&end=162.31
  /// ">Reference</a>
  #[test]
  fn example21() {
    let ff = crate::ast::test_parse_fmt::<FF>(
      r#"(IQ, IQN) {   
        next_state : "D * CLR’" ;   
        clocked_on : "CP" ;
    }
    "#,
      r#"
liberty_db::expression::boolean_expression::latch_ff::FF (IQ, IQN) {
| clocked_on : "CP";
| next_state : "D*!CLR";
}"#,
    );
    let var1_expr = ff.variable1_expr();
    let var2_expr = ff.variable2_expr();
    println!("{var1_expr}");
    println!("{var2_expr}");
    let id_var1_expr: IdBooleanExpression = var1_expr.into();
    let id_var2_expr: IdBooleanExpression = var2_expr.into();
    let (var1_expr_, var2_expr_) = ff.variable_expr();
    let id_var1_expr_: IdBooleanExpression = var1_expr_.into();
    let id_var2_expr_: IdBooleanExpression = var2_expr_.into();
    assert_eq!(id_var1_expr_, id_var1_expr);
    assert_eq!(id_var2_expr_, id_var2_expr);
  }

  /// Example 22 shows an ff  group for a master-slave D flip-flop with
  /// rising-edge sampling, falling-edge data transfer, negative clear and preset,
  /// and output values set high when clear and preset are both active
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=163.11&end=163.13
  /// ">Reference</a>
  #[test]
  fn example22() {
    let ff = crate::ast::test_parse_fmt::<FF>(
      r#"(IQ, IQN) {  
        next_state : "D" ;  
        clocked_on : "CLK" ;  
        clocked_on_also : "CLKN’" ;  
        clear : "CDN’" ;  
        preset : "PDN’" ;  
        clear_preset_var1 : H ;  
        clear_preset_var2 : H ;
    }
    "#,
      r#"
liberty_db::expression::boolean_expression::latch_ff::FF (IQ, IQN) {
| clear : "!CDN";
| clear_preset_var1 : H;
| clear_preset_var2 : H;
| clocked_on : "CLK";
| clocked_on_also : "!CLKN";
| next_state : "D";
| preset : "!PDN";
}"#,
    );
    let var1_expr = ff.variable1_expr();
    let var2_expr = ff.variable2_expr();
    println!("{var1_expr}");
    println!("{var2_expr}");
    let id_var1_expr: IdBooleanExpression = var1_expr.into();
    let id_var2_expr: IdBooleanExpression = var2_expr.into();
    let (var1_expr_, var2_expr_) = ff.variable_expr();
    let id_var1_expr_: IdBooleanExpression = var1_expr_.into();
    let id_var2_expr_: IdBooleanExpression = var2_expr_.into();
    assert_eq!(id_var1_expr_, id_var1_expr);
    assert_eq!(id_var2_expr_, id_var2_expr);
  }
  /// Example 23 A multibit register containing four rising-edge-triggered D flip-flops
  /// with clear  and preset is shown in Figure 1 and Example 23
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=167.32&end=167.33
  /// ">Reference</a>
  #[test]
  fn example23() {
    let ff = crate::ast::test_parse_fmt::<FFBank>(
      r#"(IQ, IQN, 4) {    
        next_state : "D" ;    
        clocked_on : "CLK" ;    
        clear : "CLR’" ;    
        preset : "PRE’" ;    
        clear_preset_var1 : L ;    
        clear_preset_var2 : L ;   
      }
    "#,
      r#"
liberty_db::expression::boolean_expression::latch_ff::FFBank (IQ, IQN, 4) {
| clear : "!CLR";
| clear_preset_var1 : L;
| clear_preset_var2 : L;
| clocked_on : "CLK";
| next_state : "D";
| preset : "!PRE";
}"#,
    );
    let var1_expr = ff.variable1_expr();
    let var2_expr = ff.variable2_expr();
    println!("{var1_expr}");
    println!("{var2_expr}");
    let id_var1_expr: IdBooleanExpression = var1_expr.into();
    let id_var2_expr: IdBooleanExpression = var2_expr.into();
    let (var1_expr_, var2_expr_) = ff.variable_expr();
    let id_var1_expr_: IdBooleanExpression = var1_expr_.into();
    let id_var2_expr_: IdBooleanExpression = var2_expr_.into();
    assert_eq!(id_var1_expr_, id_var1_expr);
    assert_eq!(id_var2_expr_, id_var2_expr);
  }
  /// Example 25 D Latch With Active-High Enable and Negative Clear
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=186.19&end=186.24
  /// ">Reference</a>
  #[test]
  fn example25() {
    let latch = crate::ast::test_parse_fmt::<Latch>(
      r#"(IQ, IQN) {
        enable : "G" ;
        data_in : "D" ;
        clear : "CD’" ;
       }
    "#,
      r#"
liberty_db::expression::boolean_expression::latch_ff::Latch (IQ, IQN) {
| clear : "!CD";
| enable : "G";
| data_in : "D";
}"#,
    );
    let var1_expr = latch.variable1_expr();
    let var2_expr = latch.variable2_expr();
    println!("{var1_expr}");
    println!("{var2_expr}");
    let id_var1_expr: IdBooleanExpression = var1_expr.into();
    let id_var2_expr: IdBooleanExpression = var2_expr.into();
    let (var1_expr_, var2_expr_) = latch.variable_expr();
    let id_var1_expr_: IdBooleanExpression = var1_expr_.into();
    let id_var2_expr_: IdBooleanExpression = var2_expr_.into();
    assert_eq!(id_var1_expr_, id_var1_expr);
    assert_eq!(id_var2_expr_, id_var2_expr);
  }
  /// Example 25 SR latch. The `enable`  and `data_in`  attributes are not required for an SR latch.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=186.25&end=186.33
  /// ">Reference</a>
  #[test]
  fn example26() {
    let latch = crate::ast::test_parse_fmt::<Latch>(
      r#"(IQ, IQN) {  
        clear : "S’" ;  
        preset : "R’" ;  
        clear_preset_var1 : L ;  
        clear_preset_var2 : L ;
      }
    "#,
      r#"
liberty_db::expression::boolean_expression::latch_ff::Latch (IQ, IQN) {
| clear : "!S";
| clear_preset_var1 : L;
| clear_preset_var2 : L;
| preset : "!R";
}"#,
    );
    let var1_expr = latch.variable1_expr();
    let var2_expr = latch.variable2_expr();
    println!("{var1_expr}");
    println!("{var2_expr}");
    let id_var1_expr: IdBooleanExpression = var1_expr.into();
    let id_var2_expr: IdBooleanExpression = var2_expr.into();
    let (var1_expr_, var2_expr_) = latch.variable_expr();
    let id_var1_expr_: IdBooleanExpression = var1_expr_.into();
    let id_var2_expr_: IdBooleanExpression = var2_expr_.into();
    assert_eq!(id_var1_expr_, id_var1_expr);
    assert_eq!(id_var2_expr_, id_var2_expr);
  }
}
