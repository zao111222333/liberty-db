//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

mod items;
mod test;
use crate::{
  ast::{
    Attributes, BuilderScope, DefaultIndentation, GroupComments, GroupFn, GroupSet,
    ParseScope, ParsingBuilder,
  },
  cell::Cell,
  common::table::{CompactLutTemplate, DriverWaveform, TableTemple},
  units, ArcStr, NotNan,
};
use core::fmt::{self, Write};
pub use items::*;

/// The first line of the library group statement names the library.
///
/// It is the first executable line in your library.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=20.24&end=20.26
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Library {
  /// library name
  #[id(borrow = "&str")]
  #[size = 8]
  #[liberty(name)]
  #[default = "arcstr::literal!(\"undefined\")"]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  #[size = 0]
  #[liberty(extra_ctx)]
  extra_ctx: (),
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// The `technology`  attribute statement specifies the technology
  /// family being used in the library.
  /// When you define the technology  attribute,
  /// it must be the first attribute you use and it must be placed at the top of the listing.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=39.3&end=39.5
  /// ">Reference</a>
  #[size = 8]
  #[liberty(complex)]
  #[default = "arcstr::literal!(\"cmos\")"]
  pub technology: ArcStr,
  /// Use the `delay_model`  attribute to specify which delay model
  /// to use in the delay calculations.
  /// The `delay_model`  attribute must be the first attribute in the library
  /// if a technology attribute is not present.
  /// Otherwise, it should follow the technology attribute.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=24.3&end=24.6
  /// ">Reference</a>
  #[size = 0]
  #[liberty(simple)]
  pub delay_model: DelayModel,
  /// You can use any format within the quotation marks to report the date
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=23.5&end=23.5
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub date: ArcStr,
  /// You use the `comment`  attribute to include copyright
  /// or other product information in the library report. You can include only one comment line in a library
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=22.10&end=22.11
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub comment: Option<ArcStr>,
  /// The optional `revision`  attribute defines a revision number for your library.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=30.17&end=30.18
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub revision: Option<ArcStr>,
  /// Used in TSMC PDK
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub simulation: Option<bool>,
  /// The `nom_process`  attribute defines process scaling,
  /// one of the nominal operating conditions for a library.
  ///
  /// A floating-point number that represents the degree of process scaling in the cells of the library.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=28.3+28.10&end=28.4+28.11
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub nom_process: Option<NotNan<f64>>,
  /// The `nom_temperature`  attribute defines the temperature (in centigrade),
  /// one of the nominal operating conditions for a library.
  ///
  /// A floating-point number that represents the temperature of the cells in the library
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=28.15&end=28.22
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub nom_temperature: Option<NotNan<f64>>,
  /// The `nom_voltage`  attribute defines voltage, one of the nominal operating conditions for a library.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=28.26&end=28.27
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub nom_voltage: Option<NotNan<f64>>,
  /// Use this group to define operating conditions;
  /// that is, `process`, `voltage`, and `temperature`.
  /// You define an `operating_conditions`  group at the library-level, as shown here:
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=72.3&end=72.4
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<OperatingConditions>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<OperatingConditions>::deserialize_with")]
  pub operating_conditions: GroupSet<OperatingConditions>,
  /// Default operating conditions for the library
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.29+34.32&end=34.31+34.33
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub default_operating_conditions: Option<ArcStr>,
  /// The optional `default_threshold_voltage_group`  attribute specifies a cell’s category based on its threshold voltage characteristics
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=23.20&end=23.21
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub default_threshold_voltage_group: Option<ArcStr>,
  /// Use this attribute to define new, temporary, or user-defined attributes
  /// for use in symbol and technology libraries.
  /// You can use either a space or a comma to separate the arguments.
  /// The following example shows how to define a new string attribute called `bork`,
  /// which is valid in a `pin`  group:
  ///
  /// Example
  /// ``` liberty
  /// define ("bork", "pin", "string") ;
  /// ```
  /// You give the new library attribute a value by using the simple attribute syntax:
  /// ``` liberty
  /// bork : "nimo" ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.5&end=36.21
  /// ">Reference</a>
  #[size = 64]
  #[liberty(complex(type = Set))]
  #[serde(serialize_with = "GroupSet::<Define>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Define>::deserialize_with")]
  pub define: GroupSet<Define>,
  /// Use this special attribute to define new, temporary, or user-defined groups
  /// for use in technology libraries.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.24&end=37.25
  /// ">Reference</a>
  #[size = 64]
  #[liberty(complex(type = Set))]
  #[serde(serialize_with = "GroupSet::<DefineGroup>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<DefineGroup>::deserialize_with")]
  pub define_group: GroupSet<DefineGroup>,
  /// The `define_cell_area`  attribute defines the area resources a `cell` uses,
  /// such as the number of pad slots.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.23&end=36.24
  /// ">Reference</a>
  #[size = 64]
  #[liberty(complex(type = Set))]
  #[serde(serialize_with = "GroupSet::<DefineCellArea>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<DefineCellArea>::deserialize_with")]
  pub define_cell_area: GroupSet<DefineCellArea>,
  /// ``` liberty
  /// library_features (value_1, value_2, ..., value_n) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=18.40&end=18.41
  /// ">Reference</a>
  #[size = 24]
  #[liberty(complex)]
  pub library_features: Vec<ArcStr>,
  /// Used in TSMC library
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_leakage_power_density: Option<NotNan<f64>>,
  /// Default leakage power
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.4&end=34.5
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_cell_leakage_power: Option<NotNan<f64>>,
  /// Default connection class
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.7&end=34.8
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub default_connection_class: Option<ArcStr>,
  /// Fanout load of input pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.10&end=34.11
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_fanout_load: Option<NotNan<f64>>,
  /// Capacitance of inout pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.13&end=34.14
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_inout_pin_cap: Option<NotNan<f64>>,
  /// Capacitance of input pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.16&end=34.17
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_input_pin_cap: Option<NotNan<f64>>,
  /// Maximum capacitance of output pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.19&end=34.21
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_max_capacitance: Option<NotNan<f64>>,
  /// Maximum fanout of all output pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.23&end=34.24
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_max_fanout: Option<NotNan<f64>>,
  /// Maximum transition of output pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.26&end=34.27
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_max_transition: Option<NotNan<f64>>,
  /// Capacitance of output pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.33&end=34.34
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_output_pin_cap: Option<NotNan<f64>>,
  /// Wire load area
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.37&end=34.37
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_wire_load_area: Option<NotNan<f64>>,
  /// Wire load capacitance
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.38&end=34.39
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_wire_load_capacitance: Option<NotNan<f64>>,
  /// Wire load mode
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.41&end=34.41
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub default_wire_load_mode: Option<ArcStr>,
  /// Wire load resistance
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.42&end=34.43
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub default_wire_load_resistance: Option<NotNan<f64>>,
  /// Wire load selection
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.45&end=34.45
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub default_wire_load_selection: Option<ArcStr>,
  /// The `em_temp_degradation_factor` attribute specifies the electromigration exponential
  /// degradation factor.
  ///
  /// Syntax:
  /// `em_temp_degradation_factor : valuefloat ;`
  ///
  /// value:
  /// A floating-point number in centigrade units consistent with other temperature specifications throughout the library.
  ///
  /// Example
  /// `em_temp_degradation_factor : 40.0 ;`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=26.3&end=26.13
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub em_temp_degradation_factor: Option<NotNan<f64>>,
  /// Valid values are 1ps, 10ps, 100ps, and 1ns. The default is 1ns.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=42.25&end=42.30
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple)]
  pub time_unit: units::TimeUnit,
  /// This attribute specifies the unit for all capacitance
  /// values within the logic library, including
  /// default capacitances, max_fanout capacitances,
  /// pin capacitances, and wire capacitances.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.7&end=44.19
  /// ">Reference</a>
  #[size = 16]
  #[liberty(complex(type = Option))]
  pub capacitive_load_unit: Option<units::CapacitiveLoadUnit>,
  /// Valid values are 1mV, 10mV, 100mV, and 1V. The default is 1V.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.2&end=43.9
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple)]
  pub voltage_unit: units::VoltageUnit,
  /// The valid values are 1uA, 10uA, 100uA, 1mA, 10mA, 100mA, and 1A.
  /// **No default exists for the `current_unit` attribute if the attribute is omitted.**
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.12&end=43.24
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub current_unit: Option<units::CurrentUnit>,
  /// Valid unit values are 1ohm, 10ohm, 100ohm, and 1kohm.
  /// **No default exists for `pulling_resistance_unit` if the attribute is omitted.**
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.25&end=44.4
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub pulling_resistance_unit: Option<units::PullingResistanceUnit>,
  /// This attribute indicates the units of the power values
  /// in the library. If this attribute is missing, the
  /// leakage-power values are expressed without units.
  ///
  /// Valid values are 1W, 100mW, 10mW, 1mW, 100nW, 10nW, 1nW, 100pW, 10pW, and 1pW.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.22&end=44.31
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub leakage_power_unit: Option<units::LeakagePowerUnit>,
  /// Use the `voltage_map`  attribute to associate a voltage name
  /// with relative voltage values referenced by the cell-level `pg_pin`  groups
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=39.15&end=39.16
  /// ">Reference</a>
  #[size = 64]
  #[liberty(complex(type = Set))]
  #[serde(serialize_with = "GroupSet::<VoltageMap>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<VoltageMap>::deserialize_with")]
  pub voltage_map: GroupSet<VoltageMap>,
  /// An `input_voltage`  group is defined in the library  group to designate
  /// a set of input voltage ranges for your cells.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=61.32&end=61.33
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<InputVoltage>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<InputVoltage>::deserialize_with")]
  pub input_voltage: GroupSet<InputVoltage>,
  /// You define an `output_voltage` group in the `library` group to designate a set of output
  /// voltage level ranges to drive output cells.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=75.22&end=75.23
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<OutputVoltage>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<OutputVoltage>::deserialize_with")]
  pub output_voltage: GroupSet<OutputVoltage>,
  /// Use the `slew_upper_threshold_pct_rise`  attribute to set the value of the upper threshold point
  /// that is used to model the delay of a pin rising from 0 to 1.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the upper threshold point
  /// used to model the delay of a pin rising from 0 to 1. The default is 80.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=32.16+32.24&end=32.18+32.26
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  #[default = "unsafe { NotNan::<f64>::new_unchecked(80.0) }"]
  pub slew_upper_threshold_pct_rise: NotNan<f64>,
  /// Use the `slew_lower_threshold_pct_rise`  attribute to set the default lower threshold point
  /// that is used to model the delay of a pin rising from 0 to 1.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the lower threshold point
  /// used to model the delay of a pin rising from 0 to 1. The default is 20.0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=31.20+31.28&end=31.22+31.29
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  #[default = "unsafe { NotNan::<f64>::new_unchecked(20.0) }"]
  pub slew_lower_threshold_pct_rise: NotNan<f64>,
  /// Use the `slew_derate_from_library`  attribute to specify how the transition times need to be derated to match the transition times between the characterization trip points
  ///
  /// A floating-point number between 0.0 and 1.0. The default is 1.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=30.25+31.3&end=30.26+31.4
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  #[default = "unsafe { NotNan::<f64>::new_unchecked(1.0) }"]
  pub slew_derate_from_library: NotNan<f64>,
  /// Use the `slew_lower_threshold_pct_fall`  attribute to set the default lower threshold point
  /// that is used to model the delay of a pin falling from 1 to 0.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the lower threshold point
  /// used to model the delay of a pin falling from 1 to 0. The default is 20.0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=31.7+31.15&end=31.9+31.16
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  #[default = "unsafe { NotNan::<f64>::new_unchecked(20.0) }"]
  pub slew_lower_threshold_pct_fall: NotNan<f64>,
  /// Use the `slew_upper_threshold_pct_fall`  attribute to set the default upper threshold point
  /// that is used to model the delay of a pin falling from 1 to 0.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the upper threshold point
  /// to model the delay of a pin falling from 1 to 0. The default is 80.0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=32.3+32.11&end=32.5+32.12
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  #[default = "unsafe { NotNan::<f64>::new_unchecked(80.0) }"]
  pub slew_upper_threshold_pct_fall: NotNan<f64>,
  /// Use the `input_threshold_pct_fall`  attribute to set the default threshold point
  /// on an input pin signal falling from 1 to 0.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the threshold point
  /// of an input pin signal falling from 1 to 0. The default is 50.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=26.15+26.23&end=26.17+26.24
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  #[default = "unsafe { NotNan::<f64>::new_unchecked(50.0) }"]
  pub input_threshold_pct_fall: NotNan<f64>,
  /// Use the `input_threshold_pct_rise`  attribute to set the default threshold point
  /// on an input pin signal rising from 0 to 1.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the threshold point
  /// of an input pin signal rising from 0 to 1. The default is 50.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=26.28+27.3&end=26.30+27.4
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  #[default = "unsafe { NotNan::<f64>::new_unchecked(50.0) }"]
  pub input_threshold_pct_rise: NotNan<f64>,
  /// Use the `output_threshold_pct_rise`  attribute to set the value
  /// of the threshold point on an output pin signal rising from 0 to 1.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the threshold point
  /// of an output pin signal rising from 0 to 1.The default is 50.0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=29.17+29.24&end=29.18+29.25
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  #[default = "unsafe { NotNan::<f64>::new_unchecked(50.0) }"]
  pub output_threshold_pct_rise: NotNan<f64>,
  /// Use the `output_threshold_pct_fall`  attribute to set the value of the threshold point
  /// on an output pin signal falling from 1 to 0.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the threshold point
  /// of an output pin signal falling from 1 to 0. The default is 50.0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=29.5+29.12&end=29.6+29.13
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  #[default = "unsafe { NotNan::<f64>::new_unchecked(50.0) }"]
  pub output_threshold_pct_fall: NotNan<f64>,
  /// The `is_soi`  attribute specifies that the cell is a
  /// silicon-on-insulator (SOI) cell.
  /// The default is false, which means that the cell is a
  /// bulk-CMOS cell.
  ///
  /// If the `is_soi`  attribute is specified at both the
  /// library and cell levels,
  /// the cell-level value overrides the library-level value
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=114.25&end=114.28
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_soi: Option<bool>,
  /// The `soft_error_rate_confidence`  attribute specifies the confidence level
  /// at which the cell soft error rate is sampled in the library. The value range is from 0 to 1.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=32.30&end=32.31
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub soft_error_rate_confidence: Option<NotNan<f64>>,
  /// Use the `output_current_template`  group to describe a table template
  /// for composite current source (CCS) modeling.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=74.15&end=74.16
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<TableTemple>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<TableTemple>::deserialize_with")]
  pub output_current_template: GroupSet<TableTemple>,
  /// The `power_lut_template` group is defined within the `library` group, as shown here:
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=83.34&end=83.35
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<TableTemple>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<TableTemple>::deserialize_with")]
  pub power_lut_template: GroupSet<TableTemple>,
  /// Use the `lu_table_template`  group to define templates of common information
  /// to use in lookup tables. Define the `lu_table_template`  group at the library level
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=65.5&end=65.6
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<TableTemple>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<TableTemple>::deserialize_with")]
  pub lu_table_template: GroupSet<TableTemple>,
  /// The `base_curves`  group is a library-level group that contains
  /// the detailed description of normalized base curves.
  ///
  /// **Syntax**
  /// ``` text
  /// library (my_compact_ccs_lib) {
  ///   …
  ///   base_curves (base_curves_name) {
  ///     …
  ///   }
  /// }
  /// ```
  /// **Example**
  /// ``` text
  /// library(my_lib) {
  ///   …
  ///   base_curves (ctbct1) {
  ///     …
  ///   }
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=39.32+40.2&end=39.33+40.15
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<BaseCurves>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<BaseCurves>::deserialize_with")]
  pub base_curves: GroupSet<BaseCurves>,
  /// The `compact_lut_template`  group is a lookup table template used for compact CCS timing and power modeling
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=41.20&end=41.21
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<CompactLutTemplate>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<CompactLutTemplate>::deserialize_with")]
  pub compact_lut_template: GroupSet<CompactLutTemplate>,
  /// The library-level `normalized_driver_waveform`  group represents a collection
  /// of driver waveforms under various input slew values.
  /// The `index_1`  specifies the input slew and `index_2`  specifies the normalized voltage.
  /// Note that the slew index in the `normalized_driver_waveform`  table is
  /// based on the slew derate and slew trip points of the library (global values).
  /// When applied on a pin or cell with different slew or slew derate,
  /// the new slew should be interpreted from the waveform.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=70.28&end=70.33
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<DriverWaveform>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<DriverWaveform>::deserialize_with")]
  pub normalized_driver_waveform: GroupSet<DriverWaveform>,
  /// A `wire_load`  group is defined in a `library`  group, as follows.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.16&end=94.17
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<WireLoad>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<WireLoad>::deserialize_with")]
  pub wire_load: GroupSet<WireLoad>,
  /// A `wire_load_selection`  group is defined in a `library`  group, as follows.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.16&end=94.17
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<WireLoadSection>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<WireLoadSection>::deserialize_with")]
  pub wire_load_selection: GroupSet<WireLoadSection>,
  /// Wire load
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.36&end=34.36
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub default_wire_load: Option<ArcStr>,
  /// Used in TSMC library
  /// valid: `match_footprint`?
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub in_place_swap_mode: Option<ArcStr>,
  /// You can define one or more `fpga_isd`  groups at the library level
  /// to specify the drive current, I/O voltages, and slew rates for FPGA parts and cells
  ///
  /// When you specify more than one `fpga_isd`  group, you **must** also define
  /// the library-level `default_fpga_isd`  attribute to specify which `fpga_isd`
  /// group is the default
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=63.22+63.25&end=63.23+63.27
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<FpgaIsd>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<FpgaIsd>::deserialize_with")]
  pub fpga_isd: GroupSet<FpgaIsd>,
  /// When you specify more than one `fpga_isd`  group, you **must** also define
  /// the library-level `default_fpga_isd`  attribute to specify which `fpga_isd`
  /// group is the default
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=63.22+63.25&end=63.23+63.27
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub default_fpga_isd: Option<ArcStr>,
  /// The `sensitization` group defined at the library level describes
  /// the complete state patterns for a specific list of pins (defined by the `pin_names` attribute)
  /// that are referenced and instantiated as stimuli in the timing arc.
  ///
  /// Vector attributes in the group define all possible pin states used as stimuli.
  /// Actual stimulus waveforms can be described by a combination of these vectors.
  /// Multiple sensitization groups are allowed in a library. Each `sensitization` group
  /// can be referenced by multiple cells, and each cell can make reference to
  /// multiple `sensitization`  groups.
  ///
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=88.10&end=88.16
  /// ">Reference</a>
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Sensitization>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Sensitization>::deserialize_with")]
  pub sensitization: GroupSet<Sensitization>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Cell>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Cell>::deserialize_with")]
  pub cell: GroupSet<Cell>,
}

impl GroupFn for Library {}

impl fmt::Display for Library {
  /// Format [Library] struct as `.lib` file, see more at [examples](https://github.com/zao111222333/liberty-db/tree/master/examples)
  /// ```
  /// use liberty_db::library::Library;
  /// use std::{
  /// fs::{self, File},
  /// io::{BufWriter, Write},
  /// path::Path};
  /// let library  = Library::default();
  /// let mut writer = BufWriter::new(File::create(Path::new("out.lib")).unwrap());
  /// write!(&mut writer, "{}", library).unwrap();
  /// ```
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    self.fmt_lib::<DefaultIndentation>(f)
  }
}
use crate::ast::{parser, GroupAttri, ParserError};
impl Library {
  const KEY: &'static str = "library";
  /// Parse `.lib` file as a [Library] struct.
  #[expect(clippy::arithmetic_side_effects)]
  #[inline]
  pub fn parse_lib(i: &str) -> Result<Self, ParserError> {
    let mut scope = ParseScope::default();
    let input1 = match parser::comment_space_newline(i) {
      Ok((input1, n)) => {
        scope.line_num += n;
        input1
      }
      Err(e) => return Err(ParserError::nom(0, e)),
    };
    let (input2, key) = match parser::key::<nom::error::Error<&str>>(input1) {
      Ok(res) => res,
      Err(e) => return Err(ParserError::nom(scope.line_num, e)),
    };
    if key == Self::KEY {
      match <Self as GroupAttri>::nom_parse(input2, Self::KEY, &mut scope) {
        Err(e) => Err(ParserError::nom(scope.line_num, e)),
        Ok((_, Err(e))) => Err(ParserError::IdError(scope.line_num, e)),
        Ok((_, Ok(builder))) => {
          let mut builder_scope = BuilderScope::default();
          Ok(ParsingBuilder::build(builder, &mut builder_scope))
        }
      }
    } else {
      Err(ParserError::Other(
        scope.line_num,
        format!("Need key={}, find={key}", Self::KEY),
      ))
    }
  }
  #[inline]
  pub fn fmt_lib<I: crate::ast::Indentation>(
    &self,
    f: &mut fmt::Formatter<'_>,
  ) -> Result<(), fmt::Error> {
    let ff = &mut crate::ast::CodeFormatter::<'_, fmt::Formatter<'_>, I>::new(f);
    crate::ast::fmt_library_beginning(self.comments_this(), ff)?;
    GroupAttri::fmt_liberty(self, Self::KEY, ff)?;
    f.write_char('\n')
  }
  /// TODO: Parse `.json` file as a [Library] struct.
  #[inline]
  pub fn parse_json(_i: &str) -> Result<Self, ParserError> {
    todo!()
  }
  /// TODO: Format [Library] to .json
  #[inline]
  pub fn fmt_json<I: crate::ast::Indentation>(
    &self,
    _f: &mut fmt::Formatter<'_>,
  ) -> Result<(), fmt::Error> {
    todo!()
  }
  /// TODO: Parse `.db` file as a [Library] struct.
  #[inline]
  pub fn parse_db(_i: &str) -> Result<Self, ParserError> {
    todo!()
  }
  /// TODO: Format [Library] to .db
  #[inline]
  pub fn fmt_db(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    todo!()
  }
}
