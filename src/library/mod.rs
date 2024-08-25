//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

mod items;
use crate::{
  ast::{AttributeList, DefaultIndentation, GroupComments, GroupFn},
  cell::Cell,
  common::table::{DriverWaveform, TableTemple},
  units, ArcStr, GroupSet,
};
use core::fmt;
pub use items::*;

/// The first line of the library group statement names the library.
/// It is the first executable line in your library.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=20.24&end=20.26
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone);
        derive(derivative::Derivative);
        derivative(Default);),
  attr_filter(derivative;)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Library {
  /// library name
  #[id]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  /// The `technology`  attribute statement specifies the technology
  /// family being used in the library.
  /// When you define the technology  attribute,
  /// it must be the first attribute you use and it must be placed at the top of the listing.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=39.3&end=39.5
  /// ">Reference</a>
  #[liberty(complex)]
  pub technology: ArcStr,
  /// Use the `delay_model`  attribute to specify which delay model
  /// to use in the delay calculations.
  /// The `delay_model`  attribute must be the first attribute in the library
  /// if a technology attribute is not present.
  /// Otherwise, it should follow the technology attribute.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=24.3&end=24.6
  /// ">Reference</a>
  #[liberty(simple)]
  pub delay_model: DelayModel,
  /// You can use any format within the quotation marks to report the date
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=23.5&end=23.5
  /// ">Reference</a>
  #[liberty(simple)]
  pub date: ArcStr,
  /// You use the `comment`  attribute to include copyright
  /// or other product information in the library report. You can include only one comment line in a library
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=22.10&end=22.11
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub comment: Option<ArcStr>,
  /// The optional `revision`  attribute defines a revision number for your library.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=30.17&end=30.18
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub revision: Option<ArcStr>,
  /// Used in TSMC PDK
  #[liberty(simple(type = Option))]
  pub simulation: Option<bool>,
  /// The `nom_process`  attribute defines process scaling,
  /// one of the nominal operating conditions for a library.
  ///
  /// A floating-point number that represents the degree of process scaling in the cells of the library.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=28.3+28.10&end=28.4+28.11
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub nom_process: Option<f64>,
  /// The `nom_temperature`  attribute defines the temperature (in centigrade),
  /// one of the nominal operating conditions for a library.
  ///
  /// A floating-point number that represents the temperature of the cells in the library
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=28.15&end=28.22
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub nom_temperature: Option<f64>,
  /// The `nom_voltage`  attribute defines voltage, one of the nominal operating conditions for a library.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=28.26&end=28.27
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub nom_voltage: Option<f64>,
  /// Use this group to define operating conditions;
  /// that is, `process`, `voltage`, and `temperature`.
  /// You define an `operating_conditions`  group at the library-level, as shown here:
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=72.3&end=72.4
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub operating_conditions: GroupSet<OperatingConditions>,
  /// Default operating conditions for the library
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.29+34.32&end=34.31+34.33
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_operating_conditions: Option<ArcStr>,
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
  #[liberty(complex(type = Set))]
  pub define: GroupSet<Define>,
  /// The `define_cell_area`  attribute defines the area resources a `cell` uses,
  /// such as the number of pad slots.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.23&end=36.24
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub define_cell_area: GroupSet<DefineCellArea>,
  /// Use this special attribute to define new, temporary, or user-defined groups
  /// for use in technology libraries.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.24&end=37.25
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub define_group: GroupSet<DefineGroup>,
  /// ``` liberty
  /// library_features (value_1, value_2, ..., value_n) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=18.40&end=18.41
  /// ">Reference</a>
  #[liberty(complex)]
  pub library_features: Vec<ArcStr>,
  /// Used in TSMC library
  #[liberty(simple(type = Option))]
  pub default_leakage_power_density: Option<f64>,
  /// Default leakage power
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.4&end=34.5
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_cell_leakage_power: Option<f64>,
  /// Default connection class
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.7&end=34.8
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_connection_class: Option<ArcStr>,
  /// Fanout load of input pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.10&end=34.11
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_fanout_load: Option<f64>,
  /// Capacitance of inout pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.13&end=34.14
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_inout_pin_cap: Option<f64>,
  /// Capacitance of input pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.16&end=34.17
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_input_pin_cap: Option<f64>,
  /// Maximum capacitance of output pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.19&end=34.21
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_max_capacitance: Option<f64>,
  /// Maximum fanout of all output pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.23&end=34.24
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_max_fanout: Option<f64>,
  /// Maximum transition of output pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.26&end=34.27
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_max_transition: Option<f64>,
  /// Capacitance of output pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.33&end=34.34
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_output_pin_cap: Option<f64>,
  /// Wire load area
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.37&end=34.37
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_wire_load_area: Option<f64>,
  /// Wire load capacitance
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.38&end=34.39
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_wire_load_capacitance: Option<f64>,
  /// Wire load mode
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.41&end=34.41
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_wire_load_mode: Option<ArcStr>,
  /// Wire load resistance
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.42&end=34.43
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_wire_load_resistance: Option<f64>,
  /// Wire load selection
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.45&end=34.45
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_wire_load_selection: Option<ArcStr>,
  /// Valid values are 1ps, 10ps, 100ps, and 1ns. The default is 1ns.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=42.25&end=42.30
  /// ">Reference</a>
  #[liberty(simple)]
  pub time_unit: units::TimeUnit,
  /// This attribute specifies the unit for all capacitance
  /// values within the logic library, including
  /// default capacitances, max_fanout capacitances,
  /// pin capacitances, and wire capacitances.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.7&end=44.19
  /// ">Reference</a>
  #[liberty(complex(type = Option))]
  pub capacitive_load_unit: Option<units::CapacitiveLoadUnit>,
  /// Valid values are 1mV, 10mV, 100mV, and 1V. The default is 1V.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.2&end=43.9
  /// ">Reference</a>
  #[liberty(simple)]
  pub voltage_unit: units::VoltageUnit,
  /// The valid values are 1uA, 10uA, 100uA, 1mA, 10mA, 100mA, and 1A.
  /// **No default exists for the `current_unit` attribute if the attribute is omitted.**
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.12&end=43.24
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub current_unit: Option<units::CurrentUnit>,
  /// Valid unit values are 1ohm, 10ohm, 100ohm, and 1kohm.
  /// **No default exists for `pulling_resistance_unit` if the attribute is omitted.**
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.25&end=44.4
  /// ">Reference</a>
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
  #[liberty(simple)]
  pub leakage_power_unit: units::LeakagePowerUnit,
  /// Use the `voltage_map`  attribute to associate a voltage name
  /// with relative voltage values referenced by the cell-level `pg_pin`  groups
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=39.15&end=39.16
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub voltage_map: GroupSet<VoltageMap>,
  /// An `input_voltage`  group is defined in the library  group to designate
  /// a set of input voltage ranges for your cells.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=61.32&end=61.33
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub input_voltage: GroupSet<InputVoltage>,
  /// You define an `output_voltage` group in the `library` group to designate a set of output
  /// voltage level ranges to drive output cells.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=75.22&end=75.23
  /// ">Reference</a>
  #[liberty(group(type = Set))]
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
  #[liberty(simple)]
  #[derivative(Default(value = "80.0"))]
  pub slew_upper_threshold_pct_rise: f64,
  /// Use the `slew_lower_threshold_pct_rise`  attribute to set the default lower threshold point
  /// that is used to model the delay of a pin rising from 0 to 1.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the lower threshold point
  /// used to model the delay of a pin rising from 0 to 1. The default is 20.0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=31.20+31.28&end=31.22+31.29
  /// ">Reference</a>
  #[liberty(simple)]
  #[derivative(Default(value = "20.00"))]
  pub slew_lower_threshold_pct_rise: f64,
  /// Use the `slew_derate_from_library`  attribute to specify how the transition times need to be derated to match the transition times between the characterization trip points
  ///
  /// A floating-point number between 0.0 and 1.0. The default is 1.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=30.25+31.3&end=30.26+31.4
  /// ">Reference</a>
  #[liberty(simple)]
  #[derivative(Default(value = "1.0"))]
  pub slew_derate_from_library: f64,
  /// Use the `slew_lower_threshold_pct_fall`  attribute to set the default lower threshold point
  /// that is used to model the delay of a pin falling from 1 to 0.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the lower threshold point
  /// used to model the delay of a pin falling from 1 to 0. The default is 20.0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=31.7+31.15&end=31.9+31.16
  /// ">Reference</a>
  #[liberty(simple)]
  #[derivative(Default(value = "20.00"))]
  pub slew_lower_threshold_pct_fall: f64,
  /// Use the `slew_upper_threshold_pct_fall`  attribute to set the default upper threshold point
  /// that is used to model the delay of a pin falling from 1 to 0.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the upper threshold point
  /// to model the delay of a pin falling from 1 to 0. The default is 80.0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=32.3+32.11&end=32.5+32.12
  /// ">Reference</a>
  #[liberty(simple)]
  #[derivative(Default(value = "80.00"))]
  pub slew_upper_threshold_pct_fall: f64,
  /// Use the `input_threshold_pct_fall`  attribute to set the default threshold point
  /// on an input pin signal falling from 1 to 0.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the threshold point
  /// of an input pin signal falling from 1 to 0. The default is 50.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=26.15+26.23&end=26.17+26.24
  /// ">Reference</a>
  #[liberty(simple)]
  #[derivative(Default(value = "50.00"))]
  pub input_threshold_pct_fall: f64,
  /// Use the `input_threshold_pct_rise`  attribute to set the default threshold point
  /// on an input pin signal rising from 0 to 1.
  /// You can specify this attribute at the pin-level to override the default.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the threshold point
  /// of an input pin signal rising from 0 to 1. The default is 50.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=26.28+27.3&end=26.30+27.4
  /// ">Reference</a>
  #[liberty(simple)]
  #[derivative(Default(value = "50.00"))]
  pub input_threshold_pct_rise: f64,
  /// Use the `output_threshold_pct_rise`  attribute to set the value
  /// of the threshold point on an output pin signal rising from 0 to 1.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the threshold point
  /// of an output pin signal rising from 0 to 1.The default is 50.0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=29.17+29.24&end=29.18+29.25
  /// ">Reference</a>
  #[liberty(simple)]
  #[derivative(Default(value = "50.00"))]
  pub output_threshold_pct_rise: f64,
  /// Use the `output_threshold_pct_fall`  attribute to set the value of the threshold point
  /// on an output pin signal falling from 1 to 0.
  ///
  /// A floating-point number between 0.0 and 100.0 that specifies the threshold point
  /// of an output pin signal falling from 1 to 0. The default is 50.0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=29.5+29.12&end=29.6+29.13
  /// ">Reference</a>
  #[liberty(simple)]
  #[derivative(Default(value = "50.00"))]
  pub output_threshold_pct_fall: f64,
  /// The `soft_error_rate_confidence`  attribute specifies the confidence level
  /// at which the cell soft error rate is sampled in the library. The value range is from 0 to 1.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=32.30&end=32.31
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub soft_error_rate_confidence: Option<f64>,
  /// Use the `output_current_template`  group to describe a table template
  /// for composite current source (CCS) modeling.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=74.15&end=74.16
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub output_current_template: GroupSet<TableTemple>,
  /// The `power_lut_template` group is defined within the `library` group, as shown here:
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=83.34&end=83.35
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub power_lut_template: GroupSet<TableTemple>,
  /// Use the `lu_table_template`  group to define templates of common information
  /// to use in lookup tables. Define the `lu_table_template`  group at the library level
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=65.5&end=65.6
  /// ">Reference</a>
  #[liberty(group(type = Set))]
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
  #[liberty(group(type = Set))]
  pub base_curves: GroupSet<BaseCurves>,
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
  #[liberty(group(type = Set))]
  pub normalized_driver_waveform: GroupSet<DriverWaveform>,
  /// A `wire_load`  group is defined in a `library`  group, as follows.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.16&end=94.17
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub wire_load: GroupSet<WireLoad>,
  /// A `wire_load_selection`  group is defined in a `library`  group, as follows.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.16&end=94.17
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub wire_load_selection: GroupSet<WireLoadSection>,
  /// Wire load
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=34.36&end=34.36
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub default_wire_load: Option<ArcStr>,
  /// Used in TSMC library
  /// valid: `match_footprint`?
  #[liberty(simple(type = Option))]
  pub in_place_swap_mode: Option<ArcStr>,
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
  #[liberty(group(type = Set))]
  pub sensitization: GroupSet<Sensitization>,
  #[liberty(group(type = Set))]
  pub cell: GroupSet<Cell>,
}

impl GroupFn for Library {}

impl fmt::Display for Library {
  /// Demo
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
  /// Format [Library] struct as `.lib` file
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    self.fmt_lib::<DefaultIndentation>(f)
  }
}
use crate::ast::{parser, GroupAttri, ParserError};
impl Library {
  /// Parse `.lib` file as a [Library] struct.
  #[allow(clippy::arithmetic_side_effects)]
  #[inline]
  pub fn parse_lib(i: &str) -> Result<Self, ParserError<'_>> {
    let mut line_num = 0;
    let input1 = match parser::comment_space_newline(i) {
      Ok((input1, n)) => {
        line_num += n;
        input1
      }
      Err(e) => return Err(ParserError::NomError(line_num, e)),
    };
    let (input2, key) = match parser::key::<nom::error::Error<&str>>(input1) {
      Ok(res) => res,
      Err(e) => return Err(ParserError::NomError(line_num, e)),
    };
    if key == "library" {
      match <Self as GroupAttri>::nom_parse(input2, &mut line_num) {
        Err(e) => Err(ParserError::NomError(line_num, e)),
        Ok((_, Err(e))) => Err(ParserError::IdError(line_num, e)),
        Ok((_, Ok(l))) => Ok(l),
      }
    } else {
      Err(ParserError::Other(line_num, format!("Need key=library, find={key}")))
    }
  }
  #[inline]
  pub fn fmt_lib<I: crate::ast::Indentation>(
    &self,
    f: &mut fmt::Formatter<'_>,
  ) -> Result<(), fmt::Error> {
    let ff = &mut crate::ast::CodeFormatter::<'_, fmt::Formatter<'_>, I>::new(f);
    crate::ast::fmt_first_line_comment(&self.comments.this, ff)?;
    self.fmt_liberty("library", ff)?;
    writeln!(f)
  }
  /// TODO: Parse `.json` file as a [Library] struct.
  #[inline]
  pub fn parse_json(_i: &str) -> Result<Self, ParserError<'_>> {
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
  pub fn parse_db(_i: &str) -> Result<Self, ParserError<'_>> {
    todo!()
  }
  /// TODO: Format [Library] to .db
  #[inline]
  pub fn fmt_db(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    todo!()
  }
}
