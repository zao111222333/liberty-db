//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
use crate::{
  ast::{AttributeList, GroupComments, GroupFn, SimpleAttri},
  common::items::DummyGroup,
  expression::IdBooleanExpression,
  timing::items::Mode,
};
use std::{fmt::Display, str::FromStr};

// /// In referenced CCS noise modeling,
// /// use the `input_ccb`  group to specify the CCS noise for
// /// an input channel-connected block (CCB).
// /// You must name the `input_ccb`  group so that it can be referenced.
// /// The `input_ccb`  group includes all the attributes and subgroups
// /// of the `ccsn_first_stage` Group  on page 283.
// /// The `input_ccb`  group also includes the `related_ccb_node`  simple attribute.
// /// <a name ="reference_link" href="
// /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=296.7&end=296.12
// /// ">Reference-Instance</a>
// #[derive(Debug, Default, Clone)]
// #[derive(liberty_macros::Group)]
// #[mut_set_derive::item(
//   sort,
//   macro(derive(Debug, Clone,Default);)
// )]
// pub struct CCB {
//   #[liberty(name)]
//   #[id]
//   pub name: Vec<String>,
//   #[liberty(comments)]
//   _comments: GroupComments<Self>,
//   #[liberty(undefined)]
//   _undefined: AttributeList,
// }

/// Use the `ccsn_first_stage` group to specify CCS noise for the first stage of the channel-
/// connected block (CCB).
/// A `ccsn_first_stage` or `ccsn_last_stage` group contains the following information:
/// • A set of channel-connected block parameters: the `is_needed`, `is_inverting`,
/// stage_type, `miller_cap_rise`, `miller_cap_fall`, and optional `related_ccb_node`
/// attributes
/// • The optional `when` and `mode` attributes for conditional data modeling
/// • The optional `output_signal_level` or `input_signal_level` attribute to model CCS
/// noise stages of channel-connected blocks with internal power supplies
/// • A two-dimensional DC current table: `dc_current` group
/// • Two timing tables for rising and falling transitions: `output_current_rise` group,
/// `output_current_fall` group
/// • Two noise tables for low and high propagated noise: `propagated_noise_low` group,
/// `propagated_noise_high` group
/// Note that if the `ccsn_first_stage` and `ccsn_last_stage` groups are defined inside pin-
/// level groups, then the `ccsn_first_stage` group can only be defined in an input pin or an
/// inout pin, and the `ccsn_last_stage` group can only be defined in an output pin or an inout
/// pin.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=283.21&end=283.43
/// ">Reference-Definition</a>
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct CCSNStage {
  #[liberty(name)]
  #[id]
  pub name: Vec<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  /// Use the `is_inverting`  attribute to specify whether the channel-connecting block is inverting.
  /// This attribute is mandatory if the `is_needed` attribute value is true.
  /// If the channel-connecting block is inverting, set the attribute to true.
  /// Otherwise, set the attribute to false.
  /// This attribute is different from the timing sense of a timing arc,
  /// which might consist of multiple channel-connecting blocks.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=285.31&end=285.36
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub is_inverting: bool,
  /// Use the `is_needed`  attribute to specify
  /// whether composite current source (CCS) noise modeling data is required.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=286.5&end=286.6
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub is_needed: bool,
  /// The `is_pass_gate`  attribute is defined in a ccsn_*_stage  group,
  /// such as the `ccsn_first_stage`  group,
  /// to indicate that the ccsn_*_stage  information is modeled as a pass gate.
  /// The attribute is optional and the default is false.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=286.17&end=286.19
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub is_pass_gate: Option<bool>,
  /// Use the `miller_cap_fall`  attribute to specify the Miller capacitance value for the channel-connecting block.
  /// /// A floating-point number representing the Miller capacitance value. The value must be greater or equal to zero.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=286.25&end=286.26
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub miller_cap_fall: f64,
  /// Use the `miller_cap_rise`  attribute to specify the Miller capacitance value for the channel-connecting block.
  /// A floating-point number representing the Miller capacitance value. The value must be greater or equal to zero.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=287.3&end=287.11
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub miller_cap_rise: f64,
  /// The optional `related_ccb_node`  attribute specifies the SPICE node
  /// in the subcircuit netlist that is used for the `dc_current`  
  /// table characterization and waveform measurements.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=287.15&end=287.17
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub related_ccb_node: Option<String>,
  /// Use the `stage_type`  attribute to specify the stage type of the channel-connecting block output voltage.
  ///
  /// The valid values are `pull_up`,in which the output voltage of the channel-connecting block is always pulled up (rising);
  /// `pull_down`, in which the output voltage of the channel-connecting block is always pulled down (falling);
  /// and `both`, in which the output voltage of the channel-connecting block is pulled up or down.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=287.27+288.2&end=287.36+288.5
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub stage_type: StageType,
  #[id]
  #[liberty(simple(type = Option))]
  pub when: Option<IdBooleanExpression>,
  /// The pin-based mode  attribute is provided in the `ccsn_first_stage`  
  /// and `ccsn_last_stage` groups for conditional data modeling.
  /// If the `mode`  attribute is specified, `mode_name`  and `mode_value`  must be
  /// predefined in the `mode_definition`  group at the cell level.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=287.23&end=287.25
  /// ">Reference-Definition</a>
  #[liberty(complex(type = Option))]
  pub mode: Option<Mode>,
  #[liberty(group(type = Option))]
  pub dc_current: Option<DummyGroup>,
  #[liberty(group(type = Option))]
  pub output_voltage_fall: Option<DummyGroup>,
  #[liberty(group(type = Option))]
  pub output_voltage_rise: Option<DummyGroup>,
  #[liberty(group(type = Option))]
  pub propagated_noise_low: Option<DummyGroup>,
  #[liberty(group(type = Option))]
  pub propagated_noise_rise: Option<DummyGroup>,
}

impl GroupFn for CCSNStage {
  fn post_process(&mut self) {
    if self.miller_cap_fall < 0.0 {
      self.miller_cap_fall = 0.0;
      warn!("miller_cap_fall is negative!");
    }
    if self.miller_cap_rise < 0.0 {
      self.miller_cap_rise = 0.0;
      warn!("miller_cap_rise is negative!");
    }
  }
}

/// Use the `stage_type`  attribute to specify the stage type of the channel-connecting block output voltage.
///
/// The valid values are `pull_up`,in which the output voltage of the channel-connecting block is always pulled up (rising);
/// `pull_down`, in which the output voltage of the channel-connecting block is always pulled down (falling);
/// and `both`, in which the output voltage of the channel-connecting block is pulled up or down.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=287.27+288.2&end=287.36+288.5
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd, Default)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
pub enum StageType {
  /// pull_up,in which the output voltage of the channel-connecting block is always pulled up (rising);
  #[strum(serialize = "pull_up")]
  PullUp,
  /// in which the output voltage of the channel-connecting block is always pulled down (falling)
  #[strum(serialize = "pull_down")]
  PullDown,
  /// both, in which the output voltage of the channel-connecting block is pulled up or down
  #[strum(serialize = "both")]
  #[default]
  Both,
}
impl SimpleAttri for StageType {}
