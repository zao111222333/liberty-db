//! Timing module
//! implement.
//! Demonstrating HTML tables.
//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script> 

use std::collections::HashMap;

mod timing_type;
pub use timing_type::*;

pub mod items;
pub mod impls;
pub mod builder;
#[cfg(test)]
mod test;
use crate::{common::items::*, library::Sensitization, bus::Bus, pin::Pin,expression, units};

/// A timing group is defined in a bundle, a bus, or a pin group within a cell.
/// The timing group can be used to identify the name or names of multiple timing arcs.
/// A timing group identifies multiple timing arcs, by identifying a timing arc in a pin group
/// that has more than one related pin or when the timing arc is part of a bundle or a bus.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =67.26
/// &end
/// =67.43
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =203.8
/// &end
/// =203.29
/// ">Reference-Instatnce-In-Pin</a>
/// 
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
pub struct Timing {
    _undefined: crate::ast::AttributeList,
    /// Use this attribute to indicate that a constraint arc is for
    /// a clock gating relation between the data and clock pin,
    /// instead of a constraint found in standard sequential devices,
    /// such as registers and latches.
    ///
    /// #### Syntax
    /// `clock_gating_flag : bool ; `
    ///
    /// `Boolean`: Valid values are true and false. The value true is applicable
    /// only when the value of the timing_type attribute is setup, hold, or nochange.
    /// When not defined for a timing arc, the value false is assumed,
    /// indicating the timing arc is part of a standard sequential device.
    /// 
    /// #### Example
    /// ``` liberty
    /// clock_gating_flag : true ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =204.47
    /// &end
    /// =204.59
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =320.6
    /// &end
    /// =320.6
    /// ">Reference-Instance</a>
    #[arrti_type(simple)]
    pub clock_gating_flag: Option<bool>,
    /// The `default_timing` attribute allows you to specify one timing arc as the default 
    /// in the case of multiple timing arcs with when statements.
    /// 
    /// #### Syntax
    /// `default_timing : bool ; `
    /// 
    /// #### Example
    /// ``` liberty
    /// default_timing : true ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =205.0
    /// &end
    /// =205.6
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =320.7
    /// &end
    /// =320.7
    /// ">Reference-Instance</a>
    pub default_timing: Option<bool>,
    // /// The `fall_resistance` attribute represents the load-dependent output resistance, 
    // /// or drive capability, for a logic 1-to-0 transition.
    // /// 
    // /// #### Note
    // /// You cannot specify a resistance unit in the library. 
    // /// Instead, the resistance unit is derived from the ratio of the time_unit 
    // /// value to the capacitive_load_unit value.
    // /// 
    // /// #### Syntax
    // /// `fall_resistance : valuefloat ; `
    // /// 
    // /// `value` is a positive floating-point number in terms of delay time per load unit. 
    // /// 
    // /// #### Example
    // /// ``` liberty
    // /// fall_resistance : 0.18 ;
    // /// ```
    // /// <a name ="reference_link" href="
    // /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    // /// ?field=test
    // /// &bgn
    // /// =205.7
    // /// &end
    // /// =205.20
    // /// ">Reference-Definition</a>
    // /// <a name ="reference_link" href="
    // /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    // /// ?field=test
    // /// &bgn
    // /// =203.33
    // /// &end
    // /// =203.33
    // /// ">Reference-Instance</a>
    // pub fall_resistance: Option<units::ElectricalResistance>,
    /// The `fpga_arc_condition` attribute specifies a Boolean condition that enables 
    /// a timing arc.
    /// 
    /// #### Syntax
    /// `fpga_arc_condition : condition(Boolean) ;`
    /// 
    /// `condition` Specifies a Boolean condition. Valid values are true and false.
    /// 
    /// #### Example
    /// ``` liberty
    /// fpga_arc_condition : true;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =205.21
    /// &end
    /// =205.31
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.34
    /// &end
    /// =203.34
    /// ">Reference-Instance</a>
    pub fpga_arc_condition: Option<expression::BooleanExpression>,
    /// Use this attribute to reference a `calc_mode` value in a 
    /// [domain](crate::common::items::Domain) group in a polynomial table. 
    /// 
    /// TODO: `calc_mode`
    /// #### Syntax
    /// `fpga_domain_style : "nameid" ; `
    /// 
    /// `name`: The `calc_mode` value.
    /// 
    /// #### Example
    /// ``` liberty
    /// fpga_domain_style : "speed"; 
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =205.32
    /// &end
    /// =206.0
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.35
    /// &end
    /// =203.35
    /// ">Reference-Instance</a>
    pub fpga_domain_style: Option<String>,
    /// Use pairs of `interdependence_id` attributes to identify interdependent pairs 
    /// of `setup` and `hold` constraint tables. Interdependence data is supported 
    /// in conditional constraint checking, the `interdependence_id` attribute increases 
    /// independently for each condition. Interdependence data can be specified in 
    /// [pin](crate::pin::Pin), [bus](crate::bus::Bus), and [bundle](crate::bundle::Bundle) groups.
    /// 
    /// #### Syntax
    /// `interdependence_id : "nameenum" ;`
    /// 
    /// `name`: Valid values are 1, 2, 3, and so on.
    /// 
    /// #### Examples
    /// ``` liberty
    /// timing()
    ///     related_pin : CLK ;
    ///     timing_type: setup_rising ;
    ///     interdependence_id : 1 ;
    ///     ... 
    /// timing()
    ///     related_pin : CLK ;
    ///     timing_type: setup_rising ;
    ///     interdependence_id : 2 ;
    /// 
    /// ...
    /// pin (D_IN) {
    ///     ...
    ///     /* original non-conditional setup/hold constraints */ 
    ///     setup/hold constraints 
    ///     /* new interdependence data for non-conditional constraint  checking */ 
    ///     setup/hold, interdependent_id = 1  
    ///     setup/hold, interdependent_id = 2  
    ///     setup/hold, interdependent_id = 3   
    ///     
    ///     /* original setup/hold constraints for conditional  <condition_a> */ 
    ///     setup/hold when <condition_a> 
    ///     /* new interdependence data for <condition_a> constraint  checking */ 
    ///     setup/hold when <condition_a>, interdependent_id = 1 
    ///     setup/hold when <condition_a>, interdependent_id = 2 
    ///     setup/hold when <condition_a>, interdependent_id = 3   
    ///     /* original setup/hold constraints for conditional  <condition_b> */ 
    ///     setup/hold when <condition_b> 
    ///     /* new interdependence data for <condition_b> constraint  checking */ 
    ///     setup/hold when <condition_b>, interdependent_id = 1  
    ///     setup/hold when <condition_b>, interdependent_id = 2  
    ///     setup/hold when <condition_b>, interdependent_id = 3 
    /// }
    /// ```
    /// TODO: Need Implement [Check](crate::common::traits::Check)
    /// 
    /// #### Guidelines
    /// + To prevent potential backward-compatibility issues, interdependence data cannot be the first timing arc in the pin group.
    /// + The `interdependence_id` attribute only supports the following timing types: `setup_rising`, `setup_falling`, 
    /// `hold_rising`, and `hold_falling`. If you set this attribute on other timing types, an error is reported.
    /// + You must specify `setup` and `hold` interdependence data in pairs; otherwise an error is reported. 
    /// If you define one `setup_rising` timing arc with ```interdependence_id: 1;``` on a pin, you must also 
    /// define a `hold_rising` timing arc with ```interdependence_id: 1;``` for that pin. The `interdependence_id` could be 
    /// a random integer, but it must be found in a pair of timing arcs. These timing types are considered as 
    /// pairs: `setup_rising` with `hold_rising` and `setup_falling` with `hold_falling`.
    /// + For each set of conditional constraints (non-conditional categorized as a special condition), 
    /// a timing arc with a specific `interdependence_id` should be unique in a pin group.
    /// + For each set of conditional constraints, the `interdependence_id` must start from 1, and if there is multiple
    /// interdependence data defined, the values for the `interdependence_id` should be in consecutive order. 
    /// That is, 1, 2, 3 is allowed, but 1, 2, 4 is not.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =206.1
    /// &end
    /// =207.9
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.36
    /// &end
    /// =203.36
    /// ">Reference-Instance</a>
    pub interdependence_id: Option<usize>,
    /// On an output pin, `intrinsic_fall` defines the 1-to-Z propagation time 
    /// for a three-state-disable timing type and the Z-to-0 propagation time 
    /// for a three-state-enable timing type. 
    /// 
    /// On an input pin, `intrinsic_fall` defines a `setup`, `hold`, or `recovery` 
    /// timing requirement for a logic 1-to-0 transition. With `intrinsic_rise`, 
    /// `intrinsic_fall` defines timing checks (`rising` and `falling` transitions).
    /// 
    /// #### Syntax
    /// `intrinsic_fall : valuefloat ;`
    /// 
    /// `value`: A floating-point number that represents a timing requirement.
    /// 
    /// #### Example
    /// ``` liberty
    /// intrinsic_fall : 0.75 ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =207.10
    /// &end
    /// =207.24
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.37
    /// &end
    /// =203.37
    /// ">Reference-Instance</a>
    pub intrinsic_fall: Option<units::Time>,
    /// On an output pin, `intrinsic_rise` defines the 0-to-Z propagation time 
    /// for a three-state-disable timing type and a Z-to-1 propagation time 
    /// for a three-state-enable timing type.
    /// 
    /// On an input pin, `intrinsic_rise` defines a `setup`, `hold`, or `recovery` 
    /// timing requirement for a logic 0-to-1 transition. With intrinsic_fall, 
    /// `intrinsic_rise` defines timing checks (`rising` and `falling` transitions). 
    /// 
    /// #### Syntax
    /// `intrinsic_rise : valuefloat ;`
    /// 
    /// `value`: A floating-point number that represents a timing requirement.
    /// 
    /// #### Example
    /// ``` liberty
    /// intrinsic_rise : 0.17 ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =207.25
    /// &end
    /// =207.39
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.38
    /// &end
    /// =203.38
    /// ">Reference-Instance</a>
    pub intrinsic_rise: Option<units::Time>,
    /// The `related_bus_equivalent` attribute generates a single timing arc 
    /// for all paths from points in a group through an internal pin (I) to given endpoints.
    /// 
    /// #### Syntax
    /// `related_bus_equivalent : " name1 [name2 name3 ... ] " ;`
    /// 
    /// #### Example1
    /// ``` liberty
    /// related_bus_equivalent : a ;
    /// ```
    /// #### Example2
    /// ``` liberty
    /// cell(acell) {
    ///     ...
    ///     bus(y) {
    ///         bus_type : bus4;
    ///         direction : output;
    ///         timing() {
    ///             related_bus_equivalent : a;
    ///             ...
    ///         }
    ///     }
    ///     bus(a) {
    ///         bus_type : bus4;
    ///         direction : input;
    ///         ...
    ///     }
    /// }
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =207.40
    /// &end
    /// =208.18
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.39
    /// &end
    /// =203.39
    /// ">Reference-Instance</a>
    pub related_bus_equivalent: Vec<Box<Pin>>,
    /// The `related_bus_pins` attribute defines the pin or pins that 
    /// are the startpoint of the timing arc. The primary use of 
    /// `related_bus_pins` is for module generators. 
    /// 
    /// TODO: Need Implement [Check](crate::common::traits::Check)
    /// 
    /// #### Note
    /// When a `related_bus_pins` attribute is within a timing group, 
    /// the timing group must be within a [bus](crate::bus::Bus) or 
    /// [bundle](crate::bundle::Bundle) group. 
    /// 
    /// #### Syntax
    /// `related_bus_pins : " name1 [name2 name3 ... ] " ;`.
    /// 
    /// #### Example
    /// ``` liberty
    /// related_bus_pins : "A" ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =208.19
    /// &end
    /// =208.28
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.40
    /// &end
    /// =203.40
    /// ">Reference-Instance</a>
    pub related_bus_pins: Vec<Box<Pin>>,
    /// The `related_output_pin` attribute specifies the output or inout pin used 
    /// to describe a load-dependent constraint. This is an attribute in the timing group 
    /// of the output or inout pin. The pin defined must be a pin in the same cell, 
    /// and its direction must be either output or inout.
    /// 
    /// TODO: Need Implement [Check](crate::common::traits::Check), output or inout pin
    /// 
    /// #### Syntax
    /// `related_output_pin : name ;`.
    /// 
    /// #### Example
    /// ``` liberty
    /// related_output_pin : Z ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =208.29
    /// &end
    /// =208.37
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.41
    /// &end
    /// =203.41
    /// ">Reference-Instance</a>
    pub related_output_pin: Option<Box<Pin>>,
    /// The `related_pin` attribute defines the pin or pins representing 
    /// the beginning point of the timing arc. It is required in all timing groups.
    /// 
    /// #### Syntax
    /// `related_pin : "name1 [name2 name3 ... ]" ;` 
    /// 
    /// #### Example1
    /// In a cell with input pin A and output pin B, define A and its relationship 
    /// to B in the `related_pin` attribute statement in the timing group that describes pin B.
    /// ``` liberty
    /// pin (B){  
    ///     direction : output ;  
    ///     function : "A’";  
    ///     timing () {  
    ///         related_pin : "A" ;  
    ///         ... 
    ///         timing information 
    ///         ...  
    ///     }
    /// }
    /// ```
    /// 
    /// #### Example2
    /// The `related_pin` attribute statement can also serve as a shortcut 
    /// for two identical timing arcs for a cell. For example, in a 2-input NAND gate
    /// with identical delays from both input pins to the output pin, 
    /// it is necessary to define only one timing arc with two related pins.
    /// ``` liberty
    /// pin (Z) {  
    ///     direction : output;  
    ///     function : "(A * B)’" ;  
    ///     timing () {  
    ///         related_pin : "A B" ;  
    ///         ... 
    ///         timing information 
    ///         ...  
    ///     } 
    /// }
    /// ```
    /// 
    /// #### Example3
    /// When a bus name appears in a `related_pin` attribute, the bus members or 
    /// range of members is distributed across all members of the parent bus. 
    /// The width of the bus or the range must be the same as the width of the parent bus. 
    /// Pin names used in a `related_pin` statement can start with a nonalphabetic character.
    /// ``` liberty
    /// related_pin : "A 1B 2C" ;
    /// ```
    /// ##### Note
    /// It is not necessary to use the escape character, `\` (backslash), with nonalphabetic characters.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =208.38
    /// &end
    /// =209.31
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.42
    /// &end
    /// =203.42
    /// ">Reference-Instance</a>
    pub related_pin: Vec<Box<Pin>>,
    /// The `rise_resistance` attribute represents the load-dependent output resistance, 
    /// or drive capability, for a logic 0-to-1 transition. 
    /// 
    /// #### Note
    /// You cannot specify a resistance unit in the library. 
    /// Instead, the resistance unit is derived from the ratio of the `time_unit` value 
    /// to the `capacitive_load_unit` value.
    /// 
    /// #### Syntax
    /// `rise_resistance : valuefloat ;`,
    /// 
    /// `value`: A positive floating-point number in terms of delay time per load unit. 
    /// 
    /// #### Example
    /// ``` liberty
    /// rise_resistance : 0.15 ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =209.32
    /// &end
    /// =209.45
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.43
    /// &end
    /// =203.43
    /// ">Reference-Instance</a>
    pub rise_resistance: Option<units::ElectricalResistance>,
    /// The `sdf_cond` attribute is defined in the state-dependent timing group 
    /// to support SDF file generation and condition matching during back-annotation.
    /// #### Syntax
    /// 
    /// `sdf_cond : "SDF expression" ;`
    /// 
    /// `SDF expression`: A string that represents a Boolean description of the 
    /// state dependency of the delay. Use a Boolean description that conforms to 
    /// the valid syntax defined in the OVI SDF, which is different from the 
    /// Synopsys Boolean expression syntax. For a complete description of the 
    /// valid syntax for these expressions, refer to the OVI specification for SDF, v1.0.
    /// 
    /// #### Example
    /// ``` liberty
    /// sdf_cond : "b == 1’b1" ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =209.46
    /// &end
    /// =210.9
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.44
    /// &end
    /// =203.44
    /// ">Reference-Instance</a>
    pub sdf_cond: Option<SdfExpression>,
    /// The `sdf_cond_end` attribute defines a timing-check condition specific 
    /// to the end event in VHDL models. The expression must conform to 
    /// `OVI SDF 2.1 timing-check condition syntax`.
    /// 
    /// #### Syntax 
    /// `sdf_cond_end : "SDF expression" ;`
    /// 
    /// `SDF expression`: An SDF expression containing names of input, output, inout, and internal pins.
    /// 
    /// #### Example
    /// ``` liberty
    /// sdf_cond_end : "SIG_0 == 1’b1" ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =210.10
    /// &end
    /// =210.19
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.45
    /// &end
    /// =203.45
    /// ">Reference-Instance</a>
    pub sdf_cond_end: Option<SdfExpression>,
    /// The `sdf_cond_start` attribute defines a timing-check condition specific 
    /// to the start event in full-timing gate-level simulation (FTGS) models. 
    /// The expression must conform to `OVI SDF 2.1 timing-check condition syntax`.
    /// 
    /// #### Syntax 
    /// `sdf_cond_start : "SDF expression" ;` 
    /// 
    /// `SDF expression`: An SDF expression containing names of 
    /// input, output, inout, and internal pins.
    /// 
    /// #### Example
    /// ``` liberty
    /// sdf_cond_start : "SIG_2 == 1’b1" ; 
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =210.20
    /// &end
    /// =210.30
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.46
    /// &end
    /// =203.46
    /// ">Reference-Instance</a>
    pub sdf_cond_start: Option<SdfExpression>,
    /// The `sdf_edges` attribute defines the edge specification on both 
    /// the start pin and the end pin. The default is noedge. 
    /// 
    /// #### Syntax
    /// `sdf_edges : sdf_edge_type;`
    /// 
    /// `sdf_edge_type`: One of these four edge types: `noedge`, `start_edge`, 
    /// `end_edge`, or `both_edges`. The default is `noedge`.
    /// 
    /// #### Example
    /// ``` liberty
    /// sdf_edges : both_edges; 
    /// sdf_edges : start_edge ; /* edge specification on starting pin */ 
    /// sdf_edges : end_edge ; /* edge specification on end pin */
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =210.31
    /// &end
    /// =211.2
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.47
    /// &end
    /// =203.47
    /// ">Reference-Instance</a>
    pub sdf_edges: SdfEdgeType,
    /// FIXME: Can Not find instance in `timing`, only find definition
    /// 
    /// The `sensitization_master` attribute defines the `sensitization` group 
    /// specific to the current timing group to generate stimulus for characterization. 
    /// The attribute is optional when the sensitization master used for 
    /// the timing arc is the same as that defined in the current cell. 
    /// It is required when they are different. Any sensitization group name 
    /// predefined in the current library is a valid attribute value.
    /// 
    /// #### Syntax
    /// `sensitization_master : sensitization_group_name;`
    /// 
    /// `sensitization_group_name`: A string identifying the sensitization 
    /// group name predefined in the current library.
    /// 
    /// #### Example
    /// ``` liberty
    /// sensitization_master : sensi_2in_1out;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =211.3
    /// &end
    /// =211.15
    /// ">Reference-Definition</a>
    pub sensitization_master: Option<Sensitization>,
    /// The `slope_fall` attribute represents the incremental delay 
    /// to add to the slope of the input waveform for a logic 1-to-0 transition.
    /// 
    /// #### Syntax
    /// `slope_fall : valuefloat ;`
    /// 
    /// `value`: A positive floating-point number multiplied by the transition 
    /// delay resulting in slope delay.
    /// 
    /// #### Example
    /// ``` liberty
    /// slope_fall : 0.8 ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =211.16
    /// &end
    /// =211.27
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.48
    /// &end
    /// =203.48
    /// ">Reference-Instance</a>
    pub slope_fall: Option<units::Time>,
    /// The `slope_rise` attribute represents the incremental delay 
    /// to add to the slope of the input waveform for a logic 0-to-1 transition.
    /// 
    /// #### Syntax
    /// `slope_rise : valuefloat ;`
    /// 
    /// `value`: A positive floating-point number multiplied by the 
    /// transition delay resulting in slope delay.
    /// 
    /// #### Example
    /// ``` liberty
    /// slope_rise : 1.0 ; 
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =211.28
    /// &end
    /// =211.39
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.49
    /// &end
    /// =203.49
    /// ">Reference-Instance</a>
    pub slope_rise: Option<units::Time>,
    /// The `steady_state_resistance_above_high` attribute specifies a 
    /// steady-state resistance value for a region of a current-voltage (I-V) curve 
    /// when the output is high and the noise is over the high voltage rail.
    /// 
    /// #### Syntax
    /// `steady_state_resistance_above_high : valuefloat ;`
    /// 
    /// `value`: A positive floating-point number that represents the resistance. 
    /// The resistance unit is a function of the unit of time divided by 
    /// the library unit of capacitance.
    /// 
    /// #### Example
    /// ``` liberty
    /// steady_state_resistance_above_high : 200 ; 
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =211.40
    /// &end
    /// =212.8
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.50
    /// &end
    /// =203.50
    /// ">Reference-Instance</a>
    pub steady_state_resistance_above_high: Option<units::ElectricalResistance>,
    /// The `steady_state_resistance_below_low` attribute specifies a steady-state 
    /// resistance value for a region of a current-voltage (I-V) curve 
    /// when the output is low and the noise is below the low voltage rail.
    /// 
    /// #### Syntax
    /// `steady_state_resistance_below_low : valuefloat ;`
    /// 
    /// `value`: A positive floating-point number that represents the resistance. 
    /// The resistance unit is a function of the unit of time divided by 
    /// the library unit of capacitance.
    /// 
    /// #### Example
    /// ``` liberty
    /// steady_state_resistance_below_low : 100 ; 
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =212.9
    /// &end
    /// =212.22
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.51
    /// &end
    /// =203.51
    /// ">Reference-Instance</a>
    pub steady_state_resistance_below_low: Option<units::ElectricalResistance>,
    /// The `steady_state_resistance_high` attribute specifies a steady-state 
    /// resistance value for a region of a current-voltage (I-V) curve when 
    /// the output is high and the noise is below the high voltage rail.
    /// 
    /// #### Syntaxs
    /// `teady_state_resistance_high : valuefloat ;`
    /// 
    /// `value`: A positive floating-point number that represents the resistance.
    /// The resistance unit is a function of the unit of time divided by 
    /// the library unit of capacitance.
    /// 
    /// #### Example
    /// ``` liberty
    /// steady_state_resistance_high : 1500 ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =212.23
    /// &end
    /// =212.36
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.52
    /// &end
    /// =203.52
    /// ">Reference-Instance</a>
    pub steady_state_resistance_high: Option<units::ElectricalResistance>,
    /// The `steady_state_resistance_low` attribute specifies a steady-state 
    /// resistance value for a region of a current-voltage (I-V) curve 
    /// when the output is low and the noise is over the low voltage rail.
    /// 
    /// #### Syntax
    /// 
    /// `steady_state_resistance_low : valuefloat ;`
    /// 
    /// `value`: A positive floating-point number that represents the resistance. 
    /// The resistance unit is a function of the unit of time divided by 
    /// the library unit of capacitance.
    /// 
    /// #### Example
    /// ``` liberty
    /// steady_state_resistance_low : 1100 ; 
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =212.37
    /// &end
    /// =213.1
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.53
    /// &end
    /// =203.53
    /// ">Reference-Instance</a>
    pub steady_state_resistance_low: Option<units::ElectricalResistance>,
    /// Used for noise modeling, the `tied_off` attribute allows you 
    /// to specify the I-V characteristics and steady-state resistance values 
    /// on tied-off cells. 
    /// 
    /// #### Syntax
    /// 
    /// `tied_off : Boolean ;`
    /// 
    /// `Boolean`: Valid values are true and false.
    /// 
    /// #### Example
    /// ``` liberty
    /// tied_off : true ;
    /// ```
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =213.2
    /// &end
    /// =213.10
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.54
    /// &end
    /// =203.54
    /// ">Reference-Instance</a>
    pub tied_off: Option<bool>,
    /// The `timing_sense` attribute describes the way an input pin logically affects an output pin.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =213.11
    /// &end
    /// =214.6
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.55
    /// &end
    /// =203.55
    /// ">Reference-Instance</a>
    /// #### Syntax 
    /// `timing_sense : positive_unate | negative_unate | non_unate ;`
    /// 
    /// `positive_unate`: Combines incoming rise delays with local rise delays and 
    /// compares incoming fall delays with local fall delays.
    /// 
    /// `negative_unate`: Combines incoming rise delays with local fall delays and 
    /// compares incoming fall delays with local rise delays.
    /// 
    /// `non_unate`: Combines local delays with the worst-case incoming delay value.
    /// The non-unate timing sense represents a function whose output value change cannot 
    /// be determined from the direction of the change in the input value.
    /// 
    /// Timing sense is derived from the logic function of a pin. For example, the value derived for 
    /// an AND gate is `positive_unate`, the value for a NAND gate is `negative_unate`, and the value 
    /// for an XOR gate is `non_unate`.
    /// 
    /// A function is said to be unate if a rising (falling) change on a positive (negative) unate 
    /// input variable causes the output function variable to rise (fall) or not change. 
    /// For a non-unate variable, further state information is required to determine the effects of 
    /// a particular state transition.
    /// 
    /// You can specify half-unate sequential timing arcs if the `timing_type` value is either 
    /// `rising_edge` or `falling_edge` and the `timing_sense` value is either `positive_unate` 
    /// or `negative_unate`.
    /// + In the case of `rising_edge` and `positive_unate` values, only the `cell_rise` and `rise_transition` 
    /// information is required.
    /// + In the case of `rising_edge` and `negative_unate` values, only the `cell_fall` and `fall_transition` 
    /// information is required.
    /// + In the case of `falling_edge` and `positive_unate` values, only the `cell_rise` and `rise_transition` 
    /// information is required.
    /// + In the case of `falling_edge` and `negative_unate` values, only the `cell_fall` and `fall_transition`
    /// information is required.
    /// 
    /// Do not define the `timing_sense` value of a pin, except when you need to override the derived value 
    /// or when you are characterizing a noncombinational gate such as a three-state component. For example, 
    /// you might want to define the timing sense manually when you model multiple paths between 
    /// an input pin and an output pin, such as in an XOR gate.
    /// 
    /// It is possible that one path is positive unate while another is negative unate. In this case, 
    /// the first timing arc is given a `positive_unate` designation and the second is given a `negative_unate` 
    /// designation.
    /// 
    /// Timing arcs with a timing type of `clear` or `preset` require a `timing_sense` attribute. 
    /// If `related_pin` is an output pin, you must define a `timing_sense`` attribute for that pin.
    pub timing_sense: Option<items::TimingSenseType>,
    /// The `timing_type` attribute distinguishes between combinational
    /// and sequential cells by defining the type of timing arc.
    /// If this attribute is not assigned, the cell is considered combinational.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =214.7
    /// +214.28
    /// +214.50
    /// +216.63
    /// +217.19
    /// &end
    /// =214.27
    /// +214.49
    /// +216.60
    /// +217.18
    /// +217.35
    /// ">Reference-Difinition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.56
    /// &end
    /// =203.70
    /// ">Reference-Instance</a>
    /// #### Syntax
    /// `timing_type : combinational | combinational_rise | combinational_fall | three_state_disable | 
    /// three_state_disable_rise | three_state_disable_fall | three_state_enable | three_state_enable_rise | 
    /// three_state_enable_fall | rising_edge | falling_edge | preset | clear | hold_rising | hold_falling | 
    /// setup_rising | setup_falling | recovery_rising | recovery_falling | skew_rising | skew_falling | 
    /// removal_rising | removal_falling | min_pulse_width | minimum_period | max_clock_tree_path |
    /// min_clock_tree_path |non_seq_setup_rising | non_seq_setup_falling | non_seq_hold_rising | 
    /// non_seq_hold_falling | nochange_high_high | nochange_high_low | nochange_low_high | nochange_low_low ;`
    /// 
    /// #### Combinational Timing Arcs
    /// The timing type and timing sense define the signal propagation pattern. The default timing type is combinational.
    /// <table>
    ///     <thead>
    ///         <tr>
    ///             <th>Timing type</th>
    ///             <th></th>
    ///             <th>Timing sense</th>
    ///             <th></th>
    ///         </tr>
    ///         <tr>
    ///             <th></th>
    ///             <th><code>positive_unate</code></th>
    ///             <th><code>negative_unate</code></th>
    ///             <th><code>non_unate</code></th>
    ///         </tr>
    ///     </thead>
    ///     <tbody>
    ///         <tr>
    ///             <td><code>combinational</code></td>
    ///             <td>R->R, F->F</td>
    ///             <td>R->F, F->R</td>
    ///             <td>{R,F}->{R,F}</td>
    ///         </tr>
    ///         <tr>
    ///             <td><code>combinational_rise</code></td>
    ///             <td>R->R</td>
    ///             <td>F->R</td>
    ///             <td>{R,F}->R</td>
    ///         </tr>
    ///         <tr>
    ///             <td><code>combinational_fall</code></td>
    ///             <td>F->F</td>
    ///             <td>R->F</td>
    ///             <td>{R,F}->F</td>
    ///         </tr>
    ///         <tr>
    ///             <td><code>three_state_disable</code></td>
    ///             <td>R->{0Z,1Z}</td>
    ///             <td>F->{0Z,1Z}</td>
    ///             <td>{R,F}->{0Z,1Z}</td>
    ///         </tr>
    ///         <tr>
    ///             <td><code>three_state_enable</code></td>
    ///             <td>R->{Z0,Z1}</td>
    ///             <td>F->{Z0,Z1}</td>
    ///             <td>{R,F}->{Z0,Z1}</td>
    ///         </tr>
    ///         <tr>
    ///             <td><code>three_state_disable_rise</code></td>
    ///             <td>R->0Z</td>
    ///             <td>F->0Z</td>
    ///             <td>{R,F}->0Z</td>
    ///         </tr>
    ///         <tr>
    ///             <td><code>three_state_disable_fall</code></td>
    ///             <td>R->1Z</td>
    ///             <td>F->1Z</td>
    ///             <td>{R,F}->1Z</td>
    ///         </tr>
    ///         <tr>
    ///             <td><code>three_state_enable_rise</code></td>
    ///             <td>R->Z1</td>
    ///             <td>F->Z1</td>
    ///             <td>{R,F}->Z1</td>
    ///         </tr>
    ///         <tr>
    ///             <td><code>three_state_enable_fall</code></td>
    ///             <td>R->Z0</td>
    ///             <td>F->Z0</td>
    ///             <td>{R,F}->Z0</td>
    ///         </tr>
    ///     </tbody>
    /// </table>
    /// 
    /// #### Sequential Timing Arcs
    /// + `rising_edge`: Identifies a timing arc whose output pin is sensitive to 
    /// a rising signal at the input pin.
    /// + `falling_edge`: Identifies a timing arc whose output pin is sensitive to 
    /// a falling signal at the input pin.
    /// + `preset`: Preset arcs affect only the rise arrival time of the arc’s endpoint pin. 
    /// A preset arc implies that you are asserting a logic 1 on the output pin when 
    /// the designated `related_pin` is asserted.
    /// + `clear`: Clear arcs affect only the fall arrival time of the arc’s endpoint pin.
    /// A clear arc implies that you are asserting a logic 0 on the output pin when 
    /// the designated `related_pin` is asserted.
    /// + `hold_rising`: Designates the rising edge of the related pin for the hold check.
    /// + `hold_falling`: Designates the falling edge of the related pin for the hold check.
    /// + `setup_rising`: Designates the rising edge of the related pin for the setup check on clocked elements.
    /// + `setup_falling`: Designates the falling edge of the related pin for the setup check on clocked elements.
    /// + `recovery_rising`: Uses the rising edge of the related pin for the recovery time check.
    /// The clock is rising-edge-triggered.
    /// + `recovery_falling`: Uses the falling edge of the related pin for the recovery time check. 
    /// The clock is falling-edge-triggered.
    /// + `skew_rising`: The timing constraint interval is measured from the rising edge of 
    /// the reference pin (specified in `related_pin`) to a transition edge of the parent pin of 
    /// the timing group. The `intrinsic_rise` value is the maximum skew time between the reference pin rising 
    /// and the parent pin rising. The `intrinsic_fall` value is the maximum skew time between the reference pin rising 
    /// and the parent pin falling.
    /// + `skew_falling`: The timing constraint interval is measured from the falling edge of 
    /// the reference pin (specified in `related_pin`) to a transition edge of the parent pin of 
    /// the timing group. The `intrinsic_rise` value is the maximum skew time between the reference pin falling 
    /// and the parent pin rising. The `intrinsic_fall` value is the maximum skew time between the reference pin falling 
    /// and the parent pin falling.
    /// + `removal_rising`: Used when the cell is a low-enable latch or a rising-edge-triggered flip-flop. 
    /// For active-low asynchronous control signals, define the removal time with the `intrinsic_rise` attribute. 
    /// For active-high asynchronous control signals, define the removal time with the `intrinsic_fall` attribute.
    /// + `removal_falling`: Used when the cell is a high-enable latch or a falling-edge-triggered flip-flop. 
    /// For active-low asynchronous control signals, define the removal time with the `intrinsic_rise` attribute. 
    /// For active-high asynchronous control signals, define the removal time with the `intrinsic_fall` attribute.
    /// + `minimum_pulse_width`: This value, together with the `minimum_period` value, lets you specify 
    /// the minimum pulse width for a clock pin. The timing check is performed on the pin itself, 
    /// so the related pin should be the same. As with other timing checks, you can include rise and fall constraints. 
    /// + `minimum_period`: This value, together with the `minimum_pulse_width` value, lets you specify 
    /// the minimum pulse width for a clock pin. The timing check is performed on the pin itself, 
    /// so the related pin should be the same. As with other timing checks, you can include rise and fall constraints.
    /// + `max_clock_tree_path`: Used in timing groups under a clock pin. Defines the maximum clock tree path constraint.
    /// + `min_clock_tree_path`: Used in timing groups under a clock pin. Defines the minimum clock tree path constraint.
    /// #### Example
    /// A sample library with the timing_type attribute and minimum_pulse_width and minimum_period values.
    /// ``` liberty
    /// library(ASIC) {  
    ///     ...  
    ///     delay_model : table_lookup;  
    ///     ...  
    ///     lu_table_template(pulse_width_template) {  
    ///         variable_1 : related_pin_transition;  
    ///         index_1 ("1.0, 2.0, 3.0");  
    ///     }  
    ///     cell(flop) {  
    ///         ...   
    ///         pin(CK) {  
    ///             direction : input;  
    ///             capacitance : 0.00707171;  
    ///             timing() {  
    ///                 timing_type : "min_pulse_width";  
    ///                 related_pin : "CK";  
    ///                 ...  
    ///                 rise_constraint("pulse_width_template") {  
    ///                     index_1("0.000000, 1.000000, 2.00000");  
    ///                     values ("6.000000, 6.250000, 7.2500000" );  
    ///                 }  
    ///                 fall_constraint("pulse_width_template") {  
    ///                     index_1("0.000000, 1.000000, 2.00000");  
    ///                     values ("6.000000, 6.250000, 7.2500000" );  
    ///                 }  
    ///             }  
    ///             timing() {  
    ///                 timing_type : "minimum_period";  
    ///                 related_pin : "CK";  
    ///                 rise_constraint("pulse_width_template") {  
    ///                     index_1(" .000000, 1.000000, 2.00000");  
    ///                     values ("6.000000, 6.250000, 7.2500000" );  
    ///                 }  
    ///                 fall_constraint("pulse_width_template") {  
    ///                     index_1("0.000000, 1.000000, 2.00000");  
    ///                     values ("6.000000, 6.250000, 7.2500000" );  
    ///                 }  
    ///             }  
    ///         }  
    ///     ...  
    ///     } /* end cell */ 
    /// } /* end library */
    /// ```
    /// #### Nonsequential Timing Arcs
    /// In some nonsequential cells, the setup and hold timing constraints are specified on
    /// the data pin with a nonclock pin as the `related_pin`. It requires the signal of 
    /// a pin to be stable for a specified period of time before and after another pin of 
    /// the same cell range state so that the cell can function as expected.
    /// + `non_seq_setup_rising`: Defines (with non_seq_setup_falling) the timing arcs used 
    /// for setup checks between pins with nonsequential behavior. The related pin in 
    /// a timing arc is used for the timing check. 
    /// + `non_seq_setup_falling`: Defines (with non_seq_setup_rising) the timing arcs used 
    /// for setup checks between pins with nonsequential behavior. The related pin in 
    /// a timing arc is used for the timing check.
    /// + `non_seq_hold_rising`: Defines (with non_seq_hold_falling) the timing arcs used 
    /// for hold checks between pins with nonsequential behavior. The related pin in 
    /// a timing arc is used for the timing check.
    /// + `non_seq_hold_falling`: Defines (with non_seq_hold_rising) the timing arcs used 
    /// for hold checks between pins with nonsequential behavior. The related pin in 
    /// a timing arc is used for the timing check.
    /// #### No-Change Timing Arcs
    /// This feature models the timing requirement of latch devices with latch-enable signals. 
    /// The four no-change timing types define the pulse waveforms of both the constrained 
    /// signal and the related signal in standard CMOS and nonlinear CMOS delay models. 
    /// The information is used in static timing verification during synthesis.
    /// + `nochange_high_high` (positive/positive): Indicates a positive pulse on 
    /// the constrained pin and a positive pulse on the related pin.
    /// + `nochange_high_low` (positive/negative): Indicates a positive pulse on 
    /// the constrained pin and a negative pulse on the related pin.
    /// + `nochange_low_high` (negative/positive): Indicates a negative pulse on 
    /// the constrained pin and a positive pulse on the related pin.
    /// + `nochange_low_low` (negative/negative): Indicates a negative pulse on 
    /// the constrained pin and a negative pulse on the related pin.
    pub timing_type: Option<TimingType>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =150.10
    /// &end
    /// =150.16
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =203.71
    /// &end
    /// =203.71
    /// ">Reference-Instance</a>
    pub when: Option<expression::BooleanExpression>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =338.12
    /// &end
    /// =338.20
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =204.0
    /// &end
    /// =204.0
    /// ">Reference-Instance</a>
    pub when_end: Option<expression::BooleanExpression>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =338.21
    /// &end
    /// =338.30
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =204.1
    /// &end
    /// =204.1
    /// ">Reference-Instance</a>
    pub when_start: Option<expression::BooleanExpression>,
    // piecewise model only
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =
    /// &end
    /// =
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =204.3
    /// &end
    /// =204.3
    /// ">Reference-Instance</a>
    pub fall_delay_intercept: Option<(i64, f64)>,
    // piecewise model only
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =
    /// &end
    /// =
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =204.4
    /// &end
    /// =204.4
    /// ">Reference-Instance</a>
    pub fall_pin_resistance: Option<(i64, f64)>,
    /// You define the mode attribute within a timing group.
    /// A mode attribute pertains to an individual timing arc.
    /// The timing arc is active when mode is instantiated with a name and a value.
    /// You can specify multiple instances of the mode attribute,
    /// but only one instance for each timing arc.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =219.39
    /// +220.11
    /// &end
    /// =220.9
    /// +222.73
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =204.5
    /// &end
    /// =204.5
    /// ">Reference-Instance</a>
    pub mode: items::Mode,
    // piecewise model only
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =
    /// &end
    /// =
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =204.6
    /// &end
    /// =204.6
    /// ">Reference-Instance</a>
    pub rise_delay_intercept: Option<(i64, f64)>,
    // piecewise model only
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =
    /// &end
    /// =
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =204.7
    /// &end
    /// =204.7
    /// ">Reference-Instance</a>
    pub rise_pin_resistance: Option<(i64, f64)>,
    /// The `cell_degradation` group describes a cell performance degradation
    /// design rule for compiling a design. A cell degradation design rule
    /// specifies the maximum capacitive load a cell can drive without causing
    /// cell performance degradation during the fall transition.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =225.4
    /// +225.27
    /// &end
    /// =225.25
    /// +227.51
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =204.9
    /// &end
    /// =204.9
    /// ">Reference-Instance</a>
    pub cell_degradation: HashMap<String, items::CellDegradation>,
    /// Defines cell delay lookup tables (independently of transition delay) in CMOS nonlinear timing models.
    ///
    /// **Note:**
    /// The same k-factors that scale the cell_fall and cell_rise values also scale the
    /// retaining_fall and retaining_rise values. There are no separate k-factors for
    /// the retaining_fall and retaining_rise values.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =227.53
    /// +228.27
    /// &end
    /// =228.25
    /// +228.62
    /// ">Reference-Definition</a>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =204.10
    /// &end
    /// =204.10
    /// ">Reference-Instance</a>
    pub cell_fall: HashMap<String, items::CellFall>,
    pub cell_rise: HashMap<String, items::CellFall>,
    pub fall_constraint: HashMap<String, items::CellFall>,
    pub fall_propagation: HashMap<String, items::CellFall>,
    pub fall_transition: HashMap<String, items::CellFall>,
    pub noise_immunity_above_high: HashMap<String, items::CellFall>,
    pub noise_immunity_below_low: HashMap<String, items::CellFall>,
    pub noise_immunity_high: HashMap<String, items::CellFall>,
    pub noise_immunity_low: HashMap<String, items::CellFall>,
    pub output_current_fall: HashMap<String, items::CellFall>,
    pub output_current_rise: HashMap<String, items::CellFall>,
    pub propogated_noise_height_above_high: HashMap<String, items::CellFall>,
    pub propogated_noise_height_below_low: HashMap<String, items::CellFall>,
    pub propogated_noise_height_high: HashMap<String, items::CellFall>,
    pub propogated_noise_height_low: HashMap<String, items::CellFall>,
    pub propogated_noise_peak_time_ratio_above_high: HashMap<String, items::CellFall>,
    pub propogated_noise_peak_time_ratio__below_low: HashMap<String, items::CellFall>,
    pub propogated_noise_peak_time_ratio_high: HashMap<String, items::CellFall>,
    pub propogated_noise_peak_time_ratio_low: HashMap<String, items::CellFall>,
    pub propogated_noise_width_above_high: HashMap<String, items::CellFall>,
    pub propogated_noise_width_below_low: HashMap<String, items::CellFall>,
    pub propogated_noise_width_high: HashMap<String, items::CellFall>,
    pub propogated_noise_width_low: HashMap<String, items::CellFall>,
    pub receiver_capacitance1_fall: HashMap<String, items::CellFall>,
    pub receiver_capacitance1_rise: HashMap<String, items::CellFall>,
    pub receiver_capacitance2_fall: HashMap<String, items::CellFall>,
    pub receiver_capacitance2_rise: HashMap<String, items::CellFall>,
    pub retaining_fall: HashMap<String, items::CellFall>,
    pub retaining_rise: HashMap<String, items::CellFall>,
    pub retain_fall_slew: HashMap<String, items::CellFall>,
    pub retain_rise_slew: HashMap<String, items::CellFall>,
    pub rise_constraint: HashMap<String, items::CellFall>,
    pub rise_propagation: HashMap<String, items::CellFall>,
    pub rise_transition: HashMap<String, items::CellFall>,
    pub steady_state_current_high: HashMap<String, items::CellFall>,
    pub steady_state_current_low: HashMap<String, items::CellFall>,
    pub steady_state_current_tristate: HashMap<String, items::CellFall>,
}