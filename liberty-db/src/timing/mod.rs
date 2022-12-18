//! <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>

use hashbrown::HashMap;
use compact_str::CompactString;
mod types;
pub use types::*;
pub mod items;
use crate::common::*;

/// A timing group is defined in a bundle, a bus, or a pin group within a cell. 
/// The timing group can be used to identify the name or names of multiple timing arcs. 
/// A timing group identifies multiple timing arcs, by identifying a timing arc in a pin group 
/// that has more than one related pin or when the timing arc is part of a bundle or a bus.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.hc.ye79.ffc.fs6.fc1.sc0.ls0.ws0
/// +t.m0.x9.hc.y2910.ffc.fs6.fc1.sc0.ls0.ws0
/// &end
/// =t.m0.x39.h8.ye8a.ff7.fs2.fc2.sc0.ls0.ws0
/// +t.m0.x39.h4.y2925.ff1.fs2.fc2.sc0.ls0.ws0
/// ">Reference</a>
/// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
#[derive(Debug, Clone, Default)]
// #[derive(Syntax)]
pub struct Timing{
    /// Use this attribute to indicate that a constraint arc is for 
    /// a clock gating relation between the data and clock pin, 
    /// instead of a constraint found in standard sequential devices, 
    /// such as registers and latches. <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.xb.h8.y2927.ff7.fs2.fc2.sc0.ls0.ws0
    /// +t.m0.x9.h4.y297d.ff1.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.xb.h8.y2927.ff7.fs2.fc2.sc0.ls0.ws0
    /// +t.m0.x39.h8.y2989.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    /// 
    /// **Syntax:**
    /// 
    /// ```clock_gating_flag : Boolean ; ```
    /// 
    /// Valid values are true and false. The value true is applicable
    /// only when the value of the timing_type attribute is setup, hold, or nochange. 
    /// When not defined for a timing arc, the value false is assumed, 
    /// indicating the timing arc is part of a standard sequential device.
    /// 
    pub clock_gating_flag: Option<bool>,
    pub default_timing: Option<bool>,
    pub fall_resistance: Option<f64>,
    pub fpga_arc_condition: Option<BooleanExpression>,
    pub fpga_domain_style: Option<CompactString>,
    pub interdependence_id: Option<i64>,
    pub intrinsic_fall: Option<f64>,
    pub intrinsic_rise: Option<f64>,
    pub related_bus_equivalent: Vec<CompactString>,
    pub related_bus_pins: Vec<CompactString>,
    pub related_output_pin: Option<CompactString>,
    pub related_pin: Vec<CompactString>,
    pub rise_resistance: Option<f64>,
    pub sdf_cond: Option<SdfExpression>,
    pub sdf_cond_end: Option<SdfExpression>,
    pub sdf_cond_start: Option<SdfExpression>,
    pub sdf_edges: Option<SdfEdgeType>,
    pub slope_fall: Option<f64>,
    pub slope_rise: Option<f64>,
    pub steady_state_resistance_above_high: Option<f64>,
    pub steady_state_resistance_below_low: Option<f64>,
    pub steady_state_resistance_high: Option<f64>,
    pub steady_state_resistance_low: Option<f64>,
    pub tied_off: Option<bool>,
    pub timing_sense: Option<TimingSenseType>,
    pub timing_type: Option<TimingType>,
    pub when: Option<BooleanExpression>,
    pub when_end: Option<BooleanExpression>,
    pub when_start: Option<BooleanExpression>,

    // Complex Attributes
    /// piecewise model only
    pub fall_delay_intercept: Option<(i64, f64)>,
    /// piecewise model only
    pub fall_pin_resistance: Option<(i64, f64)>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x9.h4.y2c4c.ff1.fs2.fc2.sc0.ls0.ws0
    /// +t.m0.x9.h7.y2c58.ff1.fs2.fc1.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2c56.ff7.fs2.fc2.sc0.ls0.ws0
    /// +t.m0.xb.h8.y2c8e.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub mode: items::Mode,
    /// piecewise model only
    pub rise_delay_intercept: Option<(i64, f64)>,
    /// piecewise model only
    pub rise_pin_resistance: Option<(i64, f64)>,
    pub cell_degradation: HashMap<CompactString,items::CallFall>,
    /// Defines cell delay lookup tables (independently of transition delay) in CMOS nonlinear timing models.
    /// **Note: **
    /// 
    /// The same k-factors that scale the cell_fall and cell_rise values also scale the 
    /// retaining_fall and retaining_rise values. There are no separate k-factors for 
    /// the retaining_fall and retaining_rise values.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x9.h4.y2d8f.ff1.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h4.y2daa.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub cell_fall: HashMap<CompactString,items::CallFall>,
    pub cell_rise: HashMap<CompactString,items::CallFall>,
    pub fall_constraint: HashMap<CompactString,items::CallFall>,
    pub fall_propagation: HashMap<CompactString,items::CallFall>,
    pub fall_transition: HashMap<CompactString,items::CallFall>,
    pub noise_immunity_above_high: HashMap<CompactString,items::CallFall>,
    pub noise_immunity_below_low: HashMap<CompactString,items::CallFall>,
    pub noise_immunity_high: HashMap<CompactString,items::CallFall>,
    pub noise_immunity_low: HashMap<CompactString,items::CallFall>,
    pub output_current_fall: HashMap<CompactString,items::CallFall>,
    pub output_current_rise: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_height_above_high: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_height_below_low: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_height_high: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_height_low: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_peak_time_ratio_above_high: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_peak_time_ratio__below_low: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_peak_time_ratio_high: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_peak_time_ratio_low: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_width_above_high: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_width_below_low: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_width_high: HashMap<CompactString,items::CallFall>,
    pub propogated_noise_width_low: HashMap<CompactString,items::CallFall>,
    pub receiver_capacitance1_fall: HashMap<CompactString,items::CallFall>,
    pub receiver_capacitance1_rise: HashMap<CompactString,items::CallFall>,
    pub receiver_capacitance2_fall: HashMap<CompactString,items::CallFall>,
    pub receiver_capacitance2_rise: HashMap<CompactString,items::CallFall>,
    pub retaining_fall: HashMap<CompactString,items::CallFall>,
    pub retaining_rise: HashMap<CompactString,items::CallFall>,
    pub retain_fall_slew: HashMap<CompactString,items::CallFall>,
    pub retain_rise_slew: HashMap<CompactString,items::CallFall>,
    pub rise_constraint: HashMap<CompactString,items::CallFall>,
    pub rise_propagation: HashMap<CompactString,items::CallFall>,
    pub rise_transition: HashMap<CompactString,items::CallFall>,
    pub steady_state_current_high: HashMap<CompactString,items::CallFall>,
    pub steady_state_current_low: HashMap<CompactString,items::CallFall>,
    pub steady_state_current_tristate: HashMap<CompactString,items::CallFall>,

}