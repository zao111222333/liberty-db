//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
use crate::{
  ast::{AttributeList, GroupComments, GroupFn, SimpleAttri},
  common::table::{
    TableLookUp, TableLookUp2D, TableLookUpMultiSegment, Vector3DGrpup, Vector4DGrpup,
  },
  expression::IdBooleanExpression,
  timing::items::Mode,
  GroupSet,
};

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
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CCSNStage {
  #[liberty(name)]
  #[id]
  pub name: Vec<String>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  #[liberty(simple(type = Option))]
  pub load_cap_fall: Option<f64>,
  #[liberty(simple(type = Option))]
  pub load_cap_rise: Option<f64>,
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
  #[id]
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
  /// Use the `dc_current`  group to specify the input and output voltage values
  /// of a two-dimensional current table for a channel-connecting block.
  ///
  /// Use `index_1`  to represent the input voltage
  /// and `index_2`  to represent the output voltage.
  /// The `values`  attribute of the group lists the relative
  /// channel-connecting block DC current values in library units measured
  /// at the channel-connecting block output node.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=289.2+288.24&end=289.4+288.25
  /// ">Reference-Definition</a>
  #[liberty(group(type = Option))]
  pub dc_current: Option<TableLookUp2D>,
  /// Use the `output_voltage_fall`  group to specify vector groups that describe
  /// three-dimensional `output_voltage`  tables of the channel-connecting block
  /// whose output node’s voltage values are falling.
  ///
  /// + The `index_1`  attribute lists the `input_net_transition`  (slew) values in library time units.
  /// + The `index_2`  attribute lists the `total_output_net_capacitance`  (load) values in library capacitance units.
  /// + The `index_3` attribute lists the sampling time values in library time units.
  /// + The `values`  attribute lists the voltage values, in library voltage units,
  /// that are measured at the channel-connecting block output node.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=289.6&end=289.26
  /// ">Reference-Definition</a>
  #[liberty(group(type = Option))]
  pub output_voltage_fall: Option<Vector3DGrpup>,
  /// Use the `output_voltage_rise`  group to specify `vector` groups that describe
  /// three-dimensional `output_voltage`  tables of the channel-connecting block
  /// whose output node’s voltage values are rising.
  /// For details, see the `output_voltage_fall`  group description.
  ///
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=289.28&end=289.30
  /// ">Reference-Definition</a>
  #[liberty(group(type = Option))]
  pub output_voltage_rise: Option<Vector3DGrpup>,
  /// The `propagated_noise_low`  group uses `vector` groups to specify the
  /// three-dimensional `output_voltage`  tables of the channel-connecting block
  /// whose output node’s voltage values are falling.
  /// Specify the following attributes in the `vector`  group:
  ///
  /// + The `index_1`  attribute lists the `input_noise_height`  values in library voltage units.
  /// + The `index_2`  attribute lists the `input_noise_width`  values in library time units.
  /// + The `index_3`  attribute lists the `total_output_net_capacitance`  values in library capacitance units.
  /// + The `index_4` attribute lists the sampling time values in library time units.
  /// + The `values`  attribute lists the voltage values, in library voltage units, that are measured at the channel-connecting block output node.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=290.19&end=290.20
  /// ">Reference-Definition</a>
  #[liberty(group(type = Option))]
  pub propagated_noise_low: Option<Vector4DGrpup>,
  /// The `propagated_noise_high`  group uses `vector` groups to specify the
  /// three-dimensional `output_voltage`  tables of the channel-connecting block
  /// whose output node’s voltage values are rising.
  /// Specify the following attributes in the `vector`  group:
  ///
  /// + The `index_1`  attribute lists the `input_noise_height`  values in library voltage units.
  /// + The `index_2`  attribute lists the `input_noise_width`  values in library time units.
  /// + The `index_3`  attribute lists the `total_output_net_capacitance`  values in library capacitance units.
  /// + The `index_4` attribute lists the sampling time values in library time units.
  /// + The `values`  attribute lists the voltage values, in library voltage units, that are measured at the channel-connecting block output node.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=289.33&end=289.35
  /// ">Reference-Definition</a>
  #[liberty(group(type = Option))]
  pub propagated_noise_high: Option<Vector4DGrpup>,
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
#[derive(serde::Serialize, serde::Deserialize)]
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

/// Use the `receiver_capacitance`  group to specify capacitance values
/// for composite current source (CCS) receiver modeling at the pin level.
///
/// Groups
///
/// For two-segment receiver capacitance model
/// + receiver_capacitance1_fall
/// + receiver_capacitance1_rise
/// + receiver_capacitance2_fall
/// + receiver_capacitance2_rise
///
/// For multisegment receiver capacitance model
/// + receiver_capacitance_fall
/// + receiver_capacitance_rise
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=316.5&end=316.31
/// ">Reference-Definition</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReceiverCapacitance {
  #[id]
  #[liberty(name)]
  name: Option<String>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  #[id]
  #[liberty(simple(type=Option))]
  pub when: Option<IdBooleanExpression>,
  #[liberty(group(type=Set))]
  pub receiver_capacitance_fall: GroupSet<TableLookUpMultiSegment>,
  #[liberty(group(type=Set))]
  pub receiver_capacitance_rise: GroupSet<TableLookUpMultiSegment>,
  #[liberty(group)]
  pub receiver_capacitance1_fall: Option<TableLookUp>,
  #[liberty(group)]
  pub receiver_capacitance1_rise: Option<TableLookUp>,
  #[liberty(group)]
  pub receiver_capacitance2_fall: Option<TableLookUp>,
  #[liberty(group)]
  pub receiver_capacitance2_rise: Option<TableLookUp>,
}
impl GroupFn for ReceiverCapacitance {}

#[test]
fn parse_file() -> anyhow::Result<()> {
  use std::fs::File;
  use std::io::{BufWriter, Write};
  let filepath = "tests/tech/ccsn.lib";
  let data = std::fs::read_to_string(filepath).expect("Failed to open file.");
  match crate::library::Library::parse(&data) {
    Ok(library) => {
      let file = File::create("output.lib")?;
      let mut writer = BufWriter::new(file);
      write!(&mut writer, "{}", library)?;
    }
    Err(e) => panic!("[ERROR] {:?}", e),
  }
  Ok(())
}
