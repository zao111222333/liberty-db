use crate::pin::Pin;
/// cell
#[derive(Debug,Default)]
#[derive(liberty_macros::NameIdx)]
#[derive(liberty_macros::GroupHashed)]
pub struct Cell {
    #[idx_len(1)]
    _idx: Box<<Self as crate::ast::HashedGroup>::Idx>,
    _undefined: crate::ast::UndefinedAttributes,
    
    #[arrti_type(simple)]
    pub area: Option<f64>,
    #[arrti_type(group_hashed)]
    pub pin: <Pin as crate::ast::GroupAttri>::Set,
    #[arrti_type(group_hashed)]
    pub statetable: <Statetable as crate::ast::GroupAttri>::Set,
}



#[derive(Default,Debug)]
#[derive(liberty_macros::GroupHashed)]
#[derive(liberty_macros::NameIdx)]
pub struct Statetable{
  #[idx_len(2)]
  _idx: Box<<Self as crate::ast::HashedGroup>::Idx>,
  _undefined: crate::ast::UndefinedAttributes,
  #[arrti_type(simple)]
  table: Option<Table>,
}

#[derive(Default,Debug)]
pub struct Table{
    pub v: Vec<String>,
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.v, f)
    }
}

impl crate::ast::SimpleAttri for Table {
    type Error=std::fmt::Error;
    fn parse(s: &str)->Result<Self, Self::Error> {
        Ok(
            Self { 
                v: s.split('\n').filter_map(|line| {
                    let _l = line.trim().trim_end_matches(|c| c == '\\' || c == ',' || c == ' ');
                    if _l==""{None}else{Some(_l.to_owned())}
                }).collect()
            } 
        )
    }
    fn nom_parse<'a>(
        i: &'a str, line_num: &mut usize
    ) -> nom::IResult<&'a str, Result<Self,(Self::Error,crate::ast::AttriValue)>, nom::error::Error<&'a str>> {
        let (input,simple_multi) = crate::ast::parser::simple_multi(i,line_num)?;
        match Self::parse(simple_multi){
        Ok(s) => Ok((input,Ok(s))),
        Err(e) => Ok((
            input,
            Err((e,crate::ast::AttriValue::Simple(simple_multi.to_string())))
        )),
        }
    }
}

#[test]
fn statetable_test(){
    use crate::ast::GroupAttri;
    let mut n= 1;
    match Statetable::nom_parse(r#"("CLK EN SE",ENL) {
            table : "	H   L  L : - : L ,\
            H   L  H : - : H ,\
            H   H  L : - : H ,\
            H   H  H : - : H ,\
            L   -  - : - : N ";
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
}