use compact_str::CompactString;
use strum_macros::{Display, EnumString};


#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct BooleanExpression{

}
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct SdfExpression{
    
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct SdfEdgeType{
    
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Display, EnumString)]
pub enum VariableType {
    #[strum(serialize = "input_net_transition")]
    InputNetTransition,
    #[strum(serialize = "normalized_voltage")]
    NormalizedVoltage,
    #[strum(serialize = "total_output_net_capacitance")]
    TotalOutputNetCapacitance,
    #[strum(serialize = "related_out_total_output_net_capacitance")]
    RelatedOutTotalOutputNetCapacitance,
    #[strum(serialize = "constrained_pin_transition")]
    ConstrainedPinTransition,
    #[strum(serialize = "fanout_number")]
    FanoutNumber,
    #[strum(serialize = "fanout_pin_capacitance")]
    FanoutPinCapacitance,
    #[strum(serialize = "driver_slew")]
    DriverSlew,
    #[strum(serialize = "input_transition_time")]
    InputTransitionTime,
}