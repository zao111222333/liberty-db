//! `pin` group structure.
//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
//! </script>

use crate::types::*;
use crate::expression::{BooleanExpression, CommonState};
use crate::timing::Timing;
mod items;
pub use items::*;
/// You can define a `pin` group within a [`cell`](crate::cell::Cell), 
/// [`test_cell`](crate::test_cell), [`model`](crate::model), 
/// or [`bus`](crate::bus::Bus) group.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =227.0
/// &end
/// =227.8
/// ">Reference</a>
/// 
/// + An example of the `pin` group syntax showing the attribute 
/// and group statements that you can use within the `pin` group
/// + Descriptions of the attributes and groups you can use in a `pin` group
#[derive(Debug, Default)]
pub struct Pin<'a>{
    pub group_name: String,
    /* Simple Attributes in a pin Group */
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =227.33
    /// &end
    /// =227.33
    /// ">Reference-Instance</a>
    pub alive_during_partial_power_down: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.2
    /// &end
    /// =228.2
    /// ">Reference-Instance</a>
    pub alive_during_power_up: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.3
    /// &end
    /// =228.3
    /// ">Reference-Instance</a>
    pub always_on: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.4
    /// &end
    /// =228.4
    /// ">Reference-Instance</a>
    pub antenna_diode_type: Option<AntennaDiodeType>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.5
    /// &end
    /// =228.5
    /// ">Reference-Instance</a>
    // FIXME:
    // pub antenna_diode_related_ground_pins : "ground_pin1 ground_pin2",
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.6
    /// &end
    /// =228.6
    /// ">Reference-Instance</a>
    // FIXME:
    // pub antenna_diode_related_power_pins : "power_pin1 power_pin2",
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.7
    /// &end
    /// =228.7
    /// ">Reference-Instance</a>
    /* bus cells */
    pub bit_width: usize, 
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.8
    /// &end
    /// =228.8
    /// ">Reference-Instance</a>
    pub capacitance: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.9
    /// &end
    /// =228.9
    /// ">Reference-Instance</a>
    pub clamp_0_function: Option<BooleanExpression>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.10
    /// &end
    /// =228.10
    /// ">Reference-Instance</a>
    pub clamp_1_function: Option<BooleanExpression>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.11
    /// &end
    /// =228.11
    /// ">Reference-Instance</a>
    pub clamp_latch_function: Option<BooleanExpression>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.12
    /// &end
    /// =228.12
    /// ">Reference-Instance</a>
    pub clamp_z_function: Option<BooleanExpression>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.13
    /// &end
    /// =228.13
    /// ">Reference-Instance</a>
    pub clock: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.14
    /// &end
    /// =228.14
    /// ">Reference-Instance</a>
    pub clock_gate_clock_pin: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.15
    /// &end
    /// =228.15
    /// ">Reference-Instance</a>
    pub clock_gate_enable_pin: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.16
    /// &end
    /// =228.16
    /// ">Reference-Instance</a>
    pub clock_gate_test_pin: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.17
    /// &end
    /// =228.17
    /// ">Reference-Instance</a>
    pub clock_gate_obs_pin: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.18
    /// &end
    /// =228.18
    /// ">Reference-Instance</a>
    pub clock_gate_out_pin: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.19
    /// &end
    /// =228.19
    /// ">Reference-Instance</a>
    pub clock_isolation_cell_clock_pin: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.20
    /// &end
    /// =228.20
    /// ">Reference-Instance</a>
    pub complementary_pin: String,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.21
    /// &end
    /// =228.21
    /// ">Reference-Instance</a>
    pub connection_class: Vec<String>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.22
    /// &end
    /// =228.22
    /// ">Reference-Instance</a>
    pub direction: Option<Direction>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.23
    /// &end
    /// =228.23
    /// ">Reference-Instance</a>
    pub dont_fault: Option<DontFault>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.24
    /// &end
    /// =228.24
    /// ">Reference-Instance</a>
    pub drive_current: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.25
    /// &end
    /// =228.27
    /// ">Reference-Instance</a>
    pub driver_type: Option<DriverType>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.28
    /// &end
    /// =228.28
    /// ">Reference-Instance</a>
    pub fall_capacitance: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.29
    /// &end
    /// =228.29
    /// ">Reference-Instance</a>
    pub fall_current_slope_after_threshold: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.30
    /// &end
    /// =228.30
    /// ">Reference-Instance</a>
    pub fall_current_slope_before_threshold: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.31
    /// &end
    /// =228.31
    /// ">Reference-Instance</a>
    pub fall_time_after_threshold: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.32
    /// &end
    /// =228.32
    /// ">Reference-Instance</a>
    pub fall_time_before_threshold: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.33
    /// &end
    /// =228.33
    /// ">Reference-Instance</a>
    pub fanout_load: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.34
    /// &end
    /// =228.34
    /// ">Reference-Instance</a>
    // FIXME:
    // pub fault_model: "two-value string" ;
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.35
    /// &end
    /// =228.35
    /// ">Reference-Instance</a>
    pub function: Option<BooleanExpression>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.36
    /// &end
    /// =228.36
    /// ">Reference-Instance</a>
    pub has_builtin_pad: Option<BooleanExpression>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.37
    /// &end
    /// =228.37
    /// ">Reference-Instance</a>
    pub hysteresis: bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.38
    /// &end
    /// =228.38
    /// ">Reference-Instance</a>
    pub illegal_clamp_condition: Option<BooleanExpression>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.39
    /// &end
    /// =228.41
    /// ">Reference-Instance</a>
    pub input_map: Vec<String>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.42
    /// &end
    /// =228.42
    /// ">Reference-Instance</a>
    pub input_signal_level: String,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.43
    /// &end
    /// =228.43
    /// ">Reference-Instance</a>
    pub input_voltage : String,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.44
    /// &end
    /// =228.46
    /// ">Reference-Instance</a>
    pub internal_node: String, /* Required in statetable cells */
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.47
    /// &end
    /// =228.47
    /// ">Reference-Instance</a>
    pub inverted_output: bool,/* Required in statetable cells */
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.48
    /// &end
    /// =228.48
    /// ">Reference-Instance</a>
    pub is_pad : bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.49
    /// &en7
    /// =228.49
    /// ">Reference-Instance</a>
    pub is_unconnected : bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.50
    /// &end
    /// =228.50
    /// ">Reference-Instance</a>
    pub max_capacitance: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.51
    /// &end
    /// =228.41
    /// ">Reference-Instance</a>
    pub max_fanout: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.52
    /// &end
    /// =228.52
    /// ">Reference-Instance</a>
    pub max_input_delta_overdrive_high: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.53
    /// &end
    /// =228.53
    /// ">Reference-Instance</a>
    pub max_input_delta_underdrive_high: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.54
    /// &end
    /// =228.54
    /// ">Reference-Instance</a>
    pub max_transition: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.55
    /// &end
    /// =228.55
    /// ">Reference-Instance</a>
    pub min_capacitance: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.56
    /// &end
    /// =228.56
    /// ">Reference-Instance</a>
    pub min_fanout: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.57
    /// &end
    /// =228.57
    /// ">Reference-Instance</a>
    pub min_period: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.58
    /// &end
    /// =228.58
    /// ">Reference-Instance</a>
    pub min_pulse_width_high: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.59
    /// &end
    /// =228.59
    /// ">Reference-Instance</a>
    pub min_pulse_width_low: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.60
    /// &end
    /// =228.60
    /// ">Reference-Instance</a>
    pub min_transition: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.61
    /// &end
    /// =228.61
    /// ">Reference-Instance</a>
    pub multicell_pad_pin : bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.62
    /// &end
    /// =228.62
    /// ">Reference-Instance</a>
    pub nextstate_type: Option<NextstateType>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.63
    /// &end
    /// =228.63
    /// ">Reference-Instance</a>
    pub output_signal_level: String,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.64
    /// &end
    /// =228.64
    /// ">Reference-Instance</a>
    pub output_signal_level_high: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.65
    /// &end
    /// =228.65
    /// ">Reference-Instance</a>
    pub output_signal_level_low: Float,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =228.66
    /// &end
    /// =228.66
    /// ">Reference-Instance</a>
    pub output_voltage : String,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =229.2
    /// &end
    /// =229.3
    /// ">Reference-Instance</a>
    pub pin_func_type : Option<PinFuncType>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =229.4
    /// &end
    /// =229.4
    /// ">Reference-Instance</a>
    // FIXME:
    pub prefer_tied : bool,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =229.5
    /// &end
    /// =229.5
    /// ">Reference-Instance</a>
    pub primary_output : bool,
    // pulling_current : current value ;
    // pulling_resistance : resistance value;
    pub restore_action: Option<CommonState>,
    // restore_edge_type : edge_trigger | leading | trailing ;
    // rise_capacitance: Float,
    // rise_current_slope_after_threshold: Float,
    // rise_current_slope_before_threshold: Float,
    // rise_time_after_threshold: Float, 
    // rise_time_before_threshold: Float,
    pub save_action: Option<CommonState>,
    // signal_type : test_scan_in | test_scan_in_inverted | test_scan_out | test_scan_out_inverted | test_scan_enable |  test_scan_enable_inverted |test_scan_clock | test_scan_clock_a | test_scan_clock_b | test_clock ;
    // slew_control : low | medium | high | none ;
    // state_function: BooleanExpression,
    // test_output_only :  bool,
    // three_state: BooleanExpression,
    // x_function: BooleanExpression,   
    // /* Complex Attributes in a pin Group */ 
    // fall_capacitance_range ( float, float) ; 
    // rise_capacitance_range ( float, float) ;    
    // /* Group Statements in a pin Group */ 
    // electromigration () { }
    // input_ccb (string) { }
    // internal_power ()  { }
    // max_trans () { }
    // min_pulse_width ()  { }
    // minimum_period ()  { }
    // output_ccb (string) { }
    // timing ()  { }
    // tlatch () {}
    // / A timing group is defined within a pin group.
    // / 
    // / Reference:
    // / <iframe 
    // / src="
    // / https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    // / ?field=test
    // / &bgn
    // / =67.26
    // / &end
    // / =67.43
    // / " 
    // / style="width: 90%; height: 600px;"></iframe>
    pub timing_list: Vec<Timing<'a>>,
}