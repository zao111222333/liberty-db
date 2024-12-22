#![allow(clippy::redundant_pub_crate, clippy::doc_markdown, dead_code)]
//! cargo expand common::demo
//! cargo expand common::demo --no-default-features
use crate::{
  ast::{Attributes, GroupComments, GroupFn, GroupSet, NamedGroup},
  cell::Statetable,
  timing::TimingType,
  ArcStr, NotNan,
};
use core::fmt::Write;
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(liberty_macros::Group)]
#[derive(liberty_macros::Nothing)]
pub(crate) struct Timing {
  /// group undefined attributes
  #[liberty(attributes)]
  attributes: Attributes,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  extra_ctx: (),
  #[liberty(complex)]
  #[default = "vec![unsafe{ NotNan::new_unchecked(0.0) }]"]
  pub values: Vec<NotNan<f64>>,
  #[liberty(simple(type = Option))]
  t1: Option<TimingType>,
  #[liberty(simple(type = Option))]
  t2: Option<TimingType>,
}
impl GroupFn for Timing {}

#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Pin {
  #[size = 8]
  #[liberty(name)]
  #[id(borrow = "&str")]
  name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  #[size = 0]
  #[liberty(extra_ctx)]
  extra_ctx: (),
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  attributes: Attributes,
  #[size = 24]
  #[liberty(group(type = Vec))]
  timing: Vec<Timing>,
}
impl GroupFn for Pin {}

#[mut_set::derive::item(sort)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
pub(crate) struct FF {
  #[id(borrow = "&str")]
  #[size = 8]
  #[liberty(name)]
  variable1: ArcStr,
  #[id(borrow = "&str")]
  #[size = 8]
  #[liberty(name)]
  variable2: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  #[size = 0]
  #[liberty(extra_ctx)]
  extra_ctx: (),
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  attributes: Attributes,
  #[liberty(simple(type = Option))]
  next_state: Option<ArcStr>,
}
impl GroupFn for FF {}
impl NamedGroup for FF {
  #[inline]
  fn parse_set_name(
    builder: &mut Self::Builder,
    mut v: Vec<&str>,
  ) -> Result<(), crate::ast::IdError> {
    let l = v.len();
    if l != 2 {
      return Err(crate::ast::IdError::length_dismatch(2, l, v));
    }
    v.pop().map_or(
      Err(crate::ast::IdError::Other("Unkown pop error".into())),
      |variable2| {
        v.pop().map_or(
          Err(crate::ast::IdError::Other("Unkown pop error".into())),
          |variable1| {
            builder.variable1 = variable1.into();
            builder.variable2 = variable2.into();
            Ok(())
          },
        )
      },
    )
  }
  #[inline]
  fn fmt_name<T: Write, I: crate::ast::Indentation>(
    &self,
    f: &mut crate::ast::CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    write!(f, "{}, {}", self.variable1, self.variable2)
  }
}

#[derive(Debug)]
#[derive(liberty_macros::Group)]
pub(crate) struct Cell {
  #[liberty(name)]
  name: ArcStr,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  extra_ctx: (),
  /// group undefined attributes
  #[liberty(attributes)]
  attributes: Attributes,
  #[liberty(simple(type = Option))]
  area: Option<NotNan<f64>>,
  #[liberty(group(type = Set))]
  ff: GroupSet<FF>,
  #[liberty(group(type = Set))]
  pin: GroupSet<Pin>,
  #[liberty(group(type = Option))]
  statetable: Option<Statetable>,
}
impl GroupFn for Cell {}

#[cfg(test)]
mod test {
  use crate::ast::{DefaultIndentation, Group};

  use super::*;
  #[test]
  fn timing_test() {
    _ = crate::ast::test_parse_fmt::<Timing>(
      r#"(w){
        // www
        /* com
        ment2 */
        t1 : "combinational";
        values ( \
            "1,2,3,4"\ // comment1
        );
    }
    "#,
      r#"
liberty_db::common::demo::Timing () {
| values ("1.0, 2.0, 3.0, 4.0");
| t1 : combinational;
}"#,
    );
    _ = crate::ast::test_parse_fmt::<Timing>(
      r#"( w ){
        t1: ombinational;
        t2: combinational;
        values ( \
            "-1e2,2,3"\
        );
        }
    "#,
      r#"
liberty_db::common::demo::Timing () {
| values ("-100.0, 2.0, 3.0");
| t2 : combinational;
| t1 : ombinational; /* user defined attribute */
}"#,
    );
  }

  #[test]
  fn pin_test() {
    _ = crate::ast::test_parse_fmt::<Pin>(
      r#"(A){
        timing(w){
            t1: combinational;
        }
    }
    "#,
      r#"
liberty_db::common::demo::Pin (A) {
| timing () {
| | values ("0.0");
| | t1 : combinational;
| }
}"#,
    );
    _ = crate::ast::test_parse_fmt::<Pin>(
      r#"(B){
        timing(w){
            t1: combinational;
        }
    }
    "#,
      r#"
liberty_db::common::demo::Pin (B) {
| timing () {
| | values ("0.0");
| | t1 : combinational;
| }
}"#,
    );
  }
  #[test]
  fn cell_test() {
    use crate::ast::GroupAttri;
    _ = crate::ast::test_parse_fmt::<Cell>(
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
                    "4 , 5 , 6");
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
      r#"
liberty_db::common::demo::Cell (INV) {
| area : 5.4;
| ff (IQ, IQN) {
| | next_state : "!A";
| }
| pin (A) {
| | timing () {
| | | values ("0.0");
| | | t1 : combinational;
| | }
| }
| pin (Y) {
| | timing () {
| | | values ("0.0");
| | | t1 : foo_error; /* user defined attribute */
| | | test_table ("1,2,", "4,5,6", "4 , 5 , 6"); /* user defined attribute */
| | }
| }
| statetable ("CLK EN SE", ENL) {
| | table : "H   L  L : - : L ,\
| |          H   L  H : - : H ,\
| |          H   H  L : - : H ,\
| |          H   H  H : - : H ,\
| |          L   -  - : - : N";
| }
}"#,
    );
    let mut cell = crate::ast::test_parse_fmt::<Cell>(
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
      r#"
liberty_db::common::demo::Cell (INV) {
| area : 5.4;
| undefine_area : 5.4; /* user defined attribute */
| undefine_pin (C) {  /* user defined attribute */
| | timing (w) {  /* user defined attribute */
| | | t1 : combinational; /* user defined attribute */
| | }
| }
| pin (A) {
| | timing () {
| | | values ("0.0");
| | | t1 : combinational;
| | }
| }
}"#,
    );
    cell.comments_this_entry().or_insert("xc".into());
    cell.comments_area_entry().or_insert("xc".into());
    println!("{}", cell.display());
  }
}
