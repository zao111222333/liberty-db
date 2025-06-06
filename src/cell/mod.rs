//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
mod items;
pub use items::*;
#[cfg(test)]
mod test;
use alloc::collections::BTreeSet;
use core::{fmt::Debug, mem};

use crate::{
  Ctx,
  ast::{Attributes, BuilderScope, GroupComments, GroupFn, GroupSet},
  common::{char_config::CharConfig, items::NameList},
  expression::{FF, FFBank, Latch, LatchBank},
  pin::{AntennaDiodeType, Bundle, Bus, Pin},
  table::TableLookUp2D,
};

pub trait CellCtx {
  fn logic_variables(&self) -> &biodivine_lib_bdd::BddVariableSet;
  fn set_logic_variables(&mut self, variables: biodivine_lib_bdd::BddVariableSet);
  fn pg_variables(&self) -> &biodivine_lib_bdd::BddVariableSet;
  fn set_pg_variables(&mut self, variables: biodivine_lib_bdd::BddVariableSet);
}

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DefaultCellCtx {
  pub logic_variables: biodivine_lib_bdd::BddVariableSet,
  pub pg_variables: biodivine_lib_bdd::BddVariableSet,
}
impl CellCtx for DefaultCellCtx {
  #[inline]
  fn logic_variables(&self) -> &biodivine_lib_bdd::BddVariableSet {
    &self.logic_variables
  }
  #[inline]
  fn set_logic_variables(&mut self, variables: biodivine_lib_bdd::BddVariableSet) {
    self.logic_variables = variables;
  }
  #[inline]
  fn pg_variables(&self) -> &biodivine_lib_bdd::BddVariableSet {
    &self.pg_variables
  }
  #[inline]
  fn set_pg_variables(&mut self, variables: biodivine_lib_bdd::BddVariableSet) {
    self.pg_variables = variables;
  }
}
impl Debug for DefaultCellCtx {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("DefaultCellCtx")
      .field("logic_variables", &format_args!("{}", self.logic_variables))
      .field("pg_variables", &format_args!("{}", self.pg_variables))
      .finish()
  }
}
impl Default for DefaultCellCtx {
  #[inline]
  fn default() -> Self {
    Self {
      logic_variables: biodivine_lib_bdd::BddVariableSet::new(&[]),
      pg_variables: biodivine_lib_bdd::BddVariableSet::new(&[]),
    }
  }
}
#[derive(liberty_macros::Duplicate)]
#[duplicated(
  name = Model,
  docs(
    /// A model group can include all the attributes that are valid in a cell group, as well as the
    /// two additional attributes described in this section. For information about the cell group
    /// attributes, see Attributes and Values on page 99.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=225.23&end=225.29
    /// ">Reference</a>
  ),
  additional_attrs(
    /// The `cell_name`  attribute specifies the name of the `cell` within a `model`  group
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=225.31&end=225.31
    /// ">Reference-Definition</a>
    #[liberty(simple(type = Option))]
    pub cell_name: Option<String>,
    /// The short attribute lists the shorted ports that are connected together by a metal or poly
    /// trace. These ports are modeled within a model group.
    /// The most common example of a shorted port is a feedthrough, where an input port is
    /// directly connected to an output port and there is no active logic between these two ports.
    /// 
    /// Example 30 Using the short Attribute in a model Group
    /// ``` text
    /// model(cellA) {
    ///   area : 0.4;
    ///   ...
    ///   short(b, y);
    ///   short(c, y);
    ///   short(b, c);
    ///   ...
    ///   pin(y) {
    ///     direction : output;
    ///     timing() {
    ///       related_pin : a;
    ///       ...
    ///     }
    ///   }
    ///   pin(a) {
    ///     direction : input;
    ///     capacitance : 0.1;
    ///   }
    ///   pin(b) {
    ///     direction : input;
    ///     capacitance : 0.1;
    ///   }
    ///   pin(c) {
    ///     direction : input;
    ///     capacitance : 0.1;
    ///     clock : true;
    ///   }
    /// }
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=226.3&end=226.42
    /// ">Reference-Definition</a>
    #[liberty(complex(type = Vec))]
    pub short: Vec<Vec<String>>,
  )
)]
/// cell group
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=98.23&end=98.32
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Cell: serde::Serialize + serde::de::DeserializeOwned")]
pub struct Cell<C: Ctx> {
  #[id(borrow = str)]
  #[liberty(name)]
  pub name: String,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Cell,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(simple(type = Option))]
  pub area: Option<f64>,
  /// The `dont_use`  attribute with a true value indicates
  /// that a cell should not be added to a design
  /// during optimization
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=107.3&end=107.4
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub dont_use: Option<bool>,
  /// The `dont_touch`  attribute with a true
  /// value indicates that all instances of the cell must
  /// remain in the network.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=106.21&end=106.22
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub dont_touch: Option<bool>,
  /// CellId
  #[liberty(simple(type = Option))]
  pub single_bit_degenerate: Option<String>,
  #[liberty(simple(type = Option))]
  pub driver_waveform_rise: Option<String>,
  #[liberty(simple(type = Option))]
  pub driver_waveform_fall: Option<String>,
  /// The `always_on`  simple attribute models always-on cells or signal pins. Specify the attribute at the cell level to determine whether a cell is an always-on cell. Specify the attribute at the pin level to determine whether a pin is an always-on signal pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=100.73&end=100.75
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub always_on: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.4
  /// &end
  /// =228.4
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub antenna_diode_type: Option<AntennaDiodeType>,
  /// You can use the `clock_gating_integrated_cell` attribute to enter specific
  /// values that determine which integrated cell functionality the clock-gating tool uses.
  ///
  /// ### Syntax:
  /// ```text
  /// clock_gating_integrated_cell:generic|value_id;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=103.19&end=103.24
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub clock_gating_integrated_cell: Option<ClockGatingIntegratedCell>,
  #[liberty(simple(type = Option))]
  pub cell_footprint: Option<String>,
  #[liberty(simple(type = Option))]
  pub cell_leakage_power: Option<f64>,
  /// The `em_temp_degradation_factor` attribute specifies the electromigration
  /// exponential degradation factor
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=109.17&end=109.18
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub em_temp_degradation_factor: Option<f64>,
  /// interprets a combination timing arc between the clock pin and the output pin as a rising edge arc or as a falling edge arc
  ///
  /// Valid values are `rising_edge_clock_cell`  and `falling_edge_clock_cell`.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=109.29+109.36&end=109.30+109.37
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub fpga_cell_type: Option<FpgaCellType>,
  /// Use the `fpga_isd`  attribute to reference the drive,
  /// `io_type`, and `slew`  information contained in a library-level `fpga_isd`  group.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=110.3&end=110.4
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub fpga_isd: Option<String>,
  /// Indicates that the `timing` arcs are interpreted according
  ///  to interface `timing` specifications semantics.
  /// If this attribute is missing or its value is set to false,
  /// the `timing` relationships are interpreted as those of
  /// a regular cell rather than according to interface timing
  /// specification semantics.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=110.14&end=110.17
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub interface_timing: Option<bool>,
  /// Use the io_type  attribute to define the I/O standard used by this I/O cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=110.25&end=110.26
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub io_type: Option<String>,
  #[liberty(simple(type = Option))]
  pub is_filler_cell: Option<bool>,
  /// The `is_pad`  attribute identifies a pad pin on
  /// any I/O cell. You can also specify the `is_pad` attribute
  /// on PG pins.
  /// The valid values are true  and false.
  /// If the cell-level `pad_cell` attribute is specified on
  /// a I/O cell, the `is_pad`  attribute must be set to true
  /// in either a `pg_pin`  group or on a signal pin for that cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=111.5&end=111.8
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub is_pad: Option<bool>,
  /// The `is_pll_cell`  Boolean attribute identifies a phase-locked loop
  /// cell. A phase-locked loop (PLL) is a feedback control system
  /// that automatically adjusts the phase of a locally-generated signal
  /// to match the phase of an input signal.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=111.5&end=111.8
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub is_pll_cell: Option<bool>,
  /// The cell-level `is_clock_gating_cell` attribute specifies that a cell is for clock gating.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=113.8&end=113.9
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub is_clock_gating_cell: Option<bool>,
  /// The `is_clock_isolation_cell`  attribute identifies a cell as a clock-isolation cell.
  /// The default is false, meaning that the cell is a standard cell.
  /// For information about pin-level attributes of the clock-isolation cell,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=113.18&end=113.20
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub is_clock_isolation_cell: Option<bool>,
  /// The cell-level `is_isolation_cell`  attribute specifies that a
  /// cell is an isolation cell.
  /// The pin-level `isolation_cell_enable_pin`  attribute specifies
  /// the enable input pin for the isolation cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=113.27&end=113.29
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub is_isolation_cell: Option<bool>,
  /// The cell-level `is_level_shifter`  attribute specifies
  /// that a cell is a level shifter cell.
  /// The pin-level `level_shifter_enable_pin`  
  /// attribute specifies the enable input pin for
  /// the level shifter cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=114.5&end=114.7
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub is_level_shifter: Option<bool>,
  /// The `is_macro_cell`  simple Boolean attribute identifies
  /// whether a cell is a macro cell.
  /// If the attribute is set to true, the cell is a macro cell.
  /// If it is set to false, the cell is not a macro cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=114.15&end=114.17
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub is_macro_cell: Option<bool>,
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
  #[liberty(simple(type = Option))]
  pub is_soi: Option<bool>,
  /// The `level_shifter_type`  attribute specifies the
  /// voltage conversion type that is supported.
  /// Valid values are:
  ///
  /// + `LH`: Low to High
  /// + `HL`: High to Low
  /// + `HL_LH`: High to Low and Low to HighThe
  ///
  /// `level_shifter_type`  attribute is optional
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=115.9&end=115.17
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub level_shifter_type: Option<LevelShifterType>,
  /// The `retention_cell`  attribute identifies a retention cell. The `retention_cell_style` value is a random string
  ///
  /// ### Syntax
  /// ``` text
  /// retention_cell : retention_cell_style ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=118.21+118.24&end=118.22+118.25
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub retention_cell: Option<String>,
  /// The `switch_cell_type`  cell-level attribute specifies
  /// the type of the switch cell for direct inference.
  ///
  /// ### Syntax:
  /// ``` text
  /// switch_cell_type : coarse_grain | fine_grain;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=210.8&end=210.13
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub switch_cell_type: Option<SwitchCellType>,
  /// The `sensitization_master` attribute defines the sensitization group referenced by
  /// the cell to generate stimuli for characterization. The attribute is required if the cell
  /// contains sensitization information. Its string value should be any sensitization group name
  /// predefined in the current library.
  ///
  /// ### Syntax
  /// ``` text
  /// sensitization_master : sensitization_group_name;
  /// ```
  /// sensitization_group_name
  ///
  /// A string identifying the sensitization group name predefined in the current library.
  ///
  /// ### Example
  /// ``` text
  /// sensitization_master : sensi_2in_1out;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=118.28+119.2&end=118.31+119.7
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub sensitization_master: Option<String>,
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
  #[liberty(after_build = crate::table::use_common_template!)]
  pub dc_current: Option<TableLookUp2D<C>>,
  /// The `input_voltage_range`  attribute specifies the allowed
  /// voltage range of the level-shifter input pin and the voltage
  /// range for all input pins of the cell under all possible operating conditions
  /// (defined across multiple libraries).
  ///
  /// The attribute defines two floating values:
  ///  the first is the lower bound, and second is the upper bound.
  ///
  /// The `input_voltage_range`  syntax differs from the pin-level
  /// `input_signal_level_low` and `input_signal_level_high`  syntax in the following ways:
  ///
  /// + The `input_signal_level_low`  and `input_signal_level_high`  attributes are defined
  /// on the input pins under one operating condition.
  /// + The `input_signal_level_low`  and `input_signal_level_high`  attributes are used
  /// to specify the partial voltage swing of an input pin (that is, to prevent from
  /// swinging from ground rail VSS to full power rail VDD).
  /// Note that `input_voltage_range`  is not related to the voltage swing.
  ///
  /// Note:
  ///
  /// The `input_voltage_range`  and `output_voltage_range`  attributes
  /// should always be defined together.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=122.7&end=122.23
  /// ">Reference</a>
  #[liberty(complex(type = Option))]
  pub input_voltage_range: Option<(f64, f64)>,
  /// The `input_voltage_range`  and `output_voltage_range`  attributes
  /// should always be defined together.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=122.7&end=122.23
  /// ">Reference</a>
  #[liberty(complex(type = Option))]
  pub output_voltage_range: Option<(f64, f64)>,
  /// Use the pin_opposite attribute to describe functionally opposite (logically inverse) groups
  /// of input or output pins.
  /// ### Syntax
  ///
  /// ``` text
  /// pin_opposite ("name_list1", "name_list2") ;
  /// ```
  ///
  /// + `name_list1`: A name_list of output pins requires the supplied output values to be opposite.
  /// + `name_list2`: A name_list of input pins requires the supplied input values to be opposite.
  ///
  /// In the following example, pins IP and OP are logically inverse.
  /// ``` text
  /// pin_opposite ("IP", "OP") ;
  /// ```
  /// The pin_opposite attribute also incorporates the functionality of the `pin_equal` complex
  /// attribute.
  ///
  /// In the following example, Q1, Q2, and Q3 are equal; QB1 and QB2 are equal; and the pins
  /// in the first group are opposite of the pins in the second group.
  /// ``` text
  /// pin_opposite ("Q1 Q2 Q3", "QB1 QB2") ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=124.9&end=124.22
  /// ">Reference</a>
  #[liberty(complex(type = Option))]
  pub pin_opposite: Option<PinOpposite>,
  /// The `char_config` group is a group of attributes including simple and complex attributes.
  /// These attributes represent library characterization configuration, and specify the settings
  /// to characterize the library. Use the `char_config` group syntax to apply an attribute value
  /// to a specific characterization model. You can specify multiple complex attributes in the
  /// `char_config` group. You can also specify a single complex attribute multiple times for
  /// different characterization models.
  /// You can also define the `char_config` group within the cell, pin, and timing groups.
  /// However, when you specify the same attribute in multiple `char_config` groups at different
  /// levels, such as at the `library`, `cell`, `pin`, and `timing` levels, the attribute specified at the lower
  /// level gets priority over the ones specified at the higher levels. For example, the pin-level
  /// `char_config` group attributes have higher priority over the library-level `char_config`
  /// group attributes.
  ///
  /// ### Syntax
  /// ``` text
  /// library (library_name) {
  ///   char_config() {
  ///     /* characterization configuration attributes */
  ///   }
  ///   ...
  ///   cell (cell_name) {
  ///     char_config() {
  ///       /* characterization configuration attributes */
  ///     }
  ///     ...
  ///     pin(pin_name) {
  ///       char_config() {
  ///         /* characterization configuration attributes */
  ///       }
  ///       timing() {
  ///         char_config() {
  ///           /* characterization configuration attributes */
  ///         }
  ///       } /* end of timing */
  ///       ...
  ///     } /* end of pin */
  ///     ...
  ///   } /* end of cell */
  ///   ...
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=43.30+44.2&end=43.31+44.37
  /// ">Reference</a>
  #[liberty(group)]
  pub char_config: Option<CharConfig<C>>,
  #[liberty(group(type = Set))]
  pub pg_pin: GroupSet<PgPin<C>>,
  #[liberty(group(type = Set))]
  pub ff: GroupSet<FF<C>>,
  #[liberty(group(type = Set))]
  pub ff_bank: GroupSet<FFBank<C>>,
  #[liberty(group(type = Set))]
  pub latch: GroupSet<Latch<C>>,
  #[liberty(group(type = Set))]
  pub latch_bank: GroupSet<LatchBank<C>>,
  #[liberty(group(type = Set))]
  pub leakage_power: GroupSet<LeakagePower<C>>,
  #[liberty(group(type = Option))]
  pub statetable: Option<Statetable<C>>,
  /// Use the `dynamic_current` group to specify a current waveform vector when the power
  /// and ground current is dependent on the logical condition of a cell. A `dynamic_current`
  /// group is defined in a cell group, as shown here:
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=147.3&end=147.5
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub dynamic_current: GroupSet<DynamicCurrent<C>>,
  /// The `intrinsic_parasitic` group specifies the state-dependent intrinsic capacitance and
  /// intrinsic resistance of a `cell`.
  /// ### Syntax
  /// ``` text
  /// library( library_name ) {
  ///   ......
  ///   lu_table_template ( template_name ) {
  ///     variable_1 : pg_voltage | pg_voltage_difference;
  ///     index_1 ( "float, ..., float" );
  ///   }
  ///   cell (cell_name) {
  ///     mode_definition (mode_name) {
  ///       mode_value (mode_value) {
  ///         when : boolean_expression ;
  ///         sdf_cond : boolean_expression ;
  ///       }
  ///     }
  ///     ...
  ///     intrinsic_parasitic () {
  ///       mode (mode_name, mode_value) ;
  ///       when : boolean expression ;
  ///       intrinsic_resistance(pg_pin_name) {
  ///         related_output : output_pin_name ;
  ///         value : float ;
  ///         reference_pg_pin : pg_pin_name;
  ///         lut_values ( template_name ) {
  ///           index_1 ("float, ... float" );
  ///           values ("float, ... float" );
  ///         }
  ///       }
  ///       intrinsic_capacitance(pg_pin_name) {
  ///         value : float ;
  ///         reference_pg_pin : pg_pin_name;
  ///         lut_values ( template_name ) {
  ///           index_1 ("float, ... float" );
  ///           values ("float, ... float" );
  ///         }
  ///       }
  ///     }
  ///   }
  /// }
  /// ```
  /// Simple Attributes
  /// + when
  /// + reference_pg_pin
  ///
  /// Complex Attribute
  /// + mode
  ///
  /// Groups
  /// + intrinsic_capacitance
  /// + intrinsic_resistance
  /// + total_capacitance
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=176.24+177.2&end=176.49+177.25
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub intrinsic_parasitic: GroupSet<IntrinsicParasitic<C>>,
  /// A `leakage_current` group is defined within a cell group or a model group to specify
  /// leakage current values that are dependent on the state of the cell.
  ///
  /// ### Syntax
  /// ``` text
  /// library (name) {
  /// cell(cell_name) {
  ///   ...
  ///   leakage_current() {
  ///     when : boolean expression;
  ///     pg_current(pg_pin_name) {
  ///       value : float;
  ///     }
  ///     ...
  ///   }
  /// }
  /// ```
  /// Simple Attributes
  /// + when
  /// + value
  ///
  /// Complex Attribute
  /// + mode
  ///
  /// Group
  /// + pg_current
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=194.49+195.2&end=194.50+195.20
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub leakage_current: GroupSet<LeakageCurrent<C>>,
  #[liberty(group(type = Set))]
  pub pin: GroupSet<Pin<C>>,
  // TODO:
  #[liberty(group(type = Set))]
  pub bus: GroupSet<Bus<C>>,
  #[liberty(group(type = Vec))]
  /// The `test_cell`  group is in a `cell` group or `model` group.
  /// It models only the nontest behavior of a scan cell, which
  /// is described by an `ff`, `ff_bank`, `latch`, `latch_bank`  or `statetable`  statement
  /// and `pin` function attributes
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=218.9&end=218.11
  /// ">Reference</a>
  pub test_cell: Vec<TestCell<C>>,
  #[liberty(group(type = Set))]
  pub bundle: GroupSet<Bundle<C>>,
}
#[duplicate::duplicate_item(
  CellModel;
  [Cell];
  [Model];
)]
impl<C: Ctx> GroupFn<C> for CellModel<C> {
  #[inline]
  fn before_build(builder: &mut Self::Builder, scope: &mut BuilderScope<C>) {
    // update variable
    let ff_latch_nodes = builder
      .ff
      .iter()
      .flat_map(|ff| [ff.variable1.as_str(), ff.variable2.as_str()])
      .chain(
        builder
          .ff_bank
          .iter()
          .flat_map(|ff| [ff.variable1.as_str(), ff.variable2.as_str()]),
      )
      .chain(
        builder
          .latch
          .iter()
          .flat_map(|latch| [latch.variable1.as_str(), latch.variable2.as_str()]),
      )
      .chain(
        builder
          .latch_bank
          .iter()
          .flat_map(|latch| [latch.variable1.as_str(), latch.variable2.as_str()]),
      );
    let mut logic_variables: BTreeSet<&str> = ff_latch_nodes.collect();
    for pin in &builder.pin {
      match &pin.name {
        NameList::Name(name) => {
          _ = logic_variables.insert(name);
        }
        NameList::List(word_set) => {
          logic_variables.extend(word_set.inner.iter().map(String::as_str));
        }
      }
    }
    for pin in &builder.bundle {
      match &pin.name {
        NameList::Name(name) => {
          _ = logic_variables.insert(name);
        }
        NameList::List(word_set) => {
          logic_variables.extend(word_set.inner.iter().map(String::as_str));
        }
      }
    }
    for pin in &builder.bus {
      match &pin.name {
        NameList::Name(name) => {
          _ = logic_variables.insert(name);
        }
        NameList::List(word_set) => {
          logic_variables.extend(word_set.inner.iter().map(String::as_str));
        }
      }
    }
    scope.cell_extra_ctx.logic_variables = biodivine_lib_bdd::BddVariableSet::new(
      &logic_variables.into_iter().collect::<Vec<_>>(),
    );
    let mut pg_variable: Vec<&str> =
      builder.pg_pin.iter().map(|pg_pin| pg_pin.name.as_str()).collect();
    pg_variable.sort_unstable();
    scope.cell_extra_ctx.pg_variables =
      biodivine_lib_bdd::BddVariableSet::new(&pg_variable);
  }
  fn after_build(&mut self, scope: &mut BuilderScope<C>) {
    self.extra_ctx.set_logic_variables(mem::replace(
      &mut scope.cell_extra_ctx.logic_variables,
      biodivine_lib_bdd::BddVariableSet::new(&[]),
    ));
    self.extra_ctx.set_pg_variables(mem::replace(
      &mut scope.cell_extra_ctx.pg_variables,
      biodivine_lib_bdd::BddVariableSet::new(&[]),
    ));
  }
}

/// The `test_cell`  group is in a `cell` group or `model` group.
///
/// It models only the nontest behavior of a scan cell, which
/// is described by an `ff`, `ff_bank`, `latch`, `latch_bank`  or `statetable`  statement
/// and `pin` function attributes
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=218.9&end=218.11
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct TestCell<C: Ctx> {
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(group(type = Set))]
  pub ff: GroupSet<FF<C>>,
  #[liberty(group(type = Set))]
  pub ff_bank: GroupSet<FFBank<C>>,
  #[liberty(group(type = Set))]
  pub latch: GroupSet<Latch<C>>,
  #[liberty(group(type = Set))]
  pub latch_bank: GroupSet<LatchBank<C>>,
  #[liberty(group(type = Set))]
  pub pin: GroupSet<Pin<C>>,
  #[liberty(group(type = Set))]
  pub statetable: GroupSet<Statetable<C>>,
}

impl<C: Ctx> GroupFn<C> for TestCell<C> {}
