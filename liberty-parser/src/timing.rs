use strum_macros::{Display, EnumString};
use compact_str::CompactString;
use crate::ast::*;
/// 
#[derive(Debug, Clone)]
pub enum Timing{
    ///
    SimpleAttribute(SimpleAttribute),
    ///
    GroupStatement(GroupStatement),
}
/// # Simple Attribute
/// 
/// <script src="http://code.jquery.com/jquery-latest.js"></script>
/// <iframe src="http://api.jquery.com/" width="80%" height="600" id='frameDemo'></iframe>
/// <script>$("#frameDemo").contents().find("div").html("new HTML content goes here");</script>
#[derive(Debug, Clone, PartialEq)]
#[derive(Display, EnumString)]
pub enum SimpleAttribute{
    /// clock_gating_flag : true|false ;
    #[strum(serialize = "clock_gating_flag")]
    ClockGatingFlag(bool),
    /// default_timing : true|false ;
    #[strum(serialize = "default_timing")]
    DefaultTiming(bool),
    /// fall_resistance : float ;
    #[strum(serialize = "fall_resistance")]
    FallResistance(f64),
    /// fpga_arc_condition : "Boolean expression" ;
    #[strum(serialize = "fpga_arc_condition")]
    FpgaArcCondition(BooleanExpression),
    /// fpga_domain_style : name ;
    #[strum(serialize = "fpga_domain_style")]
    FpgaDomainStyle(CompactString),
    /// interdependence_id : integer ;
    #[strum(serialize = "interdependence_id")]
    InterdependenceId(i64),
    /// intrinsic_fall : float ;
    #[strum(serialize = "intrinsic_fall")]
    IntrinsicFall(f64),
    /// intrinsic_rise : float ;
    #[strum(serialize = "intrinsic_rise")]
    IntrinsicRise(f64),
    /// related_bus_equivalent : " name1 [name2 name3 ... ] " ;
    #[strum(serialize = "related_bus_equivalent")]
    RelatedBusEquivalent(NameList),
    /// related_bus_pins : " name1 [name2 name3 ... ] " ;
    #[strum(serialize = "related_bus_pins")]
    RelatedBusPins(NameList),
    /// related_output_pin : name ;
    #[strum(serialize = "related_output_pin")]
    RelatedOutputPin(CompactString),
    /// related_pin : " name1 [name2 name3 ... ] " ;
    #[strum(serialize = "related_pin")]
    RelatedPin(NameList),
    /// rise_resistance : float ;
    #[strum(serialize = "rise_resistance")]
    RiseResistance(f64),
    /// sdf_cond : "SDF expression" ;
    #[strum(serialize = "sdf_cond")]
    SdfCond(SdfExpression),
    /// sdf_cond_end : "SDF expression" ;
    #[strum(serialize = "sdf_cond_end")]
    SdfCondEnd(SdfExpression),
    /// sdf_cond_start : "SDF expression" ;
    #[strum(serialize = "sdf_cond_start")]
    SdfCondStart(SdfExpression),
    /// sdf_edges : SDF edge type ;
    #[strum(serialize = "sdf_edges")]
    SdfEdges(SdfEdgeType),
    /// slope_fall : float ;
    #[strum(serialize = "slope_fall")]
    SlopeFall(f64),
    /// slope_rise : float ;
    #[strum(serialize = "slope_rise")]
    SlopeRise(f64),
    /// steady_state_resistance_above_high : float ;
    #[strum(serialize = "steady_state_resistance_above_high")]
    SteadyStateResistanceAboveHigh(f64),
    /// steady_state_resistance_below_low : float ;
    #[strum(serialize = "steady_state_resistance_below_low")]
    SteadyStateResistanceBelowLow(f64),
    /// steady_state_resistance_high : float ;
    #[strum(serialize = "steady_state_resistance_high")]
    SteadyStateResistanceHigh(f64),
    /// steady_state_resistance_low : float ;
    #[strum(serialize = "steady_state_resistance_low")]
    SteadyStateResistanceLow(f64),
    /// tied_off: Boolean ;
    #[strum(serialize = "tied_off")]
    TiedOff(bool),
    /// timing_sense : positive_unate| negative_unate| non_unate;
    #[strum(serialize = "timing_sense")]
    TimingSense(TimingSenseType),
    /// timing_type : combinational | combinational_rise |
    /// combinational_fall | three_state_disable |
    /// three_state_disable_rise | three_state_disable_fall |
    /// three_state_enable | three_state_enable_rise |
    /// three_state_enable_fall |rising_edge | falling_edge |
    /// preset | clear | hold_rising | hold_falling |
    /// setup_rising | setup_falling | recovery_rising |
    /// recovery_falling | skew_rising | skew_falling |
    /// removal_rising | removal_falling | min_pulse_width |
    /// minimum_period | max_clock_tree_path |
    /// min_clock_tree_path |non_seq_setup_rising |
    /// non_seq_setup_falling | non_seq_hold_rising |
    /// non_seq_hold_falling | nochange_high_high |
    /// nochange_high_low | nochange_low_high |
    /// nochange_low_low ;
    #[strum(serialize = "timing_type")]
    TimingType(TimingType),
    /// when : "Boolean expression" ;
    #[strum(serialize = "when")]
    When(BooleanExpression),
    /// when_end : "Boolean expression" ;
    #[strum(serialize = "when_end")]
    WhenEnd(BooleanExpression),
    /// when_start : "Boolean expression" ;
    #[strum(serialize = "when_start")]
    WhenStart(BooleanExpression),
}
// fall_delay_intercept (integer, float) ; /* piecewise model only */
// fall_pin_resistance (integer, float) ; /* piecewise model only */
// mode
// rise_delay_intercept (integer, float) ; /* piecewise model only */
// rise_pin_resistance (integer, float) ; /* piecewise model only */

// <div class="t m0 x8 h5 y2510 ff1 fs2 fc2 sc0 ls0 ws0">Complex Attributes</div>

/// # Group Statement
/// Reference:
/// <iframe 
/// src="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x8.h5.y2516.ff1.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.x9.hc.y253c.ff7.fs2.fc2.sc0.ls0
/// " 
/// style="width: 90%; height: 600px;"></iframe>
#[derive(Debug, Clone, PartialEq)]
#[derive(Display, EnumString)]
pub enum GroupStatement {
    /// cell_degradation () { }
    /// 
    /// Reference:
    /// <iframe 
    /// src="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x8.h5.y2899.ff1.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.xa.hc.y28ab.ff7.fs2.fc2.sc0.ls0
    /// " 
    /// style="width: 90%; height: 600px;"></iframe>
    #[strum(serialize = "cell_degradation")]
    CellDegradation,
    /// cell_fall () { }
    #[strum(serialize = "cell_fall")]
    CellFall(CallFall),
    /// cell_rise () { }
    #[strum(serialize = "cell_rise")]
    CellRise,
    /// fall_constraint () { }
    #[strum(serialize = "fall_constraint")]
    FallConstraint,
    /// fall_propagation () { }
    #[strum(serialize = "fall_propagation")]
    FallPropagation,
    /// fall_transition () { }
    #[strum(serialize = "fall_transition")]
    FallTransition,
    /// noise_immunity_above_high () { }
    #[strum(serialize = "noise_immunity_above_high")]
    FoiseImmunityAboveHigh,
    /// noise_immunity_below_low () { }
    #[strum(serialize = "noise_immunity_below_low")]
    FoiseImmunityBelowLow,
    /// noise_immunity_high () { }
    #[strum(serialize = "noise_immunity_high")]
    FoiseImmunityHigh,
    /// noise_immunity_low () { }
    #[strum(serialize = "noise_immunity_low")]
    FoiseImmunityLow,
    /// output_current_fall () { }
    #[strum(serialize = "output_current_fall")]
    OutputCurrentFall,
    /// output_current_rise () { }
    #[strum(serialize = "output_current_rise")]
    OutputCurrentRise,
    /// propogated_noise_height_above_high () { }
    #[strum(serialize = "propogated_noise_height_above_high")]
    PropogatedNoiseHeightAboveHigh,
    /// propogated_noise_height_below_low () { }
    #[strum(serialize = "propogated_noise_height_below_low")]
    PropogatedNoiseHeightBelowLow,
    /// propogated_noise_height_high () { }
    #[strum(serialize = "propogated_noise_height_high")]
    PropogatedNoiseHeightHigh,
    /// propogated_noise_height_low () { }
    #[strum(serialize = "propogated_noise_height_low")]
    PropogatedNoiseHeightLow,
    /// propogated_noise_peak_time_ratio_above_high () { }
    #[strum(serialize = "propogated_noise_peak_time_ratio_above_high")]
    PropogatedNoisePeakTimeRatioAboveHigh,
    /// propogated_noise_peak_time_ratio__below_low () { }
    /// 
    /// **Notice**: two `_` between `ratio` and `below`
    #[strum(serialize = "propogated_noise_peak_time_ratio__below_low")]
    PropogatedNoisePeakTimeRatioBelowLow,
    /// propogated_noise_peak_time_ratio_high () { }
    #[strum(serialize = "propogated_noise_peak_time_ratio_high")]
    PropogatedNoisePeakTimeRatioHigh,
    /// propogated_noise_peak_time_ratio_low () { }
    #[strum(serialize = "propogated_noise_peak_time_ratio_low")]
    PropogatedNoisePeakTimeRatioLow,
    /// propogated_noise_width_above_high () { }
    #[strum(serialize = "propogated_noise_width_above_high")]
    PropogatedNoiseWidthAboveHigh,
    /// propogated_noise_width_below_low () { }
    #[strum(serialize = "propogated_noise_width_below_low")]
    PropogatedNoiseWidthBelowLow,
    /// propogated_noise_width_high () { }
    #[strum(serialize = "propogated_noise_width_high")]
    PropogatedNoiseWidthHigh,
    /// propogated_noise_width_low () { }
    #[strum(serialize = "propogated_noise_width_low")]
    PropogatedNoiseWidthLow,
    /// receiver_capacitance1_fall () { }
    #[strum(serialize = "receiver_capacitance1_fall")]
    ReceiverCapacitance1Fall,
    /// receiver_capacitance1_rise () { }
    #[strum(serialize = "receiver_capacitance1_rise")]
    ReceiverCapacitance1Rise,
    /// receiver_capacitance2_fall () { }
    #[strum(serialize = "receiver_capacitance2_fall")]
    ReceiverCapacitance2Fall,
    /// receiver_capacitance2_rise () { }
    #[strum(serialize = "receiver_capacitance2_rise")]
    ReceiverCapacitance2Rise,
    /// retaining_fall () { }
    #[strum(serialize = "retaining_fall")]
    RetainingFall,
    /// retaining_rise () { }
    #[strum(serialize = "retaining_rise")]
    RetainingRise,
    /// retain_fall_slew () { }
    #[strum(serialize = "retain_fall_slew")]
    RetainFallSlew,
    /// retain_rise_slew () { }
    #[strum(serialize = "retain_rise_slew")]
    RetainRiseSlew,
    /// rise_constraint () { }
    #[strum(serialize = "rise_constraint")]
    RiseConstraint,
    /// rise_propagation () { }
    #[strum(serialize = "rise_propagation")]
    RisePropagation,
    /// rise_transition () { }
    #[strum(serialize = "rise_transition")]
    RiseTransition,
    /// steady_state_current_high () { }
    #[strum(serialize = "steady_state_current_high")]
    SteadyStateCurrentHigh,
    /// steady_state_current_low () { }
    #[strum(serialize = "steady_state_current_low")]
    SteadyStateCurrentLow,
    /// steady_state_current_tristate () { }
    #[strum(serialize = "steady_state_current_tristate")]
    SteadyStateCurrentTristate,
}

/// The `cell_fall` group defines cell delay lookup tables (independently of transition delay) in CMOS nonlinear timing models
/// Reference:
/// <iframe 
/// src="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x8.h5.y292a.ff1.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.xa.hc.y2942.ff7.fs2.fc2.sc0.ls0
/// " 
/// style="width: 90%; height: 600px;"></iframe>
#[readonly::make]
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct CallFall{
    ///
    template_name: CompactString,
    ///
    pub index_1: Vec<f64>,
    ///
    pub index_2: Vec<f64>,
    ///
    pub index_3: Vec<f64>,
    ///
    pub values: Vec<f64>,
}
///
impl Group for CallFall  {
    fn name(&self) -> &CompactString {
        // todo!()
        &self.template_name
    }
}