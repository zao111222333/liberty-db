//! All item structure inside
//! `Timing`.

use std::collections::HashMap;

use crate::{
  ast::{self, GroupId},
  common::items::Domain,
  expression::{self, LogicLike},
};

use strum_macros::{Display, EnumString};
/// The `timing_sense` attribute describes the way an input pin logically affects an output pin.
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
/// + In the case of `rising_edge` and `positive_unate` values, only the `cell_rise` and `rise_transition`
/// information is required.
/// + In the case of `rising_edge` and `negative_unate` values, only the `cell_fall` and `fall_transition`
/// information is required.
/// + In the case of `falling_edge` and `positive_unate` values, only the `cell_rise` and `rise_transition`
/// information is required.
/// + In the case of `falling_edge` and `negative_unate` values, only the `cell_fall` and `fall_transition`
/// information is required.
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
/// If `related_pin` is an output pin, you must define a `timing_sense`` attribute for that pin.
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
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
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
  NonUnate,
}

impl TimingSenseType {
  pub fn compute_edge(
    &self,
    pin_edge: &expression::EdgeState,
  ) -> Option<expression::EdgeState> {
    match self {
      TimingSenseType::PositiveUnate => Some(*pin_edge),
      TimingSenseType::NegativeUnate => Some(pin_edge.inverse()),
      TimingSenseType::NonUnate => None,
    }
  }
}

/// You define the mode attribute within a timing group.
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
#[derive(Debug, Clone, Copy, Default)]
pub struct Mode {}

/// The `cell_degradation` group describes a cell performance degradation
/// design rule for compiling a design. A cell degradation design rule
/// specifies the maximum capacitive load a cell can drive without causing
/// cell performance degradation during the fall transition.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =225.4
/// +225.27
/// &end
/// =225.25
/// +227.51
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =204.9
/// &end
/// =204.9
/// ">Reference-Instance</a>
///
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
// #[derive(liberty_macros::NameIdx)]
pub struct CellDegradation {
  #[liberty(id(auto_impl_len = 1))]
  _id: GroupId<Self>,
  #[liberty(undefined)]
  _undefined: ast::AttributeList,
  // /* polynomial model */
  // #[arrti_type(complex)]
  // pub coefs: Vec<f64>,
  // /* polynomial model */
  // #[arrti_type(complex)]
  // pub orders: Vec<usize>,
  // /* lookup table */
  // #[arrti_type(complex)]
  // pub index_1: Vec<f64>,
  // /* lookup table */
  // #[arrti_type(complex)]
  // pub values: Vec<f64>,
  // /* polynomial model */
  // #[arrti_type(complex)]
  // pub variable_n_range: Option<(f64,f64)>,
  // #[arrti_type(group)]
  // pub domain: HashMap<<Domain as ast::HashedGroup>::Id,Domain>,
  // TODO:
  // pub domain: Option<Domain>,
}

/// Defines cell delay lookup tables (independently of transition delay) in CMOS nonlinear timing models.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =227.53
/// +228.27
/// &end
/// =228.25
/// +228.62
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =204.10
/// &end
/// =204.10
/// ">Reference-Instance</a>
///
/// **Note:**
/// The same k-factors that scale the cell_fall and cell_rise values also scale the
/// retaining_fall and retaining_rise values. There are no separate k-factors for
/// the retaining_fall and retaining_rise values.
///
/// **Used By:**
/// [Timing](crate::timing::Timing)
// #[derive(liberty_macros::NameIdx)]
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
pub struct CellFall {
  #[liberty(id(auto_impl_len = 0))]
  _id: GroupId<Self>,
  #[liberty(undefined)]
  _undefined: ast::AttributeList,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
  /// ?field=test
  /// &bgn
  /// =228.22
  /// &end
  /// =228.22
  /// ">Reference</a>
  pub index_1: Vec<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
  /// ?field=test
  /// &bgn
  /// =228.23
  /// &end
  /// =228.23
  /// ">Reference</a>
  pub index_2: Vec<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
  /// ?field=test
  /// &bgn
  /// =228.24
  /// &end
  /// =228.24
  /// ">Reference</a>
  pub index_3: Vec<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
  /// ?field=test
  /// &bgn
  /// =228.25
  /// &end
  /// =228.25
  /// ">Reference</a>
  pub values: Vec<Vec<Vec<f64>>>,
  // TODO:
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
  /// ?field=test
  /// &bgn
  /// =228.27
  /// &end
  /// =228.62
  /// ">Reference-Definition</a>
  pub domain: Domain,
}
