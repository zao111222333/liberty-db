//! cargo expand common::demo

use std::fmt::Pointer;

use crate::timing::TimingType;

#[derive(Default)]
#[derive(Debug)]
struct Values {
  v: Vec<f64>,
}
impl crate::ast::ComplexAttri for Values {
  type Error=std::num::ParseFloatError;
  #[inline]
  fn parse(v: &Vec<Vec<&str>>)->Result<Self, Self::Error> {
    match v.iter().flatten()
      .map(|s| s.parse())
      .collect() {
      Ok(v) => Ok(Self { v }),
      Err(e) => Err(e),
    }
  }

  fn is_empty(&self) -> bool {
    self.v.is_empty()
  }

  fn to_wrapper(&self) -> crate::ast::ComplexWrapper {
    vec![self.v.iter().map(|f|format!("{:.10E}",f)).collect()]
  }
}
#[derive(Default,Debug)]
#[derive(liberty_macros::Group)]
struct Timing{
  _undefined: crate::ast::UndefinedAttributes,
  #[arrti_type(complex)]
  values: Values,
  #[arrti_type(simple)]
  t1: Option<TimingType>,
  #[arrti_type(simple)]
  t2: Option<TimingType>,
}
#[test]
fn timing_test(){
    use crate::ast::GroupAttri;
    let mut n= 1;
    if let Ok((_,Ok(group))) = 
    Timing::nom_parse(r#"(w){
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
    "#,&mut n) {
        println!("{:?}",group);
        println!("{:?}",group.to_wrapper());
        println!("{n}");
    }
    match Timing::nom_parse(r#"( w ){
        t1: ombinational;
        t2: combinational;
        values ( \
            1,"2,3,",\
        );
        }
    "#,&mut n){
        Ok((_,Ok(group)))=>{
            println!("{:?}",group);
            println!("{:?}",group.to_wrapper());
            println!("{n}");
        },
        Ok((_,Err(e))) => panic!("{e}"),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn pin_test(){
    use crate::ast::GroupAttri;
    let mut n= 1;
    if let Ok((_,Ok(group))) = 
    Pin::nom_parse(r#"(A){
        timing(w){
            t1: combinational;
        }
    }
    "#,&mut n){
        println!("{:?}",group);
        println!("{:?}",group.to_wrapper());
        println!("{n}");
    }
    if let Ok((_,Ok(group))) = 
    Pin::nom_parse(r#"(B){
        timing(w){
            t1: combinational;
        }
    }
    "#,&mut n){
        println!("{:?}",group);
        println!("{:?}",group.to_wrapper());
        println!("{n}");
    }
}
#[derive(Default,Debug)]
#[derive(liberty_macros::NameIdx)]
#[derive(liberty_macros::GroupHashed)]
struct Pin{
    #[idx_len(1)]
    _idx: Box<<Self as crate::ast::HashedGroup>::Idx>,
    _undefined: crate::ast::UndefinedAttributes,
    #[arrti_type(group)]
    timing: <Timing as crate::ast::GroupAttri>::Set,
}
#[derive(Default,Debug)]
#[derive(liberty_macros::GroupHashed)]
#[derive(liberty_macros::NameIdx)]
struct Ff{
  #[idx_len(2)]
  _idx: Box<<Self as crate::ast::HashedGroup>::Idx>,
  _undefined: crate::ast::UndefinedAttributes,
  #[arrti_type(simple)]
  next_state: Option<String>,
}


#[test]
fn cell_test(){
    use crate::ast::GroupAttri;
    let mut n= 1;
    
    match Cell::nom_parse(r#"(INV){
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
    "#,&mut n){
        Ok((_,Ok(group))) =>{
            println!("{:?}",group);
            println!("{:?}",group.to_wrapper());
            println!("{n}");
        },
        Ok((_,Err(e))) => panic!("{:#?}",e),
        Err(e) => panic!("{:#?}",e),
    }
    if let Ok((_,Ok(group))) =  Cell::nom_parse(r#"(INV){
        // should error
        undefine_area : 5.4;
        // should error
        undefine_pin(C){
            timing(w){
                t1: combinational;
            }
        }
        // should error
        pin("A"){
            timing(w){
                t1: combinational;
            }
        }
        // should error
        pin(A,Y){
            timing(w){
                t1: combinational;
            }
        }
    }
    "#,&mut n){
        println!("{:?}",group);
        println!("{:?}",group.to_wrapper());
        println!("{n}");
    }
}
#[derive(Default,Debug)]
#[derive(liberty_macros::GroupHashed)]
#[derive(liberty_macros::NameIdx)]
struct Cell{
  #[idx_len(1)]
  _idx: Box<<Self as crate::ast::HashedGroup>::Idx>,
  _undefined: crate::ast::UndefinedAttributes,
  #[arrti_type(simple)]
  area: Option<f64>,
  #[arrti_type(group_hashed)]
  ff: <Ff as crate::ast::GroupAttri>::Set,
  #[arrti_type(group_hashed)]
  pin: <Pin as crate::ast::GroupAttri>::Set,
  #[arrti_type(group_hashed)]
  statetable: <crate::cell::Statetable as crate::ast::GroupAttri>::Set,
}
