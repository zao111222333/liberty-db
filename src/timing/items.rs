//! All item structure inside
//! `Timing`.

use core::ops::Not;

use crate::{
  ast::{self, GroupComments, GroupFn, ParseScope, SimpleAttri},
  expression::logic,
  ArcStr, NotNan,
};
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
#[derive(Debug, Clone, Copy, PartialEq, Default, Hash, Eq, PartialOrd, Ord)]
#[derive(liberty_macros::EnumToken)]
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
  #[token("positive_unate")]
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
  #[token("negative_unate")]
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
  #[token("non_unate")]
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
impl SimpleAttri for TimingSenseType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
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
pub struct CellDegradation {
  /// name
  #[size = 8]
  #[liberty(name)]
  #[id(borrow = "&str")]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
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
impl GroupFn for CellDegradation {}
