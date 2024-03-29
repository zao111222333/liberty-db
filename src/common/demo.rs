//! cargo expand common::demo
use mut_set::MutSet;

use crate::{
  ast::{AttributeList, GroupAttri, GroupComments},
  cell::Statetable,
  timing::TimingType,
};

#[derive(Default, Debug, Clone)]
#[derive(liberty_macros::Group)]
pub(crate) struct Timing {
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(complex)]
  values: Vec<f64>,
  #[liberty(simple(type=Option))]
  t1: Option<TimingType>,
  #[liberty(simple(type=Option))]
  t2: Option<TimingType>,
}

#[mut_set_derive::item(derive(liberty_macros::Nothing, Debug, Clone))]
#[derive(Default, Debug, Clone)]
#[derive(liberty_macros::Group)]
pub(crate) struct Pin {
  #[id]
  #[liberty(name)]
  name: String,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(group(type=Vec))]
  timing: Vec<Timing>,
}

#[mut_set_derive::item(derive(liberty_macros::Nothing, Debug, Clone))]
#[derive(Default, Debug)]
#[derive(liberty_macros::Group)]
pub(crate) struct Ff {
  #[id]
  #[liberty(name)]
  name: [String; 2],
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(simple(type=Option))]
  next_state: Option<String>,
}

#[derive(Default, Debug)]
#[derive(liberty_macros::Group)]
pub(crate) struct Cell {
  #[liberty(name)]
  name: String,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(simple(type=Option))]
  area: Option<f64>,
  #[liberty(group(type=Set))]
  ff: MutSet<Ff>,
  #[liberty(group(type=Set))]
  pin: MutSet<Pin>,
  #[liberty(group(type=Option))]
  statetable: Option<Statetable>,
}

#[test]
fn timing_test() {
  let _ = crate::ast::test_parse_group::<Timing>(
    r#"(w){
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
    "#,
  );
  let _ = crate::ast::test_parse_group::<Timing>(
    r#"( w ){
        t1: ombinational;
        t2: combinational;
        values ( \
            -1e2,"2,3,",\
            1,"2,3,",\
        );
        }
    "#,
  );
}

#[test]
fn pin_test() {
  let _ = crate::ast::test_parse_group::<Pin>(
    r#"(A){
        timing(w){
            t1: combinational;
        }
    }
    "#,
  );
  let _ = crate::ast::test_parse_group::<Pin>(
    r#"(B){
        timing(w){
            t1: combinational;
        }
    }
    "#,
  );
}

#[test]
fn cell_test() {
  let _ = crate::ast::test_parse_group::<Cell>(
    r#"(INV){
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
                test_table (\
                    "1,2,",\
                    "4,5,6",\
                    4 , 5 , 6);
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
    "#,
  );
  let (g, _) = &mut crate::ast::test_parse_group::<Cell>(
    r#"(INV){
        // should error
        area : 5.4;
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
    "#,
  );
  g.comments_mut().area.push("xc".to_owned());
  g.comments_mut().area.push("xc".to_owned());
  let mut output = String::new();
  let mut f = crate::ast::CodeFormatter::new(&mut output, "| ");
  if let Err(e) = GroupAttri::fmt_liberty(g, std::any::type_name::<Cell>(), &mut f) {
    panic!("{e}");
  }
  println!("{}", output);
}
