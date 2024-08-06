//! cargo expand common::demo

use crate::{
  ast::{AttributeList, GroupComments, GroupFn, NamedGroup},
  cell::Statetable,
  timing::TimingType,
  ArcStr, GroupSet,
};

#[derive(Default, Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(liberty_macros::Nothing)]
pub(crate) struct Timing {
  /// group undefined attributes
  #[liberty(undefined)]
  undefined: AttributeList,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments<Self>,
  #[liberty(complex)]
  values: Vec<f64>,
  #[liberty(simple(type = Option))]
  t1: Option<TimingType>,
  #[liberty(simple(type = Option))]
  t2: Option<TimingType>,
}
impl GroupFn for Timing {}
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(Default, Debug, Clone)]
#[derive(liberty_macros::Group)]
pub(crate) struct Pin {
  #[id]
  #[liberty(name)]
  name: ArcStr,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  undefined: AttributeList,
  #[liberty(group(type=Vec))]
  timing: Vec<Timing>,
}
impl GroupFn for Pin {}
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(Default, Debug, Clone)]
#[derive(liberty_macros::Group)]
pub(crate) struct FF {
  #[id]
  #[liberty(name)]
  var1: ArcStr,
  #[id]
  #[liberty(name)]
  var2: ArcStr,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  undefined: AttributeList,
  #[liberty(simple(type = Option))]
  next_state: Option<ArcStr>,
}
impl GroupFn for FF {}
impl NamedGroup for FF {
  #[inline]
  fn parse(mut v: Vec<ArcStr>) -> Result<Self::Name, crate::ast::IdError> {
    let l = v.len();
    if l != 2 {
      return Err(crate::ast::IdError::LengthDismatch(2, l, v));
    }
    if let Some(var2) = v.pop() {
      if let Some(var1) = v.pop() {
        Ok(Self::Name { var1, var2 })
      } else {
        Err(crate::ast::IdError::Other("Unkown pop error".into()))
      }
    } else {
      Err(crate::ast::IdError::Other("Unkown pop error".into()))
    }
  }
  #[inline]
  fn name2vec(name: Self::Name) -> Vec<ArcStr> {
    vec![name.var1, name.var2]
  }
}

#[derive(Default, Debug)]
#[derive(liberty_macros::Group)]
pub(crate) struct Cell {
  #[liberty(name)]
  name: ArcStr,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  undefined: AttributeList,
  #[liberty(simple(type = Option))]
  area: Option<f64>,
  #[liberty(group(type=Set))]
  ff: GroupSet<FF>,
  #[liberty(group(type=Set))]
  pin: GroupSet<Pin>,
  #[liberty(group(type = Option))]
  statetable: Option<Statetable>,
}
impl GroupFn for Cell {}
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
  use crate::ast::GroupAttri;
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
  g.comments.area.push("xc".into());
  g.comments.area.push("xc".into());
  let mut output = String::new();
  let mut f = crate::ast::CodeFormatter::new(&mut output, "| ");
  if let Err(e) = GroupAttri::fmt_liberty(g, std::any::type_name::<Cell>(), &mut f) {
    panic!("{e}");
  }
  println!("{}", output);
}
