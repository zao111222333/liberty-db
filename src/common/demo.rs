//! cargo expand common::demo
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
    if let Ok((_,(title,group))) = 
    Timing::nom_parse(r#"(w){
        t1 : "combinational";
        values ( \
            1,"2,3",4,\ // comment1
            5,\ /* comment2 */
            6\ /* comment3 */
        );
    }
    "#,&mut n) {
        println!("{:?}",group);
        println!("{:?}",group.to_wrapper(title.into_iter().map(ToString::to_string).collect()
        ));
        println!("{n}");
    }
    if let Ok((_,(title,group))) = 
    Timing::nom_parse(r#"(w){
        t1: ombinational;
        t2: combinational;
        values ( \
            1,"2,3,",\
        );
        }
    "#,&mut n){
        println!("{:?}",group);
        println!("{:?}",group.to_wrapper(title.into_iter().map(ToString::to_string).collect()
        ));
        println!("{n}");
    }
}

#[test]
fn pin_test(){
    use crate::ast::GroupAttri;
    let mut n= 1;
    if let Ok((_,(title,group))) = 
    Pin::nom_parse(r#"(A){
        timing(w){
            t1: combinational;
        }
    }
    "#,&mut n){
        println!("{:?}",group);
        println!("{:?}",group.to_wrapper(title.into_iter().map(ToString::to_string).collect()
        ));
        println!("{n}");
    }
    if let Ok((_,(title,group))) = 
    Pin::nom_parse(r#"(B){
        timing(w){
            t1: combinational;
        }
    }
    "#,&mut n){
        println!("{:?}",group);
        println!("{:?}",group.to_wrapper(title.into_iter().map(ToString::to_string).collect()
        ));
        println!("{n}");
    }
}
#[derive(Default,Debug)]
#[derive(liberty_macros::GroupHashed,liberty_macros::NameIdx)]
struct Pin{
    _undefined: crate::ast::UndefinedAttributes,
    #[arrti_type(group)]
    timing: <Timing as crate::ast::GroupAttri>::Set,
}

#[test]
fn cell_test(){
    use crate::ast::GroupAttri;
    let mut n= 1;
    if let Ok((_,(title,group))) = 
    Cell::nom_parse(r#"(INV){
        area : 5.4;
          pin(A){
              timing(w){
                  t1: combinational;
              }
          }
          pin(Y){
              timing(){
                  t1: combinational;
              }
          }
      }
    "#,&mut n){
        println!("{:?}",group);
        println!("{:?}",group.to_wrapper(title.into_iter().map(ToString::to_string).collect()
        ));
        println!("{n}");
    }
    match Cell::nom_parse(r#"(INV){
        undefine_area : 5.4;
        undefine_pin(C){
            timing(w){
                t1: combinational;
            }
        }
        pin("A"){
            timing(w){
                t1: combinational;
            }
        }
        pin(A,Y){
            timing(w){
                t1: combinational;
            }
        }
    }
    "#,&mut n){
        Ok((_,(title,group)))=>{
            println!("{:?}",group);
            println!("{:?}",group.to_wrapper(title.into_iter().map(ToString::to_string).collect()
            ));
            println!("{n}");
        }
        Err(e) => println!("{:?}",e),
    }
}
#[derive(Default,Debug)]
#[derive(liberty_macros::GroupHashed)]
#[derive(liberty_macros::NameIdx)]
struct Cell{
  _undefined: crate::ast::UndefinedAttributes,
  #[arrti_type(simple)]
  area: Option<f64>,
  #[arrti_type(group_hashed)]
  pin: <Pin as crate::ast::GroupAttri>::Set,
}