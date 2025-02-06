//! Timing module
//! implement.
//! Demonstrating HTML tables.
//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

mod timing_type;
pub use timing_type::*;
pub mod impls;
pub mod items;
use crate::{
  ast::{Attributes, GroupComments, GroupFn, GroupSet},
  ccsn::PropagatingCcb,
  common::{
    char_config::CharConfig,
    items::{NameList, SdfEdgeType, WordSet},
    table::{
      CompactCcsTable, OcvSigmaTable, ReferenceTimeVector3DGrpup, TableLookUp,
      TableLookUp2D, TableLookUpMultiSegment,
    },
  },
  expression::{BooleanExpression, LogicBooleanExpression, SdfExpression},
  Ctx,
};
pub use items::*;

/// A `timing` group is defined in a `bundle`, a `bus`, or a `pin` group within a `cell`. The `timing`
/// group can be used to identify the name or names of multiple `timing` arcs. A `timing` group
/// identifies multiple `timing` arcs, by identifying a `timing` arc in a `pin` group that has more than
/// one `related pin` or when the timing arc is part of a `bundle` or a `bus`.
/// The following syntax shows a `timing` group in a `pin` group within a `cell` group.
///
/// ### Syntax
/// ``` text
/// library (namestring) {
///   cell (name) {
///     pin (name) {
///       timing (name | name_list) {
///         ... timing description ...
///       }
///     }
///   }
/// }
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=90.29+91.2&end=90.41+91.5
/// ">Reference</a>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Timing: serde::Serialize + serde::de::DeserializeOwned")]
pub struct Timing<C: Ctx> {
  #[size = 24]
  #[liberty(name)]
  pub name: Vec<String>,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  #[size = 0]
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Timing,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
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
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=322.21&end=322.32
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
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
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=322.34+323.2&end=322.37+323.3
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub default_timing: Option<bool>,
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
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=323.5&end=323.14
  /// ">Reference</a>
  #[size = 24]
  #[liberty(simple(type = Option))]
  pub fpga_arc_condition: Option<LogicBooleanExpression>,
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
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=323.16+324.2+325.2&end=323.41+324.49+325.3
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub interdependence_id: Option<usize>,
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
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=293.20&end=293.33
  /// ">Reference</a>
  #[size = 64]
  #[liberty(simple)]
  pub related_bus_pins: WordSet,
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
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=325.26&end=325.33
  /// ">Reference</a>
  #[size = 64]
  #[liberty(simple(type = Option))]
  pub related_output_pin: Option<String>,
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
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=326.3&end=326.38
  /// ">Reference</a>
  #[id(
    borrow = "crate::common::items::RefNameList<'_>",
    check_fn = "NameList::as_ref",
    with_ref = false
  )]
  #[size = 64]
  #[liberty(simple)]
  pub related_pin: NameList,
  /// The `sdf_cond` attribute is defined in the state-dependent timing group to support SDF file
  /// generation and condition matching during back-annotation.
  ///
  /// ### Syntax
  /// ``` text
  /// sdf_cond : "SDF expression" ;
  /// ```
  /// SDF expression
  ///
  /// A string that represents a Boolean description of the state dependency of the
  /// delay. Use a Boolean description that conforms to the valid syntax defined in
  /// the OVI SDF, which is different from the Boolean expression. For a complete
  /// description of the valid syntax for these expressions, see the OVI specification
  /// for SDF, V1.0.
  ///
  /// ### Example
  /// ``` text
  /// sdf_cond : "b == 1’b1" ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=327.3&end=327.14
  /// ">Reference</a>
  #[size = 24]
  #[liberty(simple(type = Option))]
  pub sdf_cond: Option<SdfExpression>,
  /// The `sdf_cond_end` attribute defines a timing-check condition specific to the end event
  /// in VHDL models. The expression must conform to OVI SDF 2.1 timing-check condition
  /// syntax.
  ///
  /// ### Syntax
  /// ``` text
  /// sdf_cond_end : "SDF expression" ;
  /// ```
  /// SDF expression
  ///
  /// An SDF expression containing names of input, output, inout, and internal pins.
  ///
  /// ### Example
  /// ``` text
  /// sdf_cond_end : "SIG_0 == 1’b1" ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=327.16&end=327.24
  /// ">Reference</a>
  #[size = 24]
  #[liberty(simple(type = Option))]
  pub sdf_cond_end: Option<SdfExpression>,
  /// The `sdf_cond_start` attribute defines a timing-check condition specific to the start event
  /// in full-timing gate-level simulation (FTGS) models. The expression must conform to OVI
  /// SDF 2.1 timing-check condition syntax.
  ///
  /// ### Syntax
  /// ``` text
  /// sdf_cond_start : "SDF expression" ;
  /// ```
  /// SDF expression
  ///
  /// An SDF expression containing names of input, output, inout, and internal pins.
  ///
  /// ### Example
  /// ``` text
  /// sdf_cond_start : "SIG_2 == 1’b1" ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=327.26+328.2&end=327.30+328.5
  /// ">Reference</a>
  #[size = 24]
  #[liberty(simple(type = Option))]
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
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=328.7&end=328.17
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub sdf_edges: Option<SdfEdgeType>,
  /// The `timing_sense` attribute describes the way an input pin logically affects an output pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=328.32+329.2+330.2&end=328.33+329.39+330.6
  /// ">Reference</a>
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
  #[size = 11]
  #[liberty(simple(type = Option))]
  #[id(
    borrow = "Option<&TimingSenseType>",
    check_fn = "mut_set::borrow_option!",
    with_ref = false
  )]
  pub timing_sense: Option<TimingSenseType>,
  /// The `timing_type` attribute distinguishes between combinational
  /// and sequential cells by defining the type of timing arc.
  /// If this attribute is not assigned, the cell is considered combinational.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=330.8+331.2+331.21+332.2+333.2+334.2+335.3+336.2&end=330.41+331.19+331.30+332.33+333.37+334.53+335.45+336.22
  /// ">Reference</a>
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
  #[size = 3]
  #[liberty(simple(type = Option))]
  #[id(
    borrow = "Option<&TimingType>",
    check_fn = "mut_set::borrow_option!",
    with_ref = false
  )]
  pub timing_type: Option<TimingType>,
  /// The when attribute is used in state-dependent timing and conditional timing checks.
  ///
  /// Note:
  ///
  /// The when attribute also appears in the `min_pulse_width` group and
  /// the `minimum_period` group (described on `min_pulse_width` Group and
  /// `minimum_period` Group, respectively). Both groups can be placed in pin, bus,
  /// and `bundle` groups. The when attribute also appears in the power, `fall_power`,
  /// and `rise_power` groups.
  ///
  /// For more details, see the “Modeling Power and Electromigration” and “Timing Arcs”
  /// chapters in the Synopsys Liberty User Guide.
  ///
  /// ### Syntax
  /// ``` text
  /// when : "Boolean expression" ;
  /// ```
  /// Boolean expression
  ///
  /// A Boolean expression containing names of input, output, inout, and internal pins.
  ///
  /// ### Example
  /// ``` text
  /// when : "CD * SD" ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=337.12&end=337.26
  /// ">Reference</a>
  #[size = 80]
  #[liberty(simple(type = Option))]
  #[id(
    borrow = "Option<&LogicBooleanExpression>",
    check_fn = "mut_set::borrow_option!",
    with_ref = false
  )]
  pub when: Option<LogicBooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=338.13&end=338.20
  /// ">Reference</a>
  #[size = 32]
  #[liberty(simple(type = Option))]
  pub when_end: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=338.22&end=338.30
  /// ">Reference</a>
  #[size = 32]
  #[liberty(simple(type = Option))]
  pub when_start: Option<BooleanExpression>,
  /// In referenced CCS noise modeling, the `active_input_ccb` attribute lists the active or
  /// switching input_ccb groups of the input pin that do not propagate the noise in the timing
  /// arc or the receiver capacitance load.
  /// You can also specify this attribute in the `receiver_capacitance` group of the input pin.
  ///
  /// ### Syntax
  /// ``` text
  /// active_input_ccb(input_ccb_name1[ , input_ccb_name2, ...]);
  /// ```
  /// ### Example
  /// ``` text
  /// active_input_ccb("A", "B");
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=339.2&end=339.6
  /// ">Reference-Instance</a>
  #[size = 24]
  #[liberty(complex)]
  pub active_input_ccb: Vec<String>,
  /// In referenced CCS noise modeling, the `active_output_ccb` attribute lists the `output_ccb`
  /// groups in the timing arc that drive the output pin, but do not propagate the noise. You must
  /// define both the `output_ccb` and `timing` groups in the same pin group.
  ///
  /// ### Syntax
  /// ``` text
  /// active_output_ccb(output_ccb_name);
  /// ```
  /// ### Example
  /// ``` text
  /// active_input_ccb("CCB_Q2");
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=339.12&end=339.18
  /// ">Reference-Instance</a>
  #[size = 24]
  #[liberty(complex(type = Option))]
  pub active_output_ccb: Option<String>,
  #[size = 16]
  #[liberty(complex(type = Option))]
  pub propagating_ccb: Option<PropagatingCcb>,
  /// You define the mode attribute within a timing group.
  /// A mode attribute pertains to an individual timing arc.
  /// The timing arc is active when mode is instantiated with a name and a value.
  /// You can specify multiple instances of the mode attribute,
  /// but only one instance for each timing arc.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=340.23+341.2+342.2+343.2+344.2&end=340.36+341.53+342.54+343.54+344.46
  /// ">Reference</a>
  #[size = 16]
  #[liberty(complex(type = Option))]
  pub mode: Option<[String; 2]>,
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
  #[size = 1312]
  #[liberty(group)]
  pub char_config: Option<CharConfig<C>>,
  /// The `cell_degradation` group describes a cell performance degradation
  /// design rule for compiling a design. A cell degradation design rule
  /// specifies the maximum capacitive load a cell can drive without causing
  /// cell performance degradation during the fall transition.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=347.33+348.2&end=347.42+348.20
  /// ">Reference</a>
  #[size = 88]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<CellDegradation<C>>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<CellDegradation<C>>::deserialize_with")]
  pub cell_degradation: GroupSet<CellDegradation<C>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=347.33+348.2&end=347.42+348.20
  /// ">Reference</a>
  #[liberty(supergroup(
    cell_rise: Option<TableLookUp2D<C>>,
    ocv_mean_shift_cell_rise: Option<TableLookUp2D<C>>,
    ocv_std_dev_cell_rise: Option<TableLookUp2D<C>>,
    ocv_skewness_cell_rise: Option<TableLookUp2D<C>>,
  ))]
  #[size = 144]
  pub cell_rise: Option<TimingTableLookUp<C>>,
  /// Defines cell delay lookup tables (independently of transition delay) in CMOS nonlinear timing models.
  ///
  /// **Note:**
  /// The same k-factors that scale the cell_fall and cell_rise values also scale the
  /// retaining_fall and retaining_rise values. There are no separate k-factors for
  /// the retaining_fall and retaining_rise values.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=348.22+349.2&end=348.49+349.32
  /// ">Reference</a>
  #[liberty(supergroup(
    cell_fall: Option<TableLookUp2D<C>>,
    ocv_mean_shift_cell_fall: Option<TableLookUp2D<C>>,
    ocv_std_dev_cell_fall: Option<TableLookUp2D<C>>,
    ocv_skewness_cell_fall: Option<TableLookUp2D<C>>,
  ))]
  #[size = 144]
  pub cell_fall: Option<TimingTableLookUp<C>>,
  #[liberty(supergroup(
    rise_transition: Option<TableLookUp2D<C>>,
    ocv_mean_shift_rise_transition: Option<TableLookUp2D<C>>,
    ocv_std_dev_rise_transition: Option<TableLookUp2D<C>>,
    ocv_skewness_rise_transition: Option<TableLookUp2D<C>>,
  ))]
  #[size = 144]
  pub rise_transition: Option<TimingTableLookUp<C>>,
  #[liberty(supergroup(
    fall_transition: Option<TableLookUp2D<C>>,
    ocv_mean_shift_fall_transition: Option<TableLookUp2D<C>>,
    ocv_std_dev_fall_transition: Option<TableLookUp2D<C>>,
    ocv_skewness_fall_transition: Option<TableLookUp2D<C>>,
  ))]
  #[size = 144]
  pub fall_transition: Option<TimingTableLookUp<C>>,
  #[liberty(supergroup(
    rise_constraint: Option<TableLookUp2D<C>>,
    ocv_mean_shift_rise_constraint: Option<TableLookUp2D<C>>,
    ocv_std_dev_rise_constraint: Option<TableLookUp2D<C>>,
    ocv_skewness_rise_constraint: Option<TableLookUp2D<C>>,
  ))]
  #[size = 144]
  pub rise_constraint: Option<TimingTableLookUp<C>>,
  #[liberty(supergroup(
    fall_constraint: Option<TableLookUp2D<C>>,
    ocv_mean_shift_fall_constraint: Option<TableLookUp2D<C>>,
    ocv_std_dev_fall_constraint: Option<TableLookUp2D<C>>,
    ocv_skewness_fall_constraint: Option<TableLookUp2D<C>>,
  ))]
  #[size = 144]
  pub fall_constraint: Option<TimingTableLookUp<C>>,
  #[liberty(supergroup(
    retaining_rise: Option<TableLookUp2D<C>>,
    ocv_mean_shift_retaining_rise: Option<TableLookUp2D<C>>,
    ocv_std_dev_retaining_rise: Option<TableLookUp2D<C>>,
    ocv_skewness_retaining_rise: Option<TableLookUp2D<C>>,
  ))]
  #[size = 144]
  pub retaining_rise: Option<TimingTableLookUp<C>>,
  #[liberty(supergroup(
    retaining_fall: Option<TableLookUp2D<C>>,
    ocv_mean_shift_retaining_fall: Option<TableLookUp2D<C>>,
    ocv_std_dev_retaining_fall: Option<TableLookUp2D<C>>,
    ocv_skewness_retaining_fall: Option<TableLookUp2D<C>>,
  ))]
  #[size = 144]
  pub retaining_fall: Option<TimingTableLookUp<C>>,
  #[liberty(supergroup(
    retain_rise_slew: Option<TableLookUp2D<C>>,
    ocv_mean_shift_retain_rise_slew: Option<TableLookUp2D<C>>,
    ocv_std_dev_retain_rise_slew: Option<TableLookUp2D<C>>,
    ocv_skewness_retain_rise_slew: Option<TableLookUp2D<C>>,
  ))]
  #[size = 144]
  pub retain_rise_slew: Option<TimingTableLookUp<C>>,
  #[liberty(supergroup(
    retain_fall_slew: Option<TableLookUp2D<C>>,
    ocv_mean_shift_retain_fall_slew: Option<TableLookUp2D<C>>,
    ocv_std_dev_retain_fall_slew: Option<TableLookUp2D<C>>,
    ocv_skewness_retain_fall_slew: Option<TableLookUp2D<C>>,
  ))]
  #[size = 144]
  pub retain_fall_slew: Option<TimingTableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub fall_propagation: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub noise_immunity_above_high: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub noise_immunity_below_low: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub noise_immunity_high: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub noise_immunity_low: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub output_current_fall: Option<ReferenceTimeVector3DGrpup<C>>,
  #[size = 216]
  #[liberty(group)]
  pub output_current_rise: Option<ReferenceTimeVector3DGrpup<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_height_above_high: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_height_below_low: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_height_high: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_height_low: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_peak_time_ratio_above_high: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_peak_time_ratio_below_low: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_peak_time_ratio_high: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_peak_time_ratio_low: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_width_above_high: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_width_below_low: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_width_high: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub propogated_noise_width_low: Option<TableLookUp<C>>,
  #[size = 88]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<TableLookUpMultiSegment<C>>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<TableLookUpMultiSegment<C>>::deserialize_with")]
  pub receiver_capacitance_fall: GroupSet<TableLookUpMultiSegment<C>>,
  #[size = 88]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<TableLookUpMultiSegment<C>>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<TableLookUpMultiSegment<C>>::deserialize_with")]
  pub receiver_capacitance_rise: GroupSet<TableLookUpMultiSegment<C>>,
  #[size = 216]
  #[liberty(group)]
  pub receiver_capacitance1_fall: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub receiver_capacitance1_rise: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub receiver_capacitance2_fall: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub receiver_capacitance2_rise: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub rise_propagation: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub steady_state_current_high: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub steady_state_current_low: Option<TableLookUp<C>>,
  #[size = 216]
  #[liberty(group)]
  pub steady_state_current_tristate: Option<TableLookUp<C>>,
  /// The `compact_ccs_rise`  and `compact_ccs_fall`  groups define the compact CCS timing data in the timing arc.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=352.40&end=352.41
  /// ">Reference-Definition</a>
  #[size = 128]
  #[liberty(group)]
  pub compact_ccs_rise: Option<CompactCcsTable<C>>,
  /// The `compact_ccs_rise`  and `compact_ccs_fall`  groups define the compact CCS timing data in the timing arc.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=352.40&end=352.41
  /// ">Reference-Definition</a>
  #[size = 168]
  #[liberty(group)]
  pub compact_ccs_fall: Option<CompactCcsTable<C>>,
  #[size = 169]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<OcvSigmaTable<C>>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<OcvSigmaTable<C>>::deserialize_with")]
  pub ocv_sigma_cell_fall: GroupSet<OcvSigmaTable<C>>,
  #[size = 169]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<OcvSigmaTable<C>>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<OcvSigmaTable<C>>::deserialize_with")]
  pub ocv_sigma_cell_rise: GroupSet<OcvSigmaTable<C>>,
  #[size = 168]
  #[liberty(group)]
  pub ocv_sigma_fall_constraint: Option<TableLookUp2D<C>>,
  #[size = 169]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<OcvSigmaTable<C>>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<OcvSigmaTable<C>>::deserialize_with")]
  pub ocv_sigma_fall_transition: GroupSet<OcvSigmaTable<C>>,
  #[size = 168]
  #[liberty(group)]
  pub ocv_sigma_rise_constraint: Option<TableLookUp2D<C>>,
  #[size = 169]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<OcvSigmaTable<C>>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<OcvSigmaTable<C>>::deserialize_with")]
  pub ocv_sigma_rise_transition: GroupSet<OcvSigmaTable<C>>,
  #[size = 168]
  #[liberty(group)]
  pub ocv_sigma_retaining_fall: Option<TableLookUp2D<C>>,
  #[size = 168]
  #[liberty(group)]
  pub ocv_sigma_retaining_rise: Option<TableLookUp2D<C>>,
  #[size = 168]
  #[liberty(group)]
  pub ocv_sigma_retain_fall_slew: Option<TableLookUp2D<C>>,
  #[size = 168]
  #[liberty(group)]
  pub ocv_sigma_retain_rise_slew: Option<TableLookUp2D<C>>,
}

impl<C: Ctx> GroupFn for Timing<C> {
  fn after_build(&mut self, _: &mut crate::ast::BuilderScope) {
    impls::need_timing_sense_when_timing_type_is_clear_or_preset(self);
    impls::need_timing_sense_when_related_pin_is_output(self);
  }
}

#[cfg(test)]
mod test {
  use crate::DefaultCtx;

  use super::*;

  #[test]
  fn active_ccb() {
    use crate::ast::GroupAttri;
    _ = crate::ast::test_parse_fmt::<Timing<DefaultCtx>>(
      r#"(){
        active_input_ccb(XOR4D4BWP30P140__nci_14_3_FRLR_RFLF:a4, XOR4D4BWP30P140__nci_15_1_FR_RF:a4);
        active_output_ccb(XOR4D4BWP30P140__nco_0_0_FR_RF:z);
      }"#,
      r#"
liberty_db::timing::Timing () {
| active_input_ccb (XOR4D4BWP30P140__nci_14_3_FRLR_RFLF:a4, XOR4D4BWP30P140__nci_15_1_FR_RF:a4);
| active_output_ccb (XOR4D4BWP30P140__nco_0_0_FR_RF:z);
}"#,
    )
  }
  #[test]
  fn lvf() {
    use crate::ast::GroupAttri;
    _ = crate::ast::test_parse_fmt::<Timing<DefaultCtx>>(
      r#"(){
        cell_rise(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.0109093, 0.013212699999999999, 0.0170902, 0.0243175, 0.0386197, 0.0672009, 0.124553, 0.238683", \
            "0.014126599999999998, 0.0164202, 0.0203034, 0.027543899999999996, 0.041853, 0.0704022, 0.12772599999999998, 0.24214499999999997", \
            "0.0184073, 0.0209574, 0.025013099999999996, 0.032315399999999994, 0.0466268, 0.07517900000000001, 0.132412, 0.24681999999999996", \
            "0.022354499999999992, 0.025422699999999996, 0.029874099999999997, 0.037371999999999996, 0.051819, 0.080353, 0.13749199999999998, 0.251644", \
            "0.025092999999999987, 0.028996999999999974, 0.034324, 0.04236999999999998, 0.05695599999999999, 0.08568699999999999, 0.14268599999999998, 0.25732099999999997", \
            "0.025117999999999994, 0.030123000000000018, 0.036904999999999986, 0.046551000000000016, 0.062202000000000014, 0.09141900000000001, 0.14870399999999998, 0.26288900000000004", \
            "0.018074000000000055, 0.02447400000000005, 0.03333700000000009, 0.04577000000000004, 0.06415900000000001, 0.09573000000000004, 0.155087, 0.270079", \
            "-0.004859999999999919, 0.0033100000000000638, 0.015199999999999905, 0.032079999999999956, 0.05591999999999994, 0.09310999999999989, 0.15792999999999988, 0.27812999999999993" \
          ) ;
        }
      
        ocv_mean_shift_cell_rise(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.00006865999999999982, 0.00009142999999999974, 0.0001397199999999989, 0.00024071999999999616, 0.00045344999999999056, 0.0008832399999999973, 0.001651700000000014, 0.0033134999999999905", \
            "0.000045910000000000155, 0.00006773999999999704, 0.0001139300000000005, 0.00021833999999999843, 0.00041263000000000123, 0.0008710000000000013, 0.001638899999999992, 0.0032351999999999494", \
            "0.00003997999999999878, 0.00005817000000000115, 0.00010011999999999332, 0.00020659000000000102, 0.00038191999999999317, 0.00087659999999999, 0.0016924000000000116, 0.0031957000000000244", \
            "0.0001494400000000006, 0.00015760000000000651, 0.00022443999999999776, 0.00031186999999999713, 0.00045900000000000194, 0.0008858000000000146, 0.001915699999999978, 0.0033737000000000003", \
            "0.0003742000000000054, 0.0003436000000000059, 0.00039950000000000207, 0.0005906999999999983, 0.0007634999999999968, 0.0011095999999999853, 0.0019596000000000136, 0.0034534000000000223", \
            "0.0005760000000000104, 0.0005282999999999851, 0.0005991999999999742, 0.0007162000000000021, 0.000991199999999996, 0.00146029999999998, 0.0022785000000000296, 0.0038268000000000408", \
            "0.0008886999999999407, 0.0008602999999999768, 0.0009317999999999828, 0.001040900000000019, 0.001403900000000031, 0.001960800000000008, 0.0027232999999999576, 0.004302199999999952", \
            "0.0013719999999999412, 0.0013680000000000515, 0.0015060000000000366, 0.0016529999999999759, 0.0019950000000000146, 0.002601000000000082, 0.0036439999999999823, 0.005268000000000007" \
          ) ;
        }
      
        ocv_std_dev_cell_rise(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.0006428137871023682, 0.0007802006836563123, 0.0010611545000087826, 0.001680014878108723, 0.002969293402010354, 0.005613787183988285, 0.011131580760760503, 0.02182233199296334", \
            "0.0008131632041328204, 0.0008839710178758372, 0.0010668696505301027, 0.001576286637216299, 0.002813918383500134, 0.005495213332023101, 0.010780506393486346, 0.021722640313225693", \
            "0.0016856886662594463, 0.001679015325884654, 0.00168464912654106, 0.0018593641616005306, 0.0027389753217013264, 0.005137473313258614, 0.010322389744843216, 0.021012342167032315", \
            "0.0033011454167707712, 0.0032811635517501006, 0.0032273377976971096, 0.0031474903018295568, 0.0033861592139505545, 0.005031847237568149, 0.009853752979900038, 0.020285910573323764", \
            "0.00613748113009093, 0.006099494645733636, 0.006049252887753989, 0.005875232043644462, 0.005651073806513828, 0.006095887816115603, 0.009582872999958482, 0.01940281402958517", \
            "0.011873669918493325, 0.011786099873721303, 0.011682352234602981, 0.01149763799704578, 0.011051200264626864, 0.010590256455293654, 0.011675651628258035, 0.01879034145985054", \
            "0.02381617910884205, 0.02365386556170283, 0.023487070744939203, 0.02325198854315714, 0.022740274553458364, 0.02189112254266048, 0.021288297813326654, 0.023772849040683178", \
            "0.04817437673009724, 0.0479187457635898, 0.04760965149584228, 0.04720371479025773, 0.04668690091092647, 0.045788479252124335, 0.04451287858586548, 0.04396656729834614" \
          ) ;
        }
      
        ocv_skewness_cell_rise(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.0012576313547068042, 0.0014468979930186805, 0.0017701714515603964, 0.0024521359194907247, 0.0037782803641526978, 0.006857949875401043, 0.01448196910860532, 0.025823837125852234", \
            "0.0017004454172022477, 0.001859076761529327, 0.0020813115918937686, 0.002501797675758637, 0.00357129515884192, 0.006760925063815425, 0.011712038167856516, 0.024530286907017952", \
            "0.002983129701537248, 0.0031553559985728293, 0.0034102087815983465, 0.0037331940901113576, 0.004341169575772672, 0.006237338446827969, 0.011181492299977863, 0.02330620438621086", \
            "0.0049559740623794655, 0.005168678448470225, 0.005530759617612563, 0.006104423925714517, 0.006792287572226712, 0.007729342178734649, 0.012177066013072268, 0.020238796920351562", \
            "0.006415901281978826, 0.006681404451684659, 0.007311153173134404, 0.008382705945466857, 0.009634279523274772, 0.010790853628959025, 0.011397791311369252, 0.018397322616008505", \
            "-0.007490282961057426, -0.0075662745137101955, -0.0068315073090394465, 0.005515212064818585, 0.010601157164079451, 0.013789994722947379, 0.015107235466192753, 0.0034822024515280964", \
            "-0.020608189605925652, -0.02099821265060144, -0.020967160960841297, -0.020279307011793472, -0.016227646277445777, 0.014503284071747946, 0.021958291887055684, 0.01698404722943128", \
            "-0.041429330081663, -0.04221181834622419, -0.042288913705264865, -0.04161033160030265, -0.0385683664141943, -0.026511007342968636, 0.034560464046959075, 0.04326880537067397" \
          ) ;
        }
      
        cell_fall(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.0078259, 0.0097227, 0.0132034, 0.019940199999999998, 0.03337349999999999, 0.0601636, 0.11362399999999999, 0.22058999999999998", \
            "0.012130299999999998, 0.014054800000000001, 0.0175301, 0.024269199999999998, 0.0376904, 0.0645129, 0.11800799999999999, 0.224915", \
            "0.018462100000000002, 0.020608799999999997, 0.024189099999999998, 0.030923799999999994, 0.0443623, 0.0711092, 0.12461499999999998, 0.23136699999999996", \
            "0.02656139999999999, 0.02939609999999999, 0.03336589999999999, 0.040181999999999995, 0.053579999999999996, 0.08029299999999999, 0.13379999999999997, 0.24068099999999998", \
            "0.03753699999999998, 0.041353999999999995, 0.04624099999999999, 0.053475999999999996, 0.06690099999999997, 0.09375299999999998, 0.14708, 0.253875", \
            "0.05331499999999998, 0.058360999999999996, 0.064918, 0.07355100000000002, 0.087644, 0.11473000000000003, 0.168231, 0.274977", \
            "0.077671, 0.08435700000000008, 0.09334900000000008, 0.10503, 0.12130800000000004, 0.14956000000000005, 0.20417100000000002, 0.31161", \
            "0.11825999999999995, 0.12700999999999996, 0.1393199999999999, 0.15566999999999992, 0.17738000000000007, 0.20951000000000003, 0.26725999999999994, 0.37744999999999995" \
          ) ;
        }
      
        ocv_mean_shift_cell_fall(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.000027041000000000025, 0.00004070000000000244, 0.000038049999999999414, 0.000055039999999997305, -0.0000008299999999944539, -0.000016049999999985174, -0.000024700000000015434, -0.00022889999999996805", \
            "0.000010590000000001472, 0.000001670000000001143, -0.0000010399999999993573, 0.000011549999999998268, -0.000007179999999995936, -0.00005216000000000853, -0.00010000000000000482, -0.0002136999999999824", \
            "-0.000023790000000000648, 0.000014700000000000733, 0.000009150000000005469, 0.000004319999999992856, -0.00002688000000000079, -0.000030119999999990584, -0.00009829999999997989, -0.000013500000000012412", \
            "0.00003172000000000625, 0.00003763000000000253, 0.00002436999999999942, 0.000022129999999992353, 0.000035699999999993475, 0.00004859999999999557, -0.0000565000000000194, -0.00007120000000000074", \
            "0.00011159999999999964, 0.0000845999999999942, 0.00012819999999998566, 0.00018299999999999304, 0.00024359999999998817, 0.00020650000000002662, 0.0001506999999999848, 0.00013289999999992", \
            "0.00019940000000001225, 0.00023010000000001454, 0.00020519999999998704, 0.00031670000000000005, 0.000345400000000011, 0.0003412000000000163, 0.00040790000000000433, 0.0003180000000000396", \
            "0.00027509999999997405, 0.00031159999999997695, 0.00021409999999998067, 0.00026350000000001155, 0.0004153999999999911, 0.000484200000000054, 0.00052799999999998, 0.0005570000000000219", \
            "0.0002890000000000235, 0.0002939999999999113, 0.00029899999999992834, 0.00036900000000008936, 0.00034600000000007303, 0.0006440000000000703, 0.0007360000000000065, 0.0007630000000000313" \
          ) ;
        }
      
        ocv_std_dev_cell_fall(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.0005884792607777741, 0.0006948987776008176, 0.0009186417936279622, 0.0014208856011500564, 0.002498988495393908, 0.004684238514481894, 0.009120545135388933, 0.018055535848228554", \
            "0.0009060469309036923, 0.0009734970878801391, 0.0011609829639864092, 0.001604313197636644, 0.002621012359036528, 0.004778245986226419, 0.00920276175938506, 0.01806722376785591", \
            "0.0020045599696968688, 0.00203156352376959, 0.0021368695276814008, 0.002434914954846131, 0.003256332855222268, 0.00521151765287447, 0.009496338324147187, 0.018283441016091282", \
            "0.003972955195317458, 0.004001163967050367, 0.004066834924401694, 0.004247252508007706, 0.004806182940292175, 0.006401131708967995, 0.010303526270705146, 0.01888119577663331", \
            "0.0074796076278549955, 0.007541878401447624, 0.007574974452468835, 0.007718104963151399, 0.008077879881366013, 0.009134547197316356, 0.012303441063828899, 0.02020357101246103", \
            "0.014043744563960776, 0.014123670977397413, 0.014169902225021406, 0.014266290275798168, 0.01445966179257162, 0.015079416816014843, 0.017274770250602787, 0.02361887060993579", \
            "0.026744300661594746, 0.026874642766411282, 0.026969796556930523, 0.027025265270565698, 0.027120186488214924, 0.02741989797695584, 0.02866570038681536, 0.0330758867320456", \
            "0.051850220496691767, 0.052049978802429266, 0.05227345246128496, 0.05238694121629926, 0.052379438756485756, 0.052487994013446115, 0.05307902017014087, 0.055609325566850755" \
          ) ;
        }
      
        ocv_skewness_cell_fall(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.0009271156039293627, 0.0009883930243098668, 0.0009063734516642819, -0.0012841293687656464, -0.0031885106075635414, -0.006328537562236666, -0.012673526469905229, -0.025811670860470155", \
            "0.00112190424206152, 0.0011884239648730643, 0.0012498112356133266, 0.0007394365935940763, -0.0028376342339430085, -0.006144583600443278, -0.012614564172648416, -0.025285043616362295", \
            "0.001736778438148716, 0.0019689771344223532, 0.002111514974188094, 0.002431221792429438, 0.0019160859020514783, -0.004847789569771145, -0.0115761430159942, -0.02458902309002602", \
            "0.002080254468241496, 0.0026209535301657413, 0.0019484670570220747, 0.0025039302315953134, 0.0033666573553792823, 0.00338916123927787, -0.00998539659241647, -0.022905182560355963", \
            "-0.005718759767707448, -0.005121261293762691, -0.004229663171301431, -0.0042123370316451665, -0.0025105594477310106, 0.004892395532586893, 0.0054694196714361615, -0.018875096808616168", \
            "-0.01319744192336911, -0.012396024199182292, -0.011484308680429512, -0.010927734890695534, -0.011287267159889865, -0.011640620104436854, -0.00694098332856638, 0.007836224868772662", \
            "-0.025724828991734987, -0.02481786325878898, -0.02390446331668271, -0.02241901027371641, -0.02210818984772819, -0.023482258488909065, -0.023409579875019088, -0.019605995921199208", \
            "-0.0463301789377073, -0.04578075435683372, -0.044065282875445366, -0.04099246347206821, -0.03796217589996837, -0.038058034688827264, -0.0422981524744322, -0.045056775370171845" \
          ) ;
        }
      
        fall_transition(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.003163880000000002, 0.005908199999999996, 0.011790599999999998, 0.023879400000000002, 0.047944, 0.09615879999999999, 0.19226139999999997, 0.38630999999999993", \
            "0.0033186000000000005, 0.0059743999999999995, 0.011820000000000004, 0.023940600000000006, 0.04806040000000001, 0.09608339999999999, 0.1925726, 0.38624400000000003", \
            "0.0044980000000000055, 0.006766799999999994, 0.012133999999999995, 0.02389639999999999, 0.047876800000000004, 0.09622059999999999, 0.19241399999999995, 0.38506", \
            "0.006498599999999996, 0.008685400000000005, 0.013168799999999996, 0.024224999999999986, 0.048098, 0.09613200000000002, 0.19255800000000003, 0.38565", \
            "0.009605999999999996, 0.011905999999999979, 0.015963999999999996, 0.025657999999999972, 0.048666000000000015, 0.09663800000000002, 0.19300000000000003, 0.38619999999999993", \
            "0.014079999999999914, 0.01706399999999997, 0.021709999999999917, 0.0299039999999999, 0.050696000000000005, 0.09782199999999999, 0.19338399999999992, 0.385182", \
            "0.020919999999999963, 0.025589999999999908, 0.03079399999999988, 0.03956000000000001, 0.057099999999999936, 0.10204600000000004, 0.19670999999999983, 0.387652", \
            "0.03259999999999966, 0.038340000000000096, 0.04695999999999959, 0.0571199999999999, 0.07418000000000025, 0.11361999999999986, 0.20674000000000003, 0.39414000000000005" \
          ) ;
        }
      
        ocv_mean_shift_fall_transition(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.000024930000000000783, 0.000017899999999998135, 0.0000385600000000014, -0.0000169999999999972, 0.000034299999999993095, 0.000053059999999998154, 0.00016827999999994845, -0.000942200000000028", \
            "0.000031879999999999654, 0.000010319999999997475, 0.000009900000000002209, -0.000045559999999995213, -0.00005603999999999296, 0.00009630000000000151, -0.00006289999999999158, -0.0006219999999999075", \
            "0.000017259999999996233, 0.0000249800000000042, 0.000008960000000005649, -0.000017520000000003342, 0.000063319999999982, -0.00006937999999997118, 0.00006726000000002692, 0.000606199999999989", \
            "0.000024139999999991858, -0.000004139999999995578, 0.000054099999999986204, 0.000053880000000015917, -0.000031120000000005624, 0.00005339999999998763, -0.00010200000000002198, 0.0002052000000000646", \
            "-0.000027800000000011653, 0.00006520000000004297, 0.00014220000000004113, 0.00004640000000002129, 0.000014200000000041682, 0.0000010000000000344278, -0.00027780000000002047, -0.0008534000000001554", \
            "0.000058600000000007035, 0.0001830000000000609, -0.00007040000000000422, 0.00006160000000011355, 0.00009739999999993858, 0.00008219999999991416, -0.00005679999999998578, 0.0002881999999998977", \
            "0.00021799999999996695, -0.00007920000000001767, 0.00007580000000005184, 0.000011199999999809151, 0.0004894000000000637, 0.00020700000000009554, -0.00008699999999994499, 0.0006897999999999617", \
            "0.00021399999999997463, 0.00027000000000012545, -0.00004800000000008219, 0.00026799999999973346, 0.000820000000000042, 0.0005199999999999501, 0.0008880000000000438, 0.00018599999999988954" \
          ) ;
        }
      
        ocv_std_dev_fall_transition(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.00018675746131160398, 0.0004254465079315029, 0.0009333842119465666, 0.0018996349778020235, 0.003849024241429392, 0.007691212934280557, 0.01545473504088634, 0.030886712590080252", \
            "0.00013808835174948113, 0.00040897587214895706, 0.0009319594805211939, 0.001894009444538224, 0.003827648539305095, 0.007719692316407436, 0.015464754770121627, 0.030867707815414123", \
            "0.00008222857302800341, 0.00027970351684119513, 0.0008500779404789236, 0.0018799039247318504, 0.0038579746536809185, 0.007700981402652516, 0.015456464947587463, 0.030745786463983803", \
            "0.0001826798790842119, 0.0001336810482703801, 0.0006482848225210199, 0.0018476000341584324, 0.003787309385830526, 0.007710415181212142, 0.015452569092405171, 0.030966236296973516", \
            "0.0002988666741021717, 0.0002780378871065296, 0.0003360720160917883, 0.0015396391496422376, 0.0038478698811449273, 0.007619748566863754, 0.015137718819778198, 0.030613378572121028", \
            "0.0004836417866332244, 0.0005258104855047001, 0.0003272726352413083, 0.0009110915553456991, 0.0035942988251458073, 0.0076641837979584525, 0.015368396640147228, 0.031243411372284167", \
            "0.0007794944230432774, 0.0008053246136393593, 0.00084652568380015, 0.000578705989447339, 0.002765633550080916, 0.007647901745649604, 0.015468162750918047, 0.03071281917592929", \
            "0.0013797600435502392, 0.0015043418642198827, 0.0017275338877524363, 0.0015859998598852037, 0.001789053132556722, 0.006957106678306214, 0.015790884008749426, 0.030665553675455124" \
          ) ;
        }
      
        ocv_skewness_fall_transition(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.000201398946702654, -0.0005480370558794946, -0.0012813713680072836, -0.0027712273597134945, -0.005511465030880916, -0.011231695460038098, -0.022485191384937563, -0.04630923880941348", \
            "0.00014689037307900063, -0.0005287282529273771, -0.0013021063866575388, -0.0027705885000732914, -0.005560691775091793, -0.011277583900747764, -0.022701627180311826, -0.04519573304595123", \
            "0.00008056667125200777, 0.00022969818235504602, -0.001153704745916823, -0.002796283752704877, -0.005650528790429956, -0.011343084019893531, -0.022613489681729864, -0.04468371402140012", \
            "0.00022385761352639147, 0.00018168458799091438, -0.0006242012125393695, -0.002555084200584369, -0.005595140776375203, -0.011200310720028422, -0.02253317613053692, -0.04464842881132925", \
            "0.00027866222815466063, -0.00040016922306545296, 0.0006720413069846715, -0.0019515460344705074, -0.005617164511327372, -0.011153108184063988, -0.02217732344298713, -0.0453230384888029", \
            "-0.0006846950958582848, 0.0006856607331690559, 0.0005604867794659538, 0.00088192934353082, -0.004916825576932352, -0.01121427278304054, -0.02251544397079647, -0.045222311732702254", \
            "-0.0010490947226702168, -0.0012707295463546754, -0.0007976979156972691, -0.0002905052051483442, -0.0012859461412672013, -0.010812407602181726, -0.02294034167700156, -0.04389860897986193", \
            "-0.0021833721846642564, -0.002560221335029738, -0.0029370842117759474, -0.0015862788340358647, 0.002406054159582661, -0.006209228850336533, -0.022283353506135122, -0.04584019086081102" \
          ) ;
        }
      
        rise_transition(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.004159999999999998, 0.007054799999999999, 0.0131694, 0.025991600000000004, 0.052467999999999994, 0.1046242, 0.20828499999999997, 0.41765599999999997", \
            "0.004186400000000003, 0.0070966000000000015, 0.013181999999999998, 0.025997799999999995, 0.05217959999999999, 0.1043298, 0.2086026, 0.4177759999999999", \
            "0.005138999999999999, 0.007879200000000006, 0.013632199999999999, 0.026042199999999984, 0.05205640000000001, 0.104299, 0.20804599999999998, 0.41715199999999997", \
            "0.00689259999999999, 0.009749999999999991, 0.014920800000000007, 0.026665400000000002, 0.05216400000000001, 0.10498000000000003, 0.20802400000000004, 0.41716000000000003", \
            "0.009754000000000027, 0.012739999999999998, 0.01790000000000001, 0.028544000000000017, 0.05292000000000001, 0.10484399999999999, 0.208684, 0.4182760000000001", \
            "0.013855999999999957, 0.017577999999999934, 0.023462000000000056, 0.03329399999999993, 0.056308000000000066, 0.10652600000000001, 0.20941600000000005, 0.41815599999999986", \
            "0.02046200000000017, 0.025378000000000157, 0.032049999999999905, 0.043476000000000105, 0.06493999999999998, 0.11421399999999998, 0.214786, 0.4190279999999999", \
            "0.031580000000000004, 0.03802000000000025, 0.047259999999999684, 0.061520000000000005, 0.08425999999999986, 0.13221999999999978, 0.23203999999999994, 0.43198" \
          ) ;
        }
      
        ocv_mean_shift_rise_transition(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.000026740000000002366, 0.00008859999999999943, 0.00016627999999999915, 0.0003247799999999985, 0.0004601999999999967, 0.0012347199999999897, 0.003051799999999984, 0.004613400000000047", \
            "0.0000517000000000031, 0.00008825999999999623, 0.00017838000000000795, 0.00031345999999999734, 0.0007034799999999876, 0.0014769800000000056, 0.00266276000000001, 0.0047933999999999885", \
            "0.000023519999999995845, 0.00010628000000000628, 0.00019266000000000213, 0.0003646800000000028, 0.000707999999999986, 0.0012996600000000345, 0.0034852000000000264, 0.005880199999999944", \
            "0.000036860000000006366, 0.000005259999999991033, 0.00020058000000001164, 0.0003659999999999764, 0.0007479400000000102, 0.000722600000000013, 0.0033976000000000193, 0.005738000000000058", \
            "0.000006999999999987348, 0.00011580000000000077, 0.0002335999999999767, 0.00047219999999998334, 0.0008743999999999931, 0.001375400000000045, 0.002871599999999953, 0.005317200000000025", \
            "0.00011140000000002474, 0.00014739999999997328, 0.00010359999999999886, 0.0006612000000000279, 0.0009025999999999868, 0.0014734000000000457, 0.002959400000000024, 0.004868599999999869", \
            "0.0002669999999999737, 0.0002478000000000378, 0.0005186000000001048, 0.0005125999999999693, 0.001346600000000028, 0.0015560000000000487, 0.0028852000000001007, 0.006515399999999872", \
            "0.00029999999999996924, 0.000264000000000158, 0.0004220000000000206, 0.0006920000000003916, 0.0012939999999998948, 0.00224599999999988, 0.003499999999999781, 0.00651199999999977" \
          ) ;
        }
      
        ocv_std_dev_rise_transition(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.00027704977571227444, 0.0005459416004584295, 0.0011576701975183701, 0.002457558637618507, 0.00492849169624947, 0.009719830443971518, 0.019659578252512612, 0.03918630450155882", \
            "0.0002625899422631744, 0.0005417918013406957, 0.0011539680005191733, 0.0024460478509901085, 0.004901070299615968, 0.009710082286617802, 0.019810236222900294, 0.03921513353059276", \
            "0.00023782982151109868, 0.0004954549711572635, 0.0011129357535615246, 0.002414377341768358, 0.004857430559919969, 0.009662513808206317, 0.019411388941088726, 0.038939696927999375", \
            "0.00021126895023484136, 0.0003944183684025547, 0.0009844194158092506, 0.002348788684501957, 0.004920309072541412, 0.009753067531124074, 0.01955308204861832, 0.03913884251964766", \
            "0.0003151059786449245, 0.000351395250710968, 0.0008266627956898608, 0.002158165362833277, 0.0048638567047605795, 0.00983660583060369, 0.019644042779201808, 0.03908985962511394", \
            "0.0004347812221233966, 0.0005863537423016267, 0.0007775201605103242, 0.00200276568774282, 0.00497338534825348, 0.009949123738076429, 0.019734206310645222, 0.03933876715065349", \
            "0.0005881135566840693, 0.0007840070577913633, 0.0011202856976880709, 0.002040778947145196, 0.005412062991339111, 0.0108432336710247, 0.020206903225273153, 0.038994401934978634", \
            "0.0009174360407618779, 0.001143087437114364, 0.0017294687687905359, 0.002710591571348771, 0.005746071508237389, 0.01275307736282585, 0.022323478422703116, 0.03971195308440242" \
          ) ;
        }
      
        ocv_skewness_rise_transition(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.0003996419886379299, 0.0006754372545971134, 0.00136278091891822, 0.003004309161673819, 0.004669923443884102, 0.012439561163708906, 0.025107547244839736, 0.051096855570732395", \
            "0.00029741087216211636, 0.0004246778043363361, 0.0012977374772871126, 0.00289686159726121, 0.005599266185651656, 0.01046516370668218, 0.022555380124321338, 0.04716002242255121", \
            "-0.00042484597403201946, 0.00019572045486014022, 0.0013408659610681657, 0.0028627490936412535, 0.0052274235225220405, 0.0114817897885615, 0.02247551653388058, 0.04458531365827391", \
            "-0.00038025639224253515, -0.0006144318373801482, 0.0009051017802884535, 0.0026755117228234645, 0.00615404439246452, 0.01028528267204596, 0.023239989508024063, 0.04576608292480967", \
            "0.0007022183556256539, 0.0005233535827866833, 0.0006046830528196777, 0.002760575501540996, 0.0058613084187236365, 0.011966132379430177, 0.022153755078952637, 0.04260693781537203", \
            "0.0008313771787299174, 0.0012294944862739025, -0.0008850090787283158, 0.0023291896976509016, 0.00565245758692652, 0.010792202512028294, 0.022517238531921106, 0.04657185160244087", \
            "0.001062942456958502, 0.0011187992302727527, -0.0012185751265354208, -0.0018299820302699642, 0.007865850623359576, 0.010836299125196283, 0.022345240356113412, 0.04542557224448773", \
            "-0.0013266060032470655, -0.0012023801647811176, -0.0018240369914003683, -0.0030430999311403173, 0.00775171727400665, 0.01579342065262275, 0.01951012615947696, 0.03784494705318464" \
          ) ;
        }
      }
    "#,
      r#"
liberty_db::timing::Timing () {
| cell_rise (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.0109093, 0.013212699999999999, 0.0170902, 0.0243175, 0.0386197, 0.0672009, 0.124553, 0.238683", \
| | | "0.014126599999999998, 0.0164202, 0.0203034, 0.027543899999999996, 0.041853, 0.0704022, 0.12772599999999998, 0.24214499999999997", \
| | | "0.0184073, 0.0209574, 0.025013099999999996, 0.032315399999999994, 0.0466268, 0.07517900000000001, 0.132412, 0.24681999999999996", \
| | | "0.022354499999999992, 0.025422699999999996, 0.029874099999999997, 0.037371999999999996, 0.051819, 0.080353, 0.13749199999999998, 0.251644", \
| | | "0.025092999999999987, 0.028996999999999974, 0.034324, 0.04236999999999998, 0.05695599999999999, 0.08568699999999999, 0.14268599999999998, 0.25732099999999997", \
| | | "0.025117999999999994, 0.030123000000000018, 0.036904999999999986, 0.046551000000000016, 0.062202000000000014, 0.09141900000000001, 0.14870399999999998, 0.26288900000000004", \
| | | "0.018074000000000055, 0.02447400000000005, 0.03333700000000009, 0.04577000000000004, 0.06415900000000001, 0.09573000000000004, 0.155087, 0.270079", \
| | | "-0.004859999999999919, 0.0033100000000000638, 0.015199999999999905, 0.032079999999999956, 0.05591999999999994, 0.09310999999999989, 0.15792999999999988, 0.27812999999999993");
| }
| ocv_mean_shift_cell_rise (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.0000686599999999999, 0.00009142999999999998, 0.00013971999999999943, 0.00024071999999999635, 0.0004534499999999872, 0.0008832399999999935, 0.001651700000000006, 0.003313499999999997", \
| | | "0.0000459099999999997, 0.00006773999999999669, 0.00011393000000000167, 0.00021833999999999743, 0.0004126300000000041, 0.0008709999999999968, 0.0016388999999999987, 0.003235199999999938", \
| | | "0.00003997999999999849, 0.00005816999999999975, 0.00010011999999999174, 0.00020658999999999955, 0.00038191999999999393, 0.0008765999999999913, 0.0016924000000000106, 0.0031957000000000513", \
| | | "0.00014944000000000068, 0.00015760000000000773, 0.00022443999999999936, 0.0003118699999999988, 0.0004590000000000011, 0.0008858000000000199, 0.0019156999999999647, 0.0033736999999999795", \
| | | "0.0003742000000000051, 0.0003436000000000064, 0.000399500000000004, 0.0005906999999999996, 0.0007635000000000003, 0.0011095999999999884, 0.0019596000000000058, 0.0034533999999999954", \
| | | "0.0005760000000000105, 0.000528299999999985, 0.0005991999999999734, 0.0007162000000000002, 0.0009911999999999907, 0.0014602999999999838, 0.0022785000000000166, 0.003826800000000019", \
| | | "0.0008886999999999402, 0.0008602999999999771, 0.0009317999999999826, 0.0010409000000000182, 0.0014039000000000273, 0.0019608000000000125, 0.0027232999999999563, 0.004302199999999978", \
| | | "0.001371999999999941, 0.0013680000000000515, 0.001506000000000035, 0.001652999999999974, 0.0019950000000000176, 0.0026010000000000755, 0.0036439999999999806, 0.005267999999999995");
| }
| ocv_std_dev_cell_rise (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.0006428137871023682, 0.0007802006836563123, 0.0010611545000087826, 0.001680014878108723, 0.002969293402010354, 0.005613787183988285, 0.011131580760760503, 0.02182233199296334", \
| | | "0.0008131632041328204, 0.0008839710178758372, 0.0010668696505301027, 0.001576286637216299, 0.002813918383500134, 0.005495213332023101, 0.010780506393486346, 0.021722640313225693", \
| | | "0.0016856886662594463, 0.001679015325884654, 0.00168464912654106, 0.0018593641616005306, 0.0027389753217013264, 0.005137473313258614, 0.010322389744843216, 0.021012342167032315", \
| | | "0.0033011454167707712, 0.0032811635517501006, 0.0032273377976971096, 0.0031474903018295568, 0.0033861592139505545, 0.005031847237568149, 0.009853752979900038, 0.020285910573323764", \
| | | "0.00613748113009093, 0.006099494645733636, 0.006049252887753989, 0.005875232043644462, 0.005651073806513828, 0.006095887816115603, 0.009582872999958482, 0.01940281402958517", \
| | | "0.011873669918493325, 0.011786099873721303, 0.011682352234602981, 0.01149763799704578, 0.011051200264626864, 0.010590256455293654, 0.011675651628258035, 0.01879034145985054", \
| | | "0.02381617910884205, 0.02365386556170283, 0.023487070744939203, 0.02325198854315714, 0.022740274553458364, 0.02189112254266048, 0.021288297813326654, 0.023772849040683178", \
| | | "0.04817437673009724, 0.0479187457635898, 0.04760965149584228, 0.04720371479025773, 0.04668690091092647, 0.045788479252124335, 0.04451287858586548, 0.04396656729834614");
| }
| ocv_skewness_cell_rise (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.0012576313547068042, 0.0014468979930186805, 0.0017701714515603964, 0.0024521359194907247, 0.0037782803641526978, 0.006857949875401043, 0.01448196910860532, 0.025823837125852234", \
| | | "0.0017004454172022477, 0.001859076761529327, 0.0020813115918937686, 0.002501797675758637, 0.00357129515884192, 0.006760925063815425, 0.011712038167856516, 0.024530286907017952", \
| | | "0.002983129701537248, 0.0031553559985728293, 0.0034102087815983465, 0.0037331940901113576, 0.004341169575772672, 0.006237338446827969, 0.011181492299977863, 0.02330620438621086", \
| | | "0.0049559740623794655, 0.005168678448470225, 0.005530759617612563, 0.006104423925714517, 0.006792287572226712, 0.007729342178734649, 0.012177066013072268, 0.020238796920351562", \
| | | "0.006415901281978826, 0.006681404451684659, 0.007311153173134404, 0.008382705945466857, 0.009634279523274772, 0.010790853628959025, 0.011397791311369252, 0.018397322616008505", \
| | | "-0.007490282961057426, -0.0075662745137101955, -0.0068315073090394465, 0.005515212064818585, 0.010601157164079451, 0.013789994722947379, 0.015107235466192753, 0.0034822024515280964", \
| | | "-0.020608189605925652, -0.02099821265060144, -0.020967160960841297, -0.020279307011793472, -0.016227646277445777, 0.014503284071747946, 0.021958291887055684, 0.01698404722943128", \
| | | "-0.041429330081663, -0.04221181834622419, -0.042288913705264865, -0.04161033160030265, -0.0385683664141943, -0.026511007342968636, 0.034560464046959075, 0.04326880537067397");
| }
| cell_fall (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.0078259, 0.0097227, 0.0132034, 0.019940199999999998, 0.03337349999999999, 0.0601636, 0.11362399999999999, 0.22058999999999998", \
| | | "0.012130299999999998, 0.014054800000000001, 0.0175301, 0.024269199999999998, 0.0376904, 0.0645129, 0.11800799999999999, 0.224915", \
| | | "0.018462100000000002, 0.020608799999999997, 0.024189099999999998, 0.030923799999999994, 0.0443623, 0.0711092, 0.12461499999999998, 0.23136699999999996", \
| | | "0.02656139999999999, 0.02939609999999999, 0.03336589999999999, 0.040181999999999995, 0.053579999999999996, 0.08029299999999999, 0.13379999999999997, 0.24068099999999998", \
| | | "0.03753699999999998, 0.041353999999999995, 0.04624099999999999, 0.053475999999999996, 0.06690099999999997, 0.09375299999999998, 0.14708, 0.253875", \
| | | "0.05331499999999998, 0.058360999999999996, 0.064918, 0.07355100000000002, 0.087644, 0.11473000000000003, 0.168231, 0.274977", \
| | | "0.077671, 0.08435700000000008, 0.09334900000000008, 0.10503, 0.12130800000000004, 0.14956000000000005, 0.20417100000000002, 0.31161", \
| | | "0.11825999999999995, 0.12700999999999996, 0.1393199999999999, 0.15566999999999992, 0.17738000000000007, 0.20951000000000003, 0.26725999999999994, 0.37744999999999995");
| }
| ocv_mean_shift_cell_fall (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.000027041000000000218, 0.00004070000000000289, 0.00003804999999999954, 0.00005503999999999579, -8.299999999933361e-7, -0.000016049999999982745, -0.000024700000000016376, -0.00022889999999997634", \
| | | "0.000010590000000001293, 1.6700000000004905e-6, -1.0400000000007625e-6, 0.000011549999999999061, -7.179999999995523e-6, -0.00005216000000000942, -0.00010000000000000286, -0.00021369999999998335", \
| | | "-0.00002378999999999923, 0.000014699999999999436, 9.150000000006375e-6, 4.319999999991692e-6, -0.00002687999999999996, -0.000030119999999994596, -0.00009829999999998174, -0.000013499999999999623", \
| | | "0.00003172000000000591, 0.00003763000000000377, 0.000024370000000002723, 0.00002212999999999521, 0.00003569999999999268, 0.00004859999999999587, -0.00005650000000001487, -0.00007119999999999349", \
| | | "0.00011159999999999642, 0.00008459999999999718, 0.00012819999999998805, 0.00018299999999999567, 0.00024359999999998272, 0.0002065000000000261, 0.00015069999999997585, 0.00013289999999993585", \
| | | "0.0001994000000000093, 0.0002301000000000178, 0.00020519999999998872, 0.0003167000000000031, 0.0003454000000000096, 0.0003412000000000137, 0.0004079000000000166, 0.00031800000000004047", \
| | | "0.0002750999999999726, 0.00031159999999998134, 0.00021409999999998097, 0.0002635000000000137, 0.00041539999999999633, 0.0004842000000000457, 0.0005279999999999729, 0.0005570000000000297", \
| | | "0.00028900000000002535, 0.00029399999999990545, 0.0002989999999999382, 0.0003690000000000915, 0.0003460000000000685, 0.0006440000000000612, 0.0007360000000000144, 0.0007630000000000137");
| }
| ocv_std_dev_cell_fall (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.0005884792607777741, 0.0006948987776008176, 0.0009186417936279622, 0.0014208856011500564, 0.002498988495393908, 0.004684238514481894, 0.009120545135388933, 0.018055535848228554", \
| | | "0.0009060469309036923, 0.0009734970878801391, 0.0011609829639864092, 0.001604313197636644, 0.002621012359036528, 0.004778245986226419, 0.00920276175938506, 0.01806722376785591", \
| | | "0.0020045599696968688, 0.00203156352376959, 0.0021368695276814008, 0.002434914954846131, 0.003256332855222268, 0.00521151765287447, 0.009496338324147187, 0.018283441016091282", \
| | | "0.003972955195317458, 0.004001163967050367, 0.004066834924401694, 0.004247252508007706, 0.004806182940292175, 0.006401131708967995, 0.010303526270705146, 0.01888119577663331", \
| | | "0.0074796076278549955, 0.007541878401447624, 0.007574974452468835, 0.007718104963151399, 0.008077879881366013, 0.009134547197316356, 0.012303441063828899, 0.02020357101246103", \
| | | "0.014043744563960776, 0.014123670977397413, 0.014169902225021406, 0.014266290275798168, 0.01445966179257162, 0.015079416816014843, 0.017274770250602787, 0.02361887060993579", \
| | | "0.026744300661594746, 0.026874642766411282, 0.026969796556930523, 0.027025265270565698, 0.027120186488214924, 0.02741989797695584, 0.02866570038681536, 0.0330758867320456", \
| | | "0.051850220496691767, 0.052049978802429266, 0.05227345246128496, 0.05238694121629926, 0.052379438756485756, 0.052487994013446115, 0.05307902017014087, 0.055609325566850755");
| }
| ocv_skewness_cell_fall (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.0009271156039293627, 0.0009883930243098668, 0.0009063734516642819, -0.0012841293687656464, -0.0031885106075635414, -0.006328537562236666, -0.012673526469905229, -0.025811670860470155", \
| | | "0.00112190424206152, 0.0011884239648730643, 0.0012498112356133266, 0.0007394365935940763, -0.0028376342339430085, -0.006144583600443278, -0.012614564172648416, -0.025285043616362295", \
| | | "0.001736778438148716, 0.0019689771344223532, 0.002111514974188094, 0.002431221792429438, 0.0019160859020514783, -0.004847789569771145, -0.0115761430159942, -0.02458902309002602", \
| | | "0.002080254468241496, 0.0026209535301657413, 0.0019484670570220747, 0.0025039302315953134, 0.0033666573553792823, 0.00338916123927787, -0.00998539659241647, -0.022905182560355963", \
| | | "-0.005718759767707448, -0.005121261293762691, -0.004229663171301431, -0.0042123370316451665, -0.0025105594477310106, 0.004892395532586893, 0.0054694196714361615, -0.018875096808616168", \
| | | "-0.01319744192336911, -0.012396024199182292, -0.011484308680429512, -0.010927734890695534, -0.011287267159889865, -0.011640620104436854, -0.00694098332856638, 0.007836224868772662", \
| | | "-0.025724828991734987, -0.02481786325878898, -0.02390446331668271, -0.02241901027371641, -0.02210818984772819, -0.023482258488909065, -0.023409579875019088, -0.019605995921199208", \
| | | "-0.0463301789377073, -0.04578075435683372, -0.044065282875445366, -0.04099246347206821, -0.03796217589996837, -0.038058034688827264, -0.0422981524744322, -0.045056775370171845");
| }
| rise_transition (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.004159999999999998, 0.007054799999999999, 0.0131694, 0.025991600000000004, 0.052467999999999994, 0.1046242, 0.20828499999999997, 0.41765599999999997", \
| | | "0.004186400000000003, 0.0070966000000000015, 0.013181999999999998, 0.025997799999999995, 0.05217959999999999, 0.1043298, 0.2086026, 0.4177759999999999", \
| | | "0.005138999999999999, 0.007879200000000006, 0.013632199999999999, 0.026042199999999984, 0.05205640000000001, 0.104299, 0.20804599999999998, 0.41715199999999997", \
| | | "0.00689259999999999, 0.009749999999999991, 0.014920800000000007, 0.026665400000000002, 0.05216400000000001, 0.10498000000000003, 0.20802400000000004, 0.41716000000000003", \
| | | "0.009754000000000027, 0.012739999999999998, 0.01790000000000001, 0.028544000000000017, 0.05292000000000001, 0.10484399999999999, 0.208684, 0.4182760000000001", \
| | | "0.013855999999999957, 0.017577999999999934, 0.023462000000000056, 0.03329399999999993, 0.056308000000000066, 0.10652600000000001, 0.20941600000000005, 0.41815599999999986", \
| | | "0.02046200000000017, 0.025378000000000157, 0.032049999999999905, 0.043476000000000105, 0.06493999999999998, 0.11421399999999998, 0.214786, 0.4190279999999999", \
| | | "0.031580000000000004, 0.03802000000000025, 0.047259999999999684, 0.061520000000000005, 0.08425999999999986, 0.13221999999999978, 0.23203999999999994, 0.43198");
| }
| ocv_mean_shift_rise_transition (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.000026740000000002526, 0.00008859999999999944, 0.00016627999999999955, 0.0003247800000000002, 0.00046019999999999395, 0.0012347199999999947, 0.0030517999999999934, 0.004613400000000045", \
| | | "0.00005170000000000348, 0.00008825999999999626, 0.0001783800000000075, 0.00031345999999999805, 0.0007034799999999855, 0.0014769800000000027, 0.002662760000000014, 0.004793400000000003", \
| | | "0.000023519999999995628, 0.00010628000000000547, 0.00019266000000000248, 0.00036468000000000264, 0.0007079999999999864, 0.0012996600000000358, 0.0034852000000000216, 0.005880199999999947", \
| | | "0.00003686000000000661, 5.259999999991313e-6, 0.00020058000000001096, 0.00036599999999997745, 0.0007479400000000094, 0.0007226000000000177, 0.0033976000000000284, 0.005738000000000076", \
| | | "6.999999999987919e-6, 0.00011580000000000097, 0.0002335999999999762, 0.00047219999999998166, 0.0008743999999999905, 0.0013754000000000405, 0.0028715999999999464, 0.005317200000000022", \
| | | "0.00011140000000002537, 0.00014739999999997463, 0.00010359999999999883, 0.0006612000000000284, 0.0009025999999999895, 0.0014734000000000413, 0.0029594000000000287, 0.00486859999999989", \
| | | "0.0002669999999999721, 0.0002478000000000376, 0.0005186000000001051, 0.0005125999999999672, 0.001346600000000031, 0.0015560000000000435, 0.0028852000000000877, 0.0065153999999998935", \
| | | "0.00029999999999996696, 0.00026400000000015994, 0.0004220000000000196, 0.0006920000000003937, 0.0012939999999998925, 0.002245999999999887, 0.003499999999999781, 0.006511999999999796");
| }
| ocv_std_dev_rise_transition (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.00027704977571227444, 0.0005459416004584295, 0.0011576701975183701, 0.002457558637618507, 0.00492849169624947, 0.009719830443971518, 0.019659578252512612, 0.03918630450155882", \
| | | "0.0002625899422631744, 0.0005417918013406957, 0.0011539680005191733, 0.0024460478509901085, 0.004901070299615968, 0.009710082286617802, 0.019810236222900294, 0.03921513353059276", \
| | | "0.00023782982151109868, 0.0004954549711572635, 0.0011129357535615246, 0.002414377341768358, 0.004857430559919969, 0.009662513808206317, 0.019411388941088726, 0.038939696927999375", \
| | | "0.00021126895023484136, 0.0003944183684025547, 0.0009844194158092506, 0.002348788684501957, 0.004920309072541412, 0.009753067531124074, 0.01955308204861832, 0.03913884251964766", \
| | | "0.0003151059786449245, 0.000351395250710968, 0.0008266627956898608, 0.002158165362833277, 0.0048638567047605795, 0.00983660583060369, 0.019644042779201808, 0.03908985962511394", \
| | | "0.0004347812221233966, 0.0005863537423016267, 0.0007775201605103242, 0.00200276568774282, 0.00497338534825348, 0.009949123738076429, 0.019734206310645222, 0.03933876715065349", \
| | | "0.0005881135566840693, 0.0007840070577913633, 0.0011202856976880709, 0.002040778947145196, 0.005412062991339111, 0.0108432336710247, 0.020206903225273153, 0.038994401934978634", \
| | | "0.0009174360407618779, 0.001143087437114364, 0.0017294687687905359, 0.002710591571348771, 0.005746071508237389, 0.01275307736282585, 0.022323478422703116, 0.03971195308440242");
| }
| ocv_skewness_rise_transition (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.0003996419886379299, 0.0006754372545971134, 0.00136278091891822, 0.003004309161673819, 0.004669923443884102, 0.012439561163708906, 0.025107547244839736, 0.051096855570732395", \
| | | "0.00029741087216211636, 0.0004246778043363361, 0.0012977374772871126, 0.00289686159726121, 0.005599266185651656, 0.01046516370668218, 0.022555380124321338, 0.04716002242255121", \
| | | "-0.00042484597403201946, 0.00019572045486014022, 0.0013408659610681657, 0.0028627490936412535, 0.0052274235225220405, 0.0114817897885615, 0.02247551653388058, 0.04458531365827391", \
| | | "-0.00038025639224253515, -0.0006144318373801482, 0.0009051017802884535, 0.0026755117228234645, 0.00615404439246452, 0.01028528267204596, 0.023239989508024063, 0.04576608292480967", \
| | | "0.0007022183556256539, 0.0005233535827866833, 0.0006046830528196777, 0.002760575501540996, 0.0058613084187236365, 0.011966132379430177, 0.022153755078952637, 0.04260693781537203", \
| | | "0.0008313771787299174, 0.0012294944862739025, -0.0008850090787283158, 0.0023291896976509016, 0.00565245758692652, 0.010792202512028294, 0.022517238531921106, 0.04657185160244087", \
| | | "0.001062942456958502, 0.0011187992302727527, -0.0012185751265354208, -0.0018299820302699642, 0.007865850623359576, 0.010836299125196283, 0.022345240356113412, 0.04542557224448773", \
| | | "-0.0013266060032470655, -0.0012023801647811176, -0.0018240369914003683, -0.0030430999311403173, 0.00775171727400665, 0.01579342065262275, 0.01951012615947696, 0.03784494705318464");
| }
| fall_transition (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.003163880000000002, 0.005908199999999996, 0.011790599999999998, 0.023879400000000002, 0.047944, 0.09615879999999999, 0.19226139999999997, 0.38630999999999993", \
| | | "0.0033186000000000005, 0.0059743999999999995, 0.011820000000000004, 0.023940600000000006, 0.04806040000000001, 0.09608339999999999, 0.1925726, 0.38624400000000003", \
| | | "0.0044980000000000055, 0.006766799999999994, 0.012133999999999995, 0.02389639999999999, 0.047876800000000004, 0.09622059999999999, 0.19241399999999995, 0.38506", \
| | | "0.006498599999999996, 0.008685400000000005, 0.013168799999999996, 0.024224999999999986, 0.048098, 0.09613200000000002, 0.19255800000000003, 0.38565", \
| | | "0.009605999999999996, 0.011905999999999979, 0.015963999999999996, 0.025657999999999972, 0.048666000000000015, 0.09663800000000002, 0.19300000000000003, 0.38619999999999993", \
| | | "0.014079999999999914, 0.01706399999999997, 0.021709999999999917, 0.0299039999999999, 0.050696000000000005, 0.09782199999999999, 0.19338399999999992, 0.385182", \
| | | "0.020919999999999963, 0.025589999999999908, 0.03079399999999988, 0.03956000000000001, 0.057099999999999936, 0.10204600000000004, 0.19670999999999983, 0.387652", \
| | | "0.03259999999999966, 0.038340000000000096, 0.04695999999999959, 0.0571199999999999, 0.07418000000000025, 0.11361999999999986, 0.20674000000000003, 0.39414000000000005");
| }
| ocv_mean_shift_fall_transition (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.000024930000000000698, 0.00001789999999999778, 0.00003856000000000172, -0.000016999999999996185, 0.00003429999999999406, 0.00005305999999999367, 0.00016827999999993737, -0.0009422000000000041", \
| | | "0.000031879999999999756, 0.000010319999999997692, 9.900000000001921e-6, -0.000045559999999996575, -0.00005603999999999332, 0.00009630000000000749, -0.00006290000000000462, -0.0006219999999999004", \
| | | "0.000017259999999996375, 0.000024980000000004304, 8.960000000005769e-6, -0.000017520000000003505, 0.00006331999999998478, -0.00006937999999996614, 0.00006726000000001342, 0.0006062000000000012", \
| | | "0.000024139999999991946, -4.13999999999623e-6, 0.00005409999999998576, 0.000053880000000016554, -0.000031120000000002535, 0.00005339999999998124, -0.00010200000000001874, 0.000205200000000072", \
| | | "-0.000027800000000011843, 0.000065200000000043, 0.0001422000000000402, 0.00004640000000002281, 0.000014200000000040569, 1.0000000000287557e-6, -0.0002778000000000225, -0.0008534000000001707", \
| | | "0.000058600000000007604, 0.00018300000000006159, -0.00007040000000000518, 0.00006160000000011295, 0.00009739999999994198, 0.00008219999999990735, -0.00005679999999999574, 0.0002881999999999052", \
| | | "0.00021799999999996822, -0.00007920000000001884, 0.00007580000000005291, 0.000011199999999808585, 0.0004894000000000634, 0.000207000000000096, -0.00008699999999994823, 0.0006897999999999627", \
| | | "0.0002139999999999781, 0.0002700000000001243, -0.0000480000000000827, 0.00026799999999973373, 0.0008200000000000429, 0.0005199999999999511, 0.0008880000000000554, 0.00018599999999990846");
| }
| ocv_std_dev_fall_transition (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.00018675746131160398, 0.0004254465079315029, 0.0009333842119465666, 0.0018996349778020235, 0.003849024241429392, 0.007691212934280557, 0.01545473504088634, 0.030886712590080252", \
| | | "0.00013808835174948113, 0.00040897587214895706, 0.0009319594805211939, 0.001894009444538224, 0.003827648539305095, 0.007719692316407436, 0.015464754770121627, 0.030867707815414123", \
| | | "0.00008222857302800341, 0.00027970351684119513, 0.0008500779404789236, 0.0018799039247318504, 0.0038579746536809185, 0.007700981402652516, 0.015456464947587463, 0.030745786463983803", \
| | | "0.0001826798790842119, 0.0001336810482703801, 0.0006482848225210199, 0.0018476000341584324, 0.003787309385830526, 0.007710415181212142, 0.015452569092405171, 0.030966236296973516", \
| | | "0.0002988666741021717, 0.0002780378871065296, 0.0003360720160917883, 0.0015396391496422376, 0.0038478698811449273, 0.007619748566863754, 0.015137718819778198, 0.030613378572121028", \
| | | "0.0004836417866332244, 0.0005258104855047001, 0.0003272726352413083, 0.0009110915553456991, 0.0035942988251458073, 0.0076641837979584525, 0.015368396640147228, 0.031243411372284167", \
| | | "0.0007794944230432774, 0.0008053246136393593, 0.00084652568380015, 0.000578705989447339, 0.002765633550080916, 0.007647901745649604, 0.015468162750918047, 0.03071281917592929", \
| | | "0.0013797600435502392, 0.0015043418642198827, 0.0017275338877524363, 0.0015859998598852037, 0.001789053132556722, 0.006957106678306214, 0.015790884008749426, 0.030665553675455124");
| }
| ocv_skewness_fall_transition (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.000201398946702654, -0.0005480370558794946, -0.0012813713680072836, -0.0027712273597134945, -0.005511465030880916, -0.011231695460038098, -0.022485191384937563, -0.04630923880941348", \
| | | "0.00014689037307900063, -0.0005287282529273771, -0.0013021063866575388, -0.0027705885000732914, -0.005560691775091793, -0.011277583900747764, -0.022701627180311826, -0.04519573304595123", \
| | | "0.00008056667125200777, 0.00022969818235504602, -0.001153704745916823, -0.002796283752704877, -0.005650528790429956, -0.011343084019893531, -0.022613489681729864, -0.04468371402140012", \
| | | "0.00022385761352639147, 0.00018168458799091438, -0.0006242012125393695, -0.002555084200584369, -0.005595140776375203, -0.011200310720028422, -0.02253317613053692, -0.04464842881132925", \
| | | "0.00027866222815466063, -0.00040016922306545296, 0.0006720413069846715, -0.0019515460344705074, -0.005617164511327372, -0.011153108184063988, -0.02217732344298713, -0.0453230384888029", \
| | | "-0.0006846950958582848, 0.0006856607331690559, 0.0005604867794659538, 0.00088192934353082, -0.004916825576932352, -0.01121427278304054, -0.02251544397079647, -0.045222311732702254", \
| | | "-0.0010490947226702168, -0.0012707295463546754, -0.0007976979156972691, -0.0002905052051483442, -0.0012859461412672013, -0.010812407602181726, -0.02294034167700156, -0.04389860897986193", \
| | | "-0.0021833721846642564, -0.002560221335029738, -0.0029370842117759474, -0.0015862788340358647, 0.002406054159582661, -0.006209228850336533, -0.022283353506135122, -0.04584019086081102");
| }
}"#,
    );
  }
  #[test]
  fn lvf_index_mismatch() {
    use crate::ast::GroupAttri;
    _ = crate::ast::test_parse_fmt::<Timing<DefaultCtx>>(
      r#"(){
        cell_rise(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.0109093, 0.013212699999999999, 0.0170902, 0.0243175, 0.0386197, 0.0672009, 0.124553, 0.238683", \
            "0.014126599999999998, 0.0164202, 0.0203034, 0.027543899999999996, 0.041853, 0.0704022, 0.12772599999999998, 0.24214499999999997", \
            "0.0184073, 0.0209574, 0.025013099999999996, 0.032315399999999994, 0.0466268, 0.07517900000000001, 0.132412, 0.24681999999999996", \
            "0.022354499999999992, 0.025422699999999996, 0.029874099999999997, 0.037371999999999996, 0.051819, 0.080353, 0.13749199999999998, 0.251644", \
            "0.025092999999999987, 0.028996999999999974, 0.034324, 0.04236999999999998, 0.05695599999999999, 0.08568699999999999, 0.14268599999999998, 0.25732099999999997", \
            "0.025117999999999994, 0.030123000000000018, 0.036904999999999986, 0.046551000000000016, 0.062202000000000014, 0.09141900000000001, 0.14870399999999998, 0.26288900000000004", \
            "0.018074000000000055, 0.02447400000000005, 0.03333700000000009, 0.04577000000000004, 0.06415900000000001, 0.09573000000000004, 0.155087, 0.270079", \
            "-0.004859999999999919, 0.0033100000000000638, 0.015199999999999905, 0.032079999999999956, 0.05591999999999994, 0.09310999999999989, 0.15792999999999988, 0.27812999999999993" \
          ) ;
        }
      
        ocv_mean_shift_cell_rise(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.00006865999999999982, 0.00009142999999999974, 0.0001397199999999989, 0.00024071999999999616, 0.00045344999999999056, 0.0008832399999999973, 0.001651700000000014, 0.0033134999999999905", \
            "0.000045910000000000155, 0.00006773999999999704, 0.0001139300000000005, 0.00021833999999999843, 0.00041263000000000123, 0.0008710000000000013, 0.001638899999999992, 0.0032351999999999494", \
            "0.00003997999999999878, 0.00005817000000000115, 0.00010011999999999332, 0.00020659000000000102, 0.00038191999999999317, 0.00087659999999999, 0.0016924000000000116, 0.0031957000000000244", \
            "0.0001494400000000006, 0.00015760000000000651, 0.00022443999999999776, 0.00031186999999999713, 0.00045900000000000194, 0.0008858000000000146, 0.001915699999999978, 0.0033737000000000003", \
            "0.0003742000000000054, 0.0003436000000000059, 0.00039950000000000207, 0.0005906999999999983, 0.0007634999999999968, 0.0011095999999999853, 0.0019596000000000136, 0.0034534000000000223", \
            "0.0005760000000000104, 0.0005282999999999851, 0.0005991999999999742, 0.0007162000000000021, 0.000991199999999996, 0.00146029999999998, 0.0022785000000000296, 0.0038268000000000408", \
            "0.0008886999999999407, 0.0008602999999999768, 0.0009317999999999828, 0.001040900000000019, 0.001403900000000031, 0.001960800000000008, 0.0027232999999999576, 0.004302199999999952", \
            "0.0013719999999999412, 0.0013680000000000515, 0.0015060000000000366, 0.0016529999999999759, 0.0019950000000000146, 0.002601000000000082, 0.0036439999999999823, 0.005268000000000007" \
          ) ;
        }
      
        ocv_std_dev_cell_rise(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.0006428137871023682, 0.0007802006836563123, 0.0010611545000087826, 0.001680014878108723, 0.002969293402010354, 0.005613787183988285, 0.011131580760760503, 0.02182233199296334", \
            "0.0008131632041328204, 0.0008839710178758372, 0.0010668696505301027, 0.001576286637216299, 0.002813918383500134, 0.005495213332023101, 0.010780506393486346, 0.021722640313225693", \
            "0.0016856886662594463, 0.001679015325884654, 0.00168464912654106, 0.0018593641616005306, 0.0027389753217013264, 0.005137473313258614, 0.010322389744843216, 0.021012342167032315", \
            "0.0033011454167707712, 0.0032811635517501006, 0.0032273377976971096, 0.0031474903018295568, 0.0033861592139505545, 0.005031847237568149, 0.009853752979900038, 0.020285910573323764", \
            "0.00613748113009093, 0.006099494645733636, 0.006049252887753989, 0.005875232043644462, 0.005651073806513828, 0.006095887816115603, 0.009582872999958482, 0.01940281402958517", \
            "0.011873669918493325, 0.011786099873721303, 0.011682352234602981, 0.01149763799704578, 0.011051200264626864, 0.010590256455293654, 0.011675651628258035, 0.01879034145985054", \
            "0.02381617910884205, 0.02365386556170283, 0.023487070744939203, 0.02325198854315714, 0.022740274553458364, 0.02189112254266048, 0.021288297813326654, 0.023772849040683178", \
            "0.04817437673009724, 0.0479187457635898, 0.04760965149584228, 0.04720371479025773, 0.04668690091092647, 0.045788479252124335, 0.04451287858586548, 0.04396656729834614" \
          ) ;
        }
      
        ocv_skewness_cell_rise(delay_template_8x8){
          index_1("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.8715");
          index_2("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
          values(  \
            "0.0012576313547068042, 0.0014468979930186805, 0.0017701714515603964, 0.0024521359194907247, 0.0037782803641526978, 0.006857949875401043, 0.01448196910860532, 0.025823837125852234", \
            "0.0017004454172022477, 0.001859076761529327, 0.0020813115918937686, 0.002501797675758637, 0.00357129515884192, 0.006760925063815425, 0.011712038167856516, 0.024530286907017952", \
            "0.002983129701537248, 0.0031553559985728293, 0.0034102087815983465, 0.0037331940901113576, 0.004341169575772672, 0.006237338446827969, 0.011181492299977863, 0.02330620438621086", \
            "0.0049559740623794655, 0.005168678448470225, 0.005530759617612563, 0.006104423925714517, 0.006792287572226712, 0.007729342178734649, 0.012177066013072268, 0.020238796920351562", \
            "0.006415901281978826, 0.006681404451684659, 0.007311153173134404, 0.008382705945466857, 0.009634279523274772, 0.010790853628959025, 0.011397791311369252, 0.018397322616008505", \
            "-0.007490282961057426, -0.0075662745137101955, -0.0068315073090394465, 0.005515212064818585, 0.010601157164079451, 0.013789994722947379, 0.015107235466192753, 0.0034822024515280964", \
            "-0.020608189605925652, -0.02099821265060144, -0.020967160960841297, -0.020279307011793472, -0.016227646277445777, 0.014503284071747946, 0.021958291887055684, 0.01698404722943128", \
            "-0.041429330081663, -0.04221181834622419, -0.042288913705264865, -0.04161033160030265, -0.0385683664141943, -0.026511007342968636, 0.034560464046959075, 0.04326880537067397" \
          ) ;
        }
      }
    "#,
      r#"
liberty_db::timing::Timing () {
| /* LVF LUTs' index mismatch */
| cell_rise (delay_template_8x8) {
| | index_1 ("0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715");
| | index_2 ("0.00015, 0.00059, 0.00148, 0.00325, 0.00679, 0.01388, 0.02805, 0.05639");
| | values ("0.0109093, 0.013212699999999999, 0.0170902, 0.0243175, 0.0386197, 0.0672009, 0.124553, 0.238683", \
| | | "0.014126599999999998, 0.0164202, 0.0203034, 0.027543899999999996, 0.041853, 0.0704022, 0.12772599999999998, 0.24214499999999997", \
| | | "0.0184073, 0.0209574, 0.025013099999999996, 0.032315399999999994, 0.0466268, 0.07517900000000001, 0.132412, 0.24681999999999996", \
| | | "0.022354499999999992, 0.025422699999999996, 0.029874099999999997, 0.037371999999999996, 0.051819, 0.080353, 0.13749199999999998, 0.251644", \
| | | "0.025092999999999987, 0.028996999999999974, 0.034324, 0.04236999999999998, 0.05695599999999999, 0.08568699999999999, 0.14268599999999998, 0.25732099999999997", \
| | | "0.025117999999999994, 0.030123000000000018, 0.036904999999999986, 0.046551000000000016, 0.062202000000000014, 0.09141900000000001, 0.14870399999999998, 0.26288900000000004", \
| | | "0.018074000000000055, 0.02447400000000005, 0.03333700000000009, 0.04577000000000004, 0.06415900000000001, 0.09573000000000004, 0.155087, 0.270079", \
| | | "-0.004859999999999919, 0.0033100000000000638, 0.015199999999999905, 0.032079999999999956, 0.05591999999999994, 0.09310999999999989, 0.15792999999999988, 0.27812999999999993");
| }
}"#,
    );
  }
  #[test]
  fn table_lookup() {
    use crate::ast::GroupAttri;
    let timing = crate::ast::test_parse_fmt::<Timing<DefaultCtx>>(
      r#"(){
        cell_rise(delay_template_3x3){
          index_1("10, 20, 30");
          index_2("30, 50, 60");
          values(  \
            "100, 200, 300", \
            "400, 500, 600", \
            "700, 800, 900", \
          ) ;
        }
        ocv_mean_shift_cell_rise(delay_template_3x3){
          index_1("10, 20, 30");
          index_2("30, 50, 60");
          values(  \
            "0, 0, 0", \
            "0, 0, 0", \
            "0, 0, 0", \
          ) ;
        }
        ocv_std_dev_cell_rise(delay_template_3x3){
          index_1("10, 20, 30");
          index_2("30, 50, 60");
          values(  \
            "100, 200, 300", \
            "400, 500, 600", \
            "700, 800, 900", \
          ) ;
        }
        ocv_skewness_cell_rise(delay_template_3x3){
          index_1("10, 20, 30");
          index_2("30, 50, 60");
          values(  \
            "100, 200, 300", \
            "400, 500, 600", \
            "700, 800, 900", \
          ) ;
        }
      }
    "#,
      r#"
liberty_db::timing::Timing () {
| cell_rise (delay_template_3x3) {
| | index_1 ("10.0, 20.0, 30.0");
| | index_2 ("30.0, 50.0, 60.0");
| | values ("100.0, 200.0, 300.0", \
| | | "400.0, 500.0, 600.0", \
| | | "700.0, 800.0, 900.0");
| }
| ocv_mean_shift_cell_rise (delay_template_3x3) {
| | index_1 ("10.0, 20.0, 30.0");
| | index_2 ("30.0, 50.0, 60.0");
| | values ("0.0, 0.0, 0.0", \
| | | "0.0, 0.0, 0.0", \
| | | "0.0, 0.0, 0.0");
| }
| ocv_std_dev_cell_rise (delay_template_3x3) {
| | index_1 ("10.0, 20.0, 30.0");
| | index_2 ("30.0, 50.0, 60.0");
| | values ("100.0, 200.0, 300.0", \
| | | "400.0, 500.0, 600.0", \
| | | "700.0, 800.0, 900.0");
| }
| ocv_skewness_cell_rise (delay_template_3x3) {
| | index_1 ("10.0, 20.0, 30.0");
| | index_2 ("30.0, 50.0, 60.0");
| | values ("100.0, 200.0, 300.0", \
| | | "400.0, 500.0, 600.0", \
| | | "700.0, 800.0, 900.0");
| }
}"#,
    );
    let table = timing.cell_rise.unwrap();
    let assert_fn = |idx1: f64, idx2: f64, want: f64| {
      assert_eq!(Some(want), table.lookup(&idx1, &idx2));
    };
    assert_fn(10.0, 30.0, 100.0);
    assert_fn(30.0, 60.0, 900.0);
    assert_fn(10.0, 42.0, 160.0);
    assert_fn(14.0, 30.0, 220.0);
    // 100 + (400-100)*0.4 = 220
    // 200 + (500-200)*0.4 = 320
    // 220 + (320-220)*0.6 = 280
    assert_fn(14.0, 42.0, 280.0);
    let assert_lvf_fn = |idx1: f64, idx2: f64, want: f64| {
      assert_eq!(
        Some(LVFValue { mean: want, std_dev: want, skewness: want }),
        table.lookup_lvf(&idx1, &idx2)
      );
    };
    assert_lvf_fn(10.0, 30.0, 100.0);
    assert_lvf_fn(30.0, 60.0, 900.0);
    assert_lvf_fn(10.0, 42.0, 160.0);
    assert_lvf_fn(14.0, 30.0, 220.0);
  }
  // FIXME:
  #[ignore]
  #[test]
  fn table_lookup_mismatch_lvf() {
    use crate::ast::GroupAttri;
    let timing = crate::ast::test_parse_fmt::<Timing<DefaultCtx>>(
      r#"(){
        cell_rise(delay_template_3x3){
          index_1("10, 20, 30");
          index_2("30, 40, 60");
          values(  \
            "100, 200, 300", \
            "400, 500, 600", \
            "700, 800, 900", \
          ) ;
        }
        ocv_mean_shift_cell_rise(delay_template_3x3){
          index_1("10, 20, 30");
          index_2("30, 50, 60");
          values(  \
            "0, 0, 0", \
            "0, 0, 0", \
            "0, 0, 0", \
          ) ;
        }
        ocv_std_dev_cell_rise(delay_template_3x3){
          index_1("10, 20, 30");
          index_2("30, 50, 60");
          values(  \
            "100, 200, 300", \
            "400, 500, 600", \
            "700, 800, 900", \
          ) ;
        }
        ocv_skewness_cell_rise(delay_template_3x3){
          index_1("10, 20, 30");
          index_2("30, 50, 60");
          values(  \
            "100, 200, 300", \
            "400, 500, 600", \
            "700, 800, 900", \
          ) ;
        }
      }
    "#,
      r#"
liberty_db::timing::Timing () {
| cell_rise (delay_template_3x3) {
| | index_1 ("10.0, 20.0, 30.0");
| | index_2 ("30.0, 40.0, 60.0");
| | values ("100.0, 200.0, 300.0", \
| | | "400.0, 500.0, 600.0", \
| | | "700.0, 800.0, 900.0");
| }
| ocv_mean_shift_cell_rise (delay_template_3x3) {
| | index_1 ("10.0, 20.0, 30.0");
| | index_2 ("30.0, 50.0, 60.0");
| | values ("0.0, 0.0, 0.0", \
| | | "0.0, 0.0, 0.0", \
| | | "0.0, 0.0, 0.0");
| }
| ocv_std_dev_cell_rise (delay_template_3x3) {
| | index_1 ("10.0, 20.0, 30.0");
| | index_2 ("30.0, 50.0, 60.0");
| | values ("100.0, 200.0, 300.0", \
| | | "400.0, 500.0, 600.0", \
| | | "700.0, 800.0, 900.0");
| }
| ocv_skewness_cell_rise (delay_template_3x3) {
| | index_1 ("10.0, 20.0, 30.0");
| | index_2 ("30.0, 50.0, 60.0");
| | values ("100.0, 200.0, 300.0", \
| | | "400.0, 500.0, 600.0", \
| | | "700.0, 800.0, 900.0");
| }
}"#,
    );
    let table = timing.cell_rise.unwrap();
    let assert_fn = |idx1: f64, idx2: f64, want: f64| {
      assert_eq!(Some(want), table.lookup(&idx1, &idx2));
    };
    assert_fn(10.0, 30.0, 100.0);
    assert_fn(30.0, 60.0, 900.0);
    assert_fn(10.0, 42.0, 160.0);
    assert_fn(14.0, 30.0, 220.0);
    // 100 + (400-100)*0.4 = 220
    // 200 + (500-200)*0.4 = 320
    // 220 + (320-220)*0.6 = 280
    assert_fn(14.0, 42.0, 280.0);
    let assert_lvf_fn = |idx1: f64, idx2: f64, want: f64| {
      assert_eq!(
        Some(LVFValue { mean: want, std_dev: want, skewness: want }),
        table.lookup_lvf(&idx1, &idx2)
      );
    };
    assert_lvf_fn(10.0, 30.0, 100.0);
    assert_lvf_fn(30.0, 60.0, 900.0);
    assert_lvf_fn(10.0, 42.0, 160.0);
    assert_lvf_fn(14.0, 30.0, 220.0);
  }
}
