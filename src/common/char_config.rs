use core::fmt::{self, Write};

use strum::{Display, EnumString};

use crate::{
  Ctx,
  ast::{
    self, CodeFormatter, ComplexAttri, ComplexParseError, GroupComments, GroupFn,
    GroupSet, Indentation, ParseScope, SimpleAttri,
  },
};

use super::parse_f64;

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
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct CharConfig<C: Ctx> {
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: ast::Attributes,
  /// To specify the characterization method to account for the switching energy in the
  /// `internal_power` tables, set the `internal_power_calculation` attribute. Specify this
  /// attribute only if the `internal_power` group exists in the library.
  ///
  /// ### Syntax
  /// ``` text
  /// internal_power_calculation : exclude_switching_on_rise |
  /// exclude_switching_on_rise_and_fall |
  /// include_switching ;
  /// ```
  /// + `exclude_switching_on_rise`: The switching energy is deducted only from the `rise_power` table values.
  /// + `exclude_switching_on_rise_and_fall`: The switching energy is deducte d from both the `rise_power` and `fall_power`table values.
  /// + `include_switching`: The switching energy is not deducted from the table values in the `internal_power` group
  ///
  /// ### Example
  /// ``` text
  /// internal_power_calculation : exclude_switching_on_rise ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=49.20&end=49.36
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub internal_power_calculation: Option<InternalPowerCalculation>,
  /// The `three_state_disable_measurement_method` attribute specifies the method to
  /// identify the three-state condition of a pin. In a pin group, this attribute is valid only for a
  /// three-state pin. You must define this attribute if the library contains one or more threestate cells.
  ///
  /// ### Syntax
  /// ``` text
  /// three_state_disable_measurement_method : voltage | current ;
  /// ```
  /// + `voltage`: This method measures the volta ge waveform to thegate input of the three-state stage.
  /// + `current`: This method measures the leaka ge current flowingthrough the pullup and pulldown resistors of the threestate stage.
  ///
  /// ### Example
  /// ``` text
  /// three_state_disable_measurement_method : current ;
  /// ```
  #[liberty(simple(type = Option))]
  pub three_state_disable_measurement_method: Option<ThreeStateDisableMeasurementMethod>,
  /// The `three_state_disable_current_threshold_abs` attribute specifies the
  /// absolute current threshold value to distinguish between the low- and high-impedance
  /// states of a three-state output pin. The unit of the absolute current threshold value is
  /// specified in the `current_unit` attribute of the library group.
  ///
  /// In the pin group, this attribute is valid only for an inout pin. If you define both the
  /// `three_state_disable_current_threshold_abs` and
  /// `three_state_disable_current_threshold_rel` attributes, the pin enters the
  /// high-impedance state upon reaching either of the two threshold values.
  ///
  /// ### Syntax
  /// ``` text
  /// three_state_disable_current_threshold_abs : float ;
  /// ```
  /// ### Example
  /// ``` text
  /// three_state_disable_current_threshold_abs : 0.05 ;
  /// ```
  #[liberty(simple(type = Option))]
  pub three_state_disable_current_threshold_abs: Option<f64>,
  /// The `three_state_disable_current_threshold_rel` attribute specifies the relative
  /// current threshold value to distinguish between the low- and high-impedance states of a
  /// three-state output pin. The relative current threshold value is specified as a percentage
  /// of the peak current, for example, 100.0 for 100 percent of the peak current.
  ///
  /// In the pin group, this attribute is valid only for an inout pin. If you define both the
  /// `three_state_disable_current_threshold_abs` and
  /// `three_state_disable_current_threshold_rel` attributes, the pin enters the
  /// high-impedance state upon reaching either of the two threshold values.
  ///
  /// ### Syntax
  /// ``` text
  /// three_state_disable_current_threshold_rel : float ;
  /// ```
  /// ### Example
  /// ``` text
  /// three_state_disable_current_threshold_rel : 2.0 ;
  /// ```
  #[liberty(simple(type = Option))]
  pub three_state_disable_current_threshold_rel: Option<f64>,
  /// The `three_state_disable_monitor_node` attribute specifies the internal node that
  /// is probed for the three-state voltage measurement method.
  /// In the pin group, this attribute is valid only for an inout pin. You must define this attribute
  /// for the voltage method.
  ///
  /// ### Syntax
  /// ``` text
  /// three_state_disable_monitor_node : string ;
  /// ```
  /// ### Example
  /// ``` text
  /// three_state_disable_monitor_node : tri_monitor ;
  /// ```
  #[liberty(simple(type = Option))]
  pub three_state_disable_monitor_node: Option<String>,
  /// The `three_state_cap_add_to_load_index` attribute specifies that the pin
  /// capacitance of a three-state pin is added to each index value of the
  /// `total_output_net_capacitance` variable. The valid values are true and false.
  /// You must define this attribute.
  ///
  /// Syntax
  /// ``` text
  /// three_state_cap_add_to_load_index : true | false ;
  /// ```
  /// ### Example
  /// ``` text
  /// three_state_cap_add_to_load_index : true ;
  /// ```
  #[liberty(simple(type = Option))]
  pub three_state_cap_add_to_load_index: Option<bool>,
  /// The `ccs_timing_segment_voltage_tolerance_rel` attribute specifies the maximum
  /// permissible voltage difference between the simulation waveform and the CCS waveform to
  /// select the CCS model point. The floating-point value is specified in percent, where 100.0
  /// represents a 100 percent voltage difference.
  ///
  /// You must define this attribute when the library includes a CCS model.
  /// ### Syntax
  /// ``` text
  /// ccs_timing_segment_voltage_tolerance_rel: float ;
  /// ```
  /// ### Example
  /// ``` text
  /// ccs_timing_segment_voltage_tolerance_rel: 1.0 ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=49.38+50.2&end=49.39+50.8
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub ccs_timing_segment_voltage_tolerance_rel: Option<f64>,
  /// The `ccs_timing_delay_tolerance_rel` attribute specifies the acceptable difference
  /// between the CCS waveform delay and the delay measured from simulation. The floating-
  /// point value is specified in percent, where 100.0 represents 100 percent acceptable
  /// difference.
  ///
  /// You must define this attribute if the library includes a CCS model.
  ///
  /// ### Syntax
  /// ``` text
  /// ccs_timing_delay_tolerance_rel: float ;
  /// ```
  /// ### Example
  /// ``` text
  /// ccs_timing_delay_tolerance_rel: 2.0 ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=50.10&end=50.18
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub ccs_timing_delay_tolerance_rel: Option<f64>,
  /// The `ccs_timing_voltage_margin_tolerance_rel` attribute specifies the voltage
  /// tolerance for a signal to acquire the rail-voltage value. The floating-point value is specified
  /// as a percentage of the rail voltage, such as 96.0 for 96 percent of the rail voltage.
  /// You must define this attribute if the library includes a CCS model.
  ///
  /// ### Syntax
  /// ``` text
  /// ccs_timing_voltage_margin_tolerance_rel: float ;
  /// ```
  /// ### Example
  /// ``` text
  /// ccs_timing_voltage_margin_tolerance_rel: 1.0 ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=50.20&end=50.27
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub ccs_timing_voltage_margin_tolerance_rel: Option<f64>,
  /// The following CCS receiver capacitance attributes specify the current-integration limits, as
  /// a percentage of the voltage, to calculate the CCS receiver capacitances. The floating-point
  /// values of these attributes can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=50.29+51.2&end=50.31+51.20
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub receiver_capacitance1_voltage_lower_threshold_pct_rise: Option<f64>,
  /// The following CCS receiver capacitance attributes specify the current-integration limits, as
  /// a percentage of the voltage, to calculate the CCS receiver capacitances. The floating-point
  /// values of these attributes can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=50.29+51.2&end=50.31+51.20
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub receiver_capacitance1_voltage_upper_threshold_pct_rise: Option<f64>,
  /// The following CCS receiver capacitance attributes specify the current-integration limits, as
  /// a percentage of the voltage, to calculate the CCS receiver capacitances. The floating-point
  /// values of these attributes can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=50.29+51.2&end=50.31+51.20
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub receiver_capacitance1_voltage_lower_threshold_pct_fall: Option<f64>,
  /// The following CCS receiver capacitance attributes specify the current-integration limits, as
  /// a percentage of the voltage, to calculate the CCS receiver capacitances. The floating-point
  /// values of these attributes can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=50.29+51.2&end=50.31+51.20
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub receiver_capacitance1_voltage_upper_threshold_pct_fall: Option<f64>,
  /// The following CCS receiver capacitance attributes specify the current-integration limits, as
  /// a percentage of the voltage, to calculate the CCS receiver capacitances. The floating-point
  /// values of these attributes can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=50.29+51.2&end=50.31+51.20
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub receiver_capacitance2_voltage_lower_threshold_pct_rise: Option<f64>,
  /// The following CCS receiver capacitance attributes specify the current-integration limits, as
  /// a percentage of the voltage, to calculate the CCS receiver capacitances. The floating-point
  /// values of these attributes can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=50.29+51.2&end=50.31+51.20
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub receiver_capacitance2_voltage_upper_threshold_pct_rise: Option<f64>,
  /// The following CCS receiver capacitance attributes specify the current-integration limits, as
  /// a percentage of the voltage, to calculate the CCS receiver capacitances. The floating-point
  /// values of these attributes can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=50.29+51.2&end=50.31+51.20
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub receiver_capacitance2_voltage_lower_threshold_pct_fall: Option<f64>,
  /// The following CCS receiver capacitance attributes specify the current-integration limits, as
  /// a percentage of the voltage, to calculate the CCS receiver capacitances. The floating-point
  /// values of these attributes can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=50.29+51.2&end=50.31+51.20
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub receiver_capacitance2_voltage_upper_threshold_pct_fall: Option<f64>,
  /// The following input-capacitance measurement attributes specify the corresponding
  /// threshold values for the rising and falling voltage waveforms, to calculate the NLDM input-
  /// pin capacitance. Each floating-point threshold value is specified as a percentage of the
  /// supply voltage, and can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=51.22&end=51.36
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub capacitance_voltage_lower_threshold_pct_rise: Option<f64>,
  /// The following input-capacitance measurement attributes specify the corresponding
  /// threshold values for the rising and falling voltage waveforms, to calculate the NLDM input-
  /// pin capacitance. Each floating-point threshold value is specified as a percentage of the
  /// supply voltage, and can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=51.22&end=51.36
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub capacitance_voltage_lower_threshold_pct_fall: Option<f64>,
  /// The following input-capacitance measurement attributes specify the corresponding
  /// threshold values for the rising and falling voltage waveforms, to calculate the NLDM input-
  /// pin capacitance. Each floating-point threshold value is specified as a percentage of the
  /// supply voltage, and can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=51.22&end=51.36
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub capacitance_voltage_upper_threshold_pct_rise: Option<f64>,
  /// The following input-capacitance measurement attributes specify the corresponding
  /// threshold values for the rising and falling voltage waveforms, to calculate the NLDM input-
  /// pin capacitance. Each floating-point threshold value is specified as a percentage of the
  /// supply voltage, and can vary from 0.0 to 100.0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=51.22&end=51.36
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub capacitance_voltage_upper_threshold_pct_fall: Option<f64>,
  /// The `driver_waveform` attribute defines the driver waveform to characterize a specific
  /// characterization model.
  /// You can define the `driver_waveform` attribute within the `char_config` group at the
  /// `library`, `cell`, `pin`, and `timing` levels. If you define the `driver_waveform` attribute within the
  /// `char_config` group at the `library` level, the library-level `normalized_driver_waveform`
  /// group is ignored when the `driver_waveform_name` attribute is not defined.
  ///
  /// ### Syntax
  /// ``` text
  /// driver_waveform (char_model, waveform_name) ;
  /// ```
  /// ### Example
  /// ``` text
  /// driver_waveform ( all, input_driver ) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=51.38+52.2&end=51.39+52.9
  /// ">Reference</a>
  #[liberty(complex(type = Option))]
  pub driver_waveform: Option<[String; 2]>,
  /// The `driver_waveform_rise` attribute defines a specific rising driver waveform to
  /// characterize a specific characterization model.
  /// You can define the `driver_waveform_rise` attribute within the `char_config` group
  /// at the `library`, `cell`, `pin`, and `timing` levels. If you define the `driver_waveform_rise`
  /// attribute within the `char_config` group at the library level, the library-level
  /// `normalized_driver_waveform` group is ignored when the `driver_waveform_name`
  /// attribute is not defined.
  /// ### Syntax
  /// ``` text
  /// driver_waveform_rise ( char_model, waveform_name ) ;
  /// ```
  /// ### Example
  /// ``` text
  /// driver_waveform_rise ( all, input_driver ) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=52.11&end=52.21
  /// ">Reference</a>
  #[liberty(complex(type = Option))]
  pub driver_waveform_rise: Option<[String; 2]>,
  /// The `driver_waveform_fall` attribute defines a specific falling driver waveform to
  /// characterize a specific characterization model.
  /// You can define the `driver_waveform_fall` attribute within the `char_config` group
  /// at the `library`, `cell`, `pin`, and `timing` levels. If you define the `driver_waveform_fall`
  /// attribute within the `char_config` group at the library level, the library-level
  /// `normalized_driver_waveform` group is ignored when the `driver_waveform_name`
  /// attribute is not defined.
  ///
  /// ### Syntax
  /// ``` text
  /// driver_waveform_fall (char_model, waveform_name) ;
  /// ```
  /// ### Example
  /// ``` text
  /// driver_waveform_fall ( all, input_driver ) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=52.23+53.2&end=52.31+53.3
  /// ">Reference</a>
  #[liberty(complex(type = Option))]
  pub driver_waveform_fall: Option<[String; 2]>,
  /// The `input_stimulus_transition` attribute specifies the transition time for all the input-
  /// signal edges except the arc input pin's last transition, during generation of the input
  /// stimulus for simulation.
  /// The time units of the `input_stimulus_transition` attribute are specified by the library-level `time_unit` attribute.
  /// You must define this attribute.
  ///
  /// ### Syntax
  /// ``` text
  /// input_stimulus_transition ( char_model, float ) ;
  /// ```
  /// ### Example
  /// ``` text
  /// input_stimulus_transition ( all, 0.1 ) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=53.5&end=53.14
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub input_stimulus_transition: GroupSet<CharModeValue>,
  /// The `input_stimulus_interval` attribute specifies the time-interval between the input-
  /// signal toggles to generate the input stimulus for a characterization cell. The time units of
  /// this attribute are specified by the library-level `time_unit` attribute.
  ///
  /// You must define the `input_stimulus_interval` attribute.
  ///
  /// ### Syntax
  /// ``` text
  /// input_stimulus_interval ( char_model, float ) ;
  /// ```
  /// ### Example
  /// ``` text
  /// input_stimulus_interval ( all, 100.0 ) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=53.16&end=53.23
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub input_stimulus_interval: GroupSet<CharModeValue>,
  /// The `unrelated_output_net_capacitance` attribute specifies a load value for an output
  /// pin that is not a related output pin of the characterization model. The valid value is a
  /// floating-point number, and is defined by the library-level `capacitive_load_unit` attribute.
  /// If you do not specify this attribute for the `nldm_delay` and `nlpm_output` characterization
  /// models, the unrelated output pins use the load value of the related output pin. However,
  /// you must specify this attribute for any other characterization model.
  ///
  /// ### Syntax
  /// ``` text
  /// unrelated_output_net_capacitance ( char_model, float ) ;
  /// ```
  /// ### Example
  /// ``` text
  /// unrelated_output_net_capacitance ( all, 1.0 ) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=53.25+54.2&end=53.30+54.5
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub unrelated_output_net_capacitance: GroupSet<CharModeValue>,
  /// The `default_value_selection_method` attribute defines the method of selecting a
  /// default value for
  /// + The delay arc from state-dependent delay arcs
  /// + The constraint arc from state-dependent constraint arcs
  /// + Pin-based minimum pulse-width constraints from simulated results with side pin combinations
  /// + Internal power arcs from multiple state-dependent `internal_power` groups
  /// + The `cell_leakage_power` attribute from the state-dependent values in leakage power models
  /// + The input-pin capacitance from capacitance values for input-slew values used for timing characterization
  ///
  /// ### Syntax
  /// ``` text
  /// default_value_selection_method ( char_model, method ) ;
  /// ```
  /// For valid values of the method argument, see Table 3.
  /// ### Example
  /// ``` text
  /// default_value_selection_method ( all, any ) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=54.7&end=54.27
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub default_value_selection_method: GroupSet<CharModeMethod>,
  /// Use the `default_value_selection_method_rise` attribute when the selection method
  /// for rise is different from the selection method for fall.
  /// You must define either the `default_value_selection_method`
  /// attribute, or the `default_value_selection_method_rise` and
  /// `default_value_selection_method_fall` attributes.
  ///
  /// ### Syntax
  /// ``` text
  /// default_value_selection_method_rise ( char_model, method ) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=54.29+55.2&end=54.36+55.3
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub default_value_selection_method_rise: GroupSet<CharModeMethod>,
  /// Use the `default_value_selection_method_fall` attribute when the selection method
  /// for fall is different from the selection method for rise.
  /// You must define either the `default_value_selection_method`
  /// attribute, or the `default_value_selection_method_rise` and
  /// `default_value_selection_method_fall` attributes.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=55.5&end=55.14
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub default_value_selection_method_fall: GroupSet<CharModeMethod>,
  /// The `merge_tolerance_abs` attribute specifies the absolute tolerance to merge arc
  /// simulation results. Specify the absolute tolerance value in the corresponding library unit.
  /// If you specify both the `merge_tolerance_abs` and `merge_tolerance_rel` attributes, the
  /// results are merged if either or both the tolerance conditions are satisfied. If you do not
  /// specify any of these attributes, data including identical data is not merged.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=55.16&end=55.24
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub merge_tolerance_abs: GroupSet<CharModeValue>,
  /// The `merge_tolerance_rel` attribute specifies the relative tolerance to merge arc
  /// simulation results. Specify the relative tolerance value in percent, for example, 10.0 for 10
  /// percent.
  /// If you specify both the `merge_tolerance_abs` and `merge_tolerance_rel` attributes, the
  /// results are merged if either or both the tolerance conditions are satisfied. If you do not
  /// specify any of these attributes, data including identical data is not merged.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=55.26+56.2&end=55.31+56.5
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub merge_tolerance_rel: GroupSet<CharModeValue>,
  /// The `merge_selection` attribute specifies the method to select the merged data. When
  /// multiple sets of state-dependent data are merged, the attribute selects a particular set of
  /// the state-dependent data to represent the merged data.
  /// You must define the `merge_selection` attribute if you have defined the
  /// `merge_tolerance_abs` or `merge_tolerance_rel` attribute.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=56.7&end=56.18
  /// ">Reference</a>
  #[liberty(complex(type = Set))]
  pub merge_selection: GroupSet<CharModeMethod>,
}
impl<C: Ctx> GroupFn<C> for CharConfig<C> {}

/// To specify the characterization method to account for the switching energy in the
/// `internal_power` tables, set the `internal_power_calculation` attribute. Specify this
/// attribute only if the `internal_power` group exists in the library.
///
/// ### Syntax
/// ``` text
/// internal_power_calculation : exclude_switching_on_rise |
/// exclude_switching_on_rise_and_fall |
/// include_switching ;
/// ```
/// + `exclude_switching_on_rise`: The switching energy is deducted only from the `rise_power` table values.
/// + `exclude_switching_on_rise_and_fall`: The switching energy is deducted from both the `rise_power` and `fall_power` table values.
/// + `include_switching`: The switching energy is not deducted from the table values in the `internal_power` group
///
/// ### Example
/// ``` text
/// internal_power_calculation : exclude_switching_on_rise ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=49.20&end=49.36
/// ">Reference</a>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString, Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum InternalPowerCalculation {
  #[strum(serialize = "exclude_switching_rise", serialize = "exclude_switching_on_rise")]
  ExcludeSwitchingOnRise,
  #[strum(
    serialize = "exclude_switching_rise_and_fall",
    serialize = "exclude_switching_on_rise_and_fall"
  )]
  ExcludeSwitchingOnRiseAndFall,
  #[strum(serialize = "include_switching")]
  IncludeSwitching,
}
ast::impl_self_builder!(InternalPowerCalculation);
impl<C: Ctx> SimpleAttri<C> for InternalPowerCalculation {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> ast::SimpleParseRes<'a, Self> {
    ast::nom_parse_from_str::<C, _>(i, scope)
  }
}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=45.28+46.2&end=45.40+46.18
/// ">Reference</a>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(EnumString, Display, Default, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CharMode {
  /// Default model. The `all` model has the lowest priority
  /// among the valid models for the `char_config` group. Any
  /// other model overrides the `all` model.
  #[strum(serialize = "all")]
  #[default]
  All,
  /// Default nonlinear delay model (NLDM)
  #[strum(serialize = "nldm")]
  Nldm,
  /// Specific NLDMs that have higher priority over the default NLDM
  #[strum(serialize = "nldm_delay")]
  NldmDelay,
  /// Specific NLDMs that have higher priority over the default NLDM
  #[strum(serialize = "nldm_transition")]
  NldmTransition,
  /// Capacitance model
  #[strum(serialize = "capacitance")]
  Capacitance,
  /// Default constraint model
  #[strum(serialize = "constraint")]
  Constraint,
  /// Specific constraint models with higher priority over the
  /// default constraint model
  #[strum(serialize = "constraint_setup")]
  ConstraintSetup,
  /// Specific constraint models with higher priority over the
  /// default constraint model
  #[strum(serialize = "constraint_hold")]
  ConstraintHold,
  /// Specific constraint models with higher priority over the
  /// default constraint model
  #[strum(serialize = "constraint_recovery")]
  ConstraintRecovery,
  /// Specific constraint models with higher priority over the
  /// default constraint model
  #[strum(serialize = "constraint_removal")]
  ConstraintRemoval,
  /// Specific constraint models with higher priority over the
  /// default constraint model
  #[strum(serialize = "constraint_skew")]
  ConstraintSkew,
  /// Specific constraint models with higher priority over the
  /// default constraint model
  #[strum(serialize = "constraint_min_pulse_width")]
  ConstraintMinPulseWidth,
  /// Specific constraint models with higher priority over the
  /// default constraint model
  #[strum(serialize = "constraint_no_change")]
  ConstraintNoChange,
  /// Specific constraint models with higher priority over the
  /// default constraint model
  #[strum(serialize = "constraint_non_seq_setup")]
  ConstraintNonSeqSetup,
  /// Specific constraint models with higher priority over the
  /// default constraint model
  #[strum(serialize = "constraint_non_seq_hold")]
  ConstraintNonSeqHold,
  /// Specific constraint models with higher priority over the
  /// default constraint model
  #[strum(serialize = "constraint_minimum_period")]
  ConstraintMinimumPeriod,
  /// Default nonlinear power model (NLPM)
  #[strum(serialize = "nlpm")]
  Nlpm,
  /// Specific NLPM with higher priority over the default NLPM
  #[strum(serialize = "nlpm_leakage")]
  NlpmLeakage,
  /// Specific NLPM with higher priority over the default NLPM
  #[strum(serialize = "nlpm_input")]
  NlpmInput,
  /// Specific NLPM with higher priority over the default NLPM
  #[strum(serialize = "nlpm_output")]
  NlpmOutput,
}

/// The `three_state_disable_measurement_method` attribute specifies the method to
/// identify the three-state condition of a pin. In a pin group, this attribute is valid only for a
/// three-state pin. You must define this attribute if the library contains one or more threestate cells.
///
/// ### Syntax
/// ``` text
/// three_state_disable_measurement_method : voltage | current ;
/// ```
/// + `voltage`: This method measures the voltage waveform to the gate input of the three-state stage.
/// + `current`: This method measures the leakage current flowing through the pullup and pulldown resistors of the threestate stage.
///
/// ### Example
/// ``` text
/// three_state_disable_measurement_method : current ;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString, Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ThreeStateDisableMeasurementMethod {
  /// Selects a random value from the state-dependent data.
  #[strum(serialize = "voltage")]
  Voltage,
  #[strum(serialize = "current")]
  Current,
}

ast::impl_self_builder!(ThreeStateDisableMeasurementMethod);
impl<C: Ctx> SimpleAttri<C> for ThreeStateDisableMeasurementMethod {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> ast::SimpleParseRes<'a, Self> {
    ast::nom_parse_from_str::<C, _>(i, scope)
  }
}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=46.21+47.2+48.2+49.2&end=46.35+47.44+48.54+49.18
/// ">Reference</a>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(EnumString, Display, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SelectionMethod {
  /// Selects a random value from the state-dependent data.
  #[strum(serialize = "any")]
  #[default]
  Any,
  #[strum(serialize = "min")]
  /// Selects the minimum value from the state-dependent data at each index point.
  Min,
  /// Selects the maximum value from the state-dependent data at each index point.
  #[strum(serialize = "max")]
  Max,
  /// Selects an average value from the state-dependent data at each index point.
  #[strum(serialize = "average")]
  Average,
  /// Selects the minimum value from the state-dependent data in a lookup table. The
  /// minimum value is selected by comparing the middle value in the lookup table,
  /// with each of the table-values.
  ///
  /// Note:
  ///
  /// The middle value corresponds to an index value. If the number of index
  /// values is odd, then the middle value is taken as the median value. However,
  /// if the number of index values is even, then the smaller of the two values is
  /// selected as the middle value.
  #[strum(serialize = "min_mid_table ")]
  MinMidTable,
  /// Selects the maximum value from the state-dependent data in the lookup table.
  /// The maximum value is selected by comparing the middle value in the lookup
  /// table, with each of the table-values.
  ///
  /// Note:
  ///
  /// The middle value corresponds to an index value. If the number of index
  /// values is odd, then the middle value is taken as the median value. However,
  /// if the number of index values is even, then the smaller of the two values is
  /// selected as the middle value.
  #[strum(serialize = "max_mid_table")]
  MaxMidTable,
  /// Selects the value from the state-dependent data for delay selection. This
  /// method is valid only for the `nldm_transition` characterization model, that
  /// is, the `follow_delay` method applies specifically to default transition-table
  /// selection and not any other default-value selection.
  #[strum(serialize = "follow_delay")]
  FollowDelay,
}

#[derive(Debug, Clone, Default, Copy)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CharModeMethod {
  #[id]
  char_mode: CharMode,
  method: SelectionMethod,
}
ast::impl_self_builder!(CharModeMethod);
impl<C: Ctx> ComplexAttri<C> for CharModeMethod {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let mut i = iter;
    let char_mode = match i.next() {
      Some(&s) => s.parse()?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let method = match i.next() {
      Some(s) => s.parse()?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self { char_mode, method })
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{}", self.char_mode)?;
    f.write_str(", ")?;
    write!(f, "{}", self.method)
  }
}

#[derive(Debug, Clone, Default, Copy)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CharModeValue {
  #[id]
  char_mode: CharMode,
  value: f64,
}
ast::impl_self_builder!(CharModeValue);
impl<C: Ctx> ComplexAttri<C> for CharModeValue {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let mut i = iter;
    let char_mode = match i.next() {
      Some(&s) => s.parse()?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let value = match i.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self { char_mode, value })
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{}", self.char_mode)?;
    f.write_str(", ")?;
    f.write_num(self.value)
  }
}

#[cfg(test)]
mod test {
  use crate::DefaultCtx;
  #[test]
  fn char_config() {
    let g = crate::ast::test_parse_fmt::<super::CharConfig<DefaultCtx>>(
      r#"() {
        /* library level default attributes*/
        driver_waveform(all, input_driver);
        input_stimulus_transition(all, 0.1);
        input_stimulus_interval(all, 100.0);
        unrelated_output_net_capacitance(all, 1.0);
        default_value_selection_method(all, any);
        merge_tolerance_abs( nldm, 0.1) ;
        merge_tolerance_abs( constraint, 0.1) ;
        merge_tolerance_abs( capacitance, 0.01) ;
        merge_tolerance_abs( nlpm, 0.05) ;
        merge_tolerance_rel( all, 2.0) ;
        merge_selection( all, max) ;
        internal_power_calculation : exclude_switching_rise;
        three_state_disable_measurement_method : current;
        three_state_disable_current_threshold_abs : 0.05;
        three_state_disable_current_threshold_rel : 2.0;
        three_state_disable_monitor_node : tri_monitor;
        three_state_cap_add_to_load_index : true;
        ccs_timing_segment_voltage_tolerance_rel: 1.0 ;
        ccs_timing_delay_tolerance_rel: 2.0 ;
        ccs_timing_voltage_margin_tolerance_rel: 1.0 ;
        receiver_capacitance1_voltage_lower_threshold_pct_rise : 20.0;
        receiver_capacitance1_voltage_upper_threshold_pct_rise : 50.0;
        receiver_capacitance1_voltage_lower_threshold_pct_fall : 50.0;
        receiver_capacitance1_voltage_upper_threshold_pct_fall : 80.0;
        receiver_capacitance2_voltage_lower_threshold_pct_rise : 20.0;
        receiver_capacitance2_voltage_upper_threshold_pct_rise : 50.0;
        receiver_capacitance2_voltage_lower_threshold_pct_fall : 50.0;
        receiver_capacitance2_voltage_upper_threshold_pct_fall : 80.0;
        capacitance_voltage_lower_threshold_pct_rise : 20.0;
        capacitance_voltage_lower_threshold_pct_fall : 50.0;
        capacitance_voltage_upper_threshold_pct_rise : 50.0;
        capacitance_voltage_upper_threshold_pct_fall : 80.0;
        }"#,
      r#"
liberty_db::common::char_config::CharConfig () {
| internal_power_calculation : exclude_switching_on_rise;
| three_state_disable_measurement_method : current;
| three_state_disable_current_threshold_abs : 0.05;
| three_state_disable_current_threshold_rel : 2.0;
| three_state_disable_monitor_node : tri_monitor;
| three_state_cap_add_to_load_index : true;
| ccs_timing_segment_voltage_tolerance_rel : 1.0;
| ccs_timing_delay_tolerance_rel : 2.0;
| ccs_timing_voltage_margin_tolerance_rel : 1.0;
| receiver_capacitance1_voltage_lower_threshold_pct_rise : 20.0;
| receiver_capacitance1_voltage_upper_threshold_pct_rise : 50.0;
| receiver_capacitance1_voltage_lower_threshold_pct_fall : 50.0;
| receiver_capacitance1_voltage_upper_threshold_pct_fall : 80.0;
| receiver_capacitance2_voltage_lower_threshold_pct_rise : 20.0;
| receiver_capacitance2_voltage_upper_threshold_pct_rise : 50.0;
| receiver_capacitance2_voltage_lower_threshold_pct_fall : 50.0;
| receiver_capacitance2_voltage_upper_threshold_pct_fall : 80.0;
| capacitance_voltage_lower_threshold_pct_rise : 20.0;
| capacitance_voltage_lower_threshold_pct_fall : 50.0;
| capacitance_voltage_upper_threshold_pct_rise : 50.0;
| capacitance_voltage_upper_threshold_pct_fall : 80.0;
| input_stimulus_transition (all, 0.1);
| input_stimulus_interval (all, 100.0);
| unrelated_output_net_capacitance (all, 1.0);
| default_value_selection_method (all, any);
| merge_tolerance_abs (nldm, 0.1);
| merge_tolerance_abs (capacitance, 0.01);
| merge_tolerance_abs (constraint, 0.1);
| merge_tolerance_abs (nlpm, 0.05);
| merge_tolerance_rel (all, 2.0);
| merge_selection (all, max);
}"#,
    );
  }
}
