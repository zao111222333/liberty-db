//! cargo expand common::demo

use std::collections::HashMap;

use crate::{
    timing::TimingType, 
    cell::Statetable,
    ast::{UndefinedAttributes,HashedGroup}, 
};

#[derive(Default,Debug)]
#[derive(liberty_macros::Group)]
struct Timing{
  _undefined: UndefinedAttributes,
  #[arrti_type(complex)]
  values: Vec<f64>,
  #[arrti_type(simple)]
  t1: Option<TimingType>,
  #[arrti_type(simple)]
  t2: Option<TimingType>,
}
#[derive(Default,Debug)]
#[derive(liberty_macros::NameIdx)]
#[derive(liberty_macros::Group)]
struct Pin{
    #[idx_len(1)]
    _idx: Box<<Self as HashedGroup>::Idx>,
    _undefined: UndefinedAttributes,
    #[arrti_type(group)]
    timing: Vec<Timing>,
}
#[derive(Default,Debug)]
#[derive(liberty_macros::Group)]
#[derive(liberty_macros::NameIdx)]
struct Ff{
  #[idx_len(2)]
  _idx: Box<<Self as HashedGroup>::Idx>,
  _undefined: UndefinedAttributes,
  #[arrti_type(simple)]
  next_state: Option<String>,
}
#[derive(Default,Debug)]
#[derive(liberty_macros::Group)]
#[derive(liberty_macros::NameIdx)]
struct Cell{
  #[idx_len(1)]
  _idx: Box<<Self as HashedGroup>::Idx>,
  _undefined: UndefinedAttributes,
  #[arrti_type(simple)]
  area: Option<f64>,
  #[arrti_type(group)]
  ff: HashMap<<Ff as HashedGroup>::Idx,Ff>,
  #[arrti_type(group)]
  pin: HashMap<<Pin as HashedGroup>::Idx,Pin>,
  #[arrti_type(group)]
  statetable: Option<Statetable>,
}

#[test]
fn timing_test(){
    let _ = crate::ast::test_parse_group::<Timing>(r#"(w){
        // www
        /* com
        ment2 */
        t1 : "combinational";
        values ( \
            1,"2,3",4,\ // comment1
            5,\ /* comment2 */
            6\ /* comment3 */
        );
    }
    "#);
    let _ = crate::ast::test_parse_group::<Timing>(r#"( w ){
        t1: ombinational;
        t2: combinational;
        values ( \
            -1e2,"2,3,",\
            1,"2,3,",\
        );
        }
    "#);
}

#[test]
fn pin_test(){
    let _ = crate::ast::test_parse_group::<Pin>(r#"(A){
        timing(w){
            t1: combinational;
        }
    }
    "#);
    let _ = crate::ast::test_parse_group::<Pin>(r#"(B){
        timing(w){
            t1: combinational;
        }
    }
    "#);
}


#[test]
fn cell_test(){
    let _ = crate::ast::test_parse_group::<Cell>(r#"(INV){
        // should ok
        area : 5.4;
        // should ok
        ff(IQ,IQN){
            next_state: "!A";
        }
        // should ok
        pin(A){
            timing(w){
                t1: combinational;
            }
        }
        // should ok
        pin(Y){
            timing(){
                // should error
                t1: foo_error;
            }
        }
        statetable ("CLK EN SE",ENL) {
            table : "	H   L  L : - : L ,\
            H   L  H : - : H ,\
            H   H  L : - : H ,\
            H   H  H : - : H ,\
            L   -  - : - : N ";
        }
      }
    "#);
    let _ = crate::ast::test_parse_group::<Cell>(r#"(INV){
        // should error
        undefine_area : 5.4;
        // should error
        undefine_pin(C){
            timing(w){
                t1: combinational;
            }
        }
        // should ok
        pin("A"){
            timing(w){
                t1: combinational;
            }
        }
        pin("A"){
            timing(w){
                t2: combinational;
            }
        }
        // should error
        pin(A,Y){
            timing(w){
                t1: combinational;
            }
        }
    }
    "#);
}