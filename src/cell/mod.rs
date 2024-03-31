//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

use crate::{
  ast::{AttributeList, GroupComments},
  expression::FFBank,
  pin::Pin,
};
mod items;
pub use items::*;
use mut_set::MutSet;

/// cell
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  macro(derive(Debug, Clone,Default);)
)]
pub struct Cell {
  #[id]
  #[liberty(name)]
  pub name: String,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(simple(type = Option))]
  pub area: Option<f64>,
  #[liberty(group(type=Set))]
  pub pin: MutSet<Pin>,
  #[liberty(group(type=Set))]
  pub ff: MutSet<crate::expression::FF>,
  #[liberty(group(type=Set))]
  pub ff_bank: MutSet<FFBank>,
  #[liberty(group(type=Set))]
  pub leakage_power: MutSet<LeakagePower>,
  #[liberty(group(type = Option))]
  pub statetable: Option<Statetable>,
}
mod test {
  #[allow(unused_imports)]
  use super::Cell;
  /// Example 23 A multibit register containing four rising-edge-triggered D flip-flops
  /// with clear  and preset is shown in Figure 1 and Example 23
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=167.32&end=167.33
  /// ">Reference</a>
  #[test]
  fn test_example23() {
    let (cell, _) = &mut crate::ast::test_parse_group::<Cell>(
      r#"(dff4) {  
        area : 1 ;  
        pin (CLK) {    
            direction : input ;    
            capacitance : 0 ;
            min_pulse_width_low  : 3 ;    
            min_pulse_width_high : 3 ;  
        }  
        bundle (D) {    
            members(D1, D2, D3, D4);    
            nextstate_type : data;    
            direction : input ;    
            capacitance : 0 ;    
            timing() {      
                related_pin     : "CLK" ;      
                timing_type     : setup_rising ;      
                cell_rise(scalar) {        
                    values (" 1.0 ") ;      
                }       
                cell_fall(scalar) {        
                    values (" 1.0 ") ;      
                }     
            }    
            timing() {      
                related_pin     : "CLK" ;      
                timing_type     : hold_rising ;      
                cell_rise(scalar) {        
                    values (" 1.0 ") ;      
                }       
                cell_fall(scalar) {        
                    values (" 1.0 ") ;      
                }     
            }  
        }  
        pin (CLR) {    
            direction : input ;    
            capacitance : 0 ;    
            timing() {      
                related_pin     : "CLK" ;      
                timing_type     : recovery_rising ;      
                cell_rise(scalar) {        
                    values (" 1.0 ") ;      
                }       
                cell_fall(scalar) {        
                    values (" 1.0 ") ;      
                }     
            }  
        }  
        pin (PRE) {    
            direction : input ;    
            capacitance : 0 ;    
            timing() {      
                related_pin     : "CLK" ;
                timing_type     : recovery_rising ;      
                cell_rise(scalar) {        
                    values (" 1.0 ") ;      
                }       
                cell_fall(scalar) {
                    values (" 1.0 ") ;      
                }     
            }  
        }  
        ff_bank (IQ, IQN, 4) {    
            next_state : "D" ;    
            clocked_on : "CLK" ;    
            clear : "CLR’" ;    
            preset : "PRE’" ;    
            clear_preset_var1 : L ;    
            clear_preset_var2 : L ;   
        }  
        bundle (Q) {    
            members(Q1, Q2, Q3, Q4);    
            direction : output ;    
            function : "(IQ)" ;    
            timing() {      
                related_pin     : "CLK" ;      
                timing_type     : rising_edge ;      
                cell_rise(scalar) {        
                    values (" 2.0 ") ;      
                }       
                cell_fall(scalar) {        
                    values (" 2.0 ") ;      
                }     
            }     
            timing() {      
                related_pin     : "PRE" ;      
                timing_type     : preset ;      
                timing_sense    : negative_unate ;      
                cell_rise(scalar) {        
                    values (" 1.0 ") ;      
                }     
            }    
            timing() {      
                related_pin     : "CLR" ;      
                timing_type     : clear ;      
                timing_sense    : positive_unate ;      
                cell_fall(scalar) {        
                    values (" 1.0 ") ;      
                }     
            }  
        }  
        bundle (QN) {    
            members(Q1N, Q2N, Q3N, Q4N);    
            direction : output ;    
            function : "IQN" ;    
            timing() {      
                related_pin     : "CLK" ;      
                timing_type     : rising_edge ;      
                cell_rise(scalar) {        
                    values (" 2.0 ") ;
                }       
                cell_fall(scalar) {        
                    values (" 2.0 ") ;      
                }     
            }    
            timing() {      
                related_pin     : "PRE" ;      
                timing_type     : clear ;      
                timing_sense    : positive_unate ;      
                cell_fall(scalar) {        
                    values (" 1.0 ") ;      
                }     
            }   
            timing() {      
                related_pin     : "CLR" ;      
                timing_type     : preset ;      
                timing_sense    : negative_unate ;      
                cell_rise(scalar) {        
                    values (" 1.0 ") ;      
                }     
            }  
        }
    } /* end of cell dff4 */
    "#,
    );
  }
}
