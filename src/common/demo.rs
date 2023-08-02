//! cargo expand common::demo
use crate::{
  ast::{AttributeList, GroupComments, GroupId, GroupMap},
  cell::Statetable,
  timing::TimingType,
};
#[derive(Default, Debug)]
#[derive(liberty_macros::Group)]
struct Timing {
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

#[derive(Default, Debug)]
#[derive(liberty_macros::Group)]
struct Pin {
  #[liberty(id(auto_impl_len = 1))]
  _id: GroupId<Self>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(group(type=Vec))]
  timing: Vec<Timing>,
}
#[derive(Default, Debug)]
#[derive(liberty_macros::Group)]
struct Ff {
  #[liberty(id(auto_impl_len = 2))]
  _id: GroupId<Self>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(simple(type=Option))]
  next_state: Option<String>,
}

#[derive(Default, Debug)]
#[derive(liberty_macros::Group)]
struct Cell {
  #[liberty(id(auto_impl_len = 1))]
  _id: GroupId<Self>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(simple(type=Option))]
  area: Option<f64>,
  #[liberty(group(type=Map))]
  ff: GroupMap<Ff>,
  #[liberty(group(type=Map))]
  pin: GroupMap<Pin>,
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
  let _ = crate::ast::test_parse_group::<Cell>(
    r#"(INV){
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
    "#,
  );
}
