use crate::timing::TimingType;
use liberty_parser::{
    parser,
    SimpleAttri, ComplexAttri, GroupAttri, GroupIdx, GroupParser, UndefinedAttributes, IdxError,
};
#[derive(Default)]
#[derive(Debug)]
struct Timing{
    _undefined: UndefinedAttributes,
    area: Option<f64>,
    value: Vec<f64>,
    t1: Option<TimingType>,
    t2: Option<TimingType>,
}
#[test]
fn timing_test(){
    let mut n= 1;
    let mut set = vec![];
    println!("{:?}",Timing::parse_push2set(r#"(w){
    t1 : "combinational";
    area : 5.4;
    value ( \
        1,"2,3",4,\ // comment1
        5,\ /* comment2 */
        6\ /* comment3 */
    );
    }
    "#,&mut set,&mut n));
    println!("{n}");
    println!("{set:?}");
    println!("{:?}",Timing::parse_push2set(r#"(w){
        t1: ombinational;
        t2: combinational;
        value ( \
            1,"2,3,",\
        );
        }
    "#,&mut set,&mut n));
    println!("{n}");
    println!("{set:?}");
}
#[derive(Hash)]
#[derive(Debug)]
#[derive(Default)]
pub(crate) struct TimingIdx;
impl GroupIdx<Timing> for TimingIdx {
    fn new<'a>(_:&Timing, _: Vec<&'a str>)-> Result<Self,IdxError<'a>> {
        Ok(Self{})
    }
}

impl GroupAttri for Timing {
    type Idx=TimingIdx;
    type Set=Vec<Self>;
    fn push_self<'a>(self, _: Vec<&'a str>, set: &mut Self::Set) -> Result<(), IdxError<'a>> {
        Ok(set.push(self))
    }
    fn add_undefine_attri(&mut self, key: &str, attri: liberty_parser::AttriValue) {
        self._undefined.push((key.to_string(),attri));
    }
}
impl GroupParser for Timing {
    fn parse_push2set<'a>(
        i: &'a str, set: &mut Self::Set, line_num: &mut usize
    ) -> nom::IResult<&'a str, (), nom::error::Error<&'a str>> {
        let (mut input,title) = parser::title(i,line_num)?;
        let mut res = Self::default();
        loop {
            match parser::key(input){
                Err(nom::Err::Error(_)) => {
                    (input,_) = parser::end_group(input)?;
                    if let Err(e) = res.push_self(title, set){
                        println!("{e}, Line={line_num}");
                    }
                    return Ok((input, ()))
                },
                Err(e) => return Err(e),
                Ok((_input,key)) => {
                    input = _input;
                    match key {
                        "area" => {
                            let simple: &str;
                            (input,simple) = parser::simple(input,line_num)?;
                            match f64::parse(simple) {
                                Ok(s) => {
                                    res.area=Some(s);
                                },
                                Err(e) => {
                                    println!("Line={line_num}; Key={key}; Value={simple}; Err={e}");
                                    res.add_undefine_attri(
                                        key,
                                        liberty_parser::AttriValue::Simple(simple.to_string())
                                    );
                                },
                            }
                        },
                        "value" => {
                            let complex: Vec<&str>;
                            (input,complex) = parser::complex(input,line_num)?;
                            match Vec::<f64>::parse(&complex) {
                                Ok(v) => {
                                    res.value=v;
                                },
                                Err(e) => {
                                    println!("Line={line_num}; Key={key}; Value={complex:?}; Err={e}");
                                    res.add_undefine_attri(
                                        key,
                                        liberty_parser::AttriValue::Complex(complex.into_iter().map(String::from).collect()),
                                    );
                                },
                            }
                        },
                        "t1" => {
                            let simple: &str;
                            (input,simple) = parser::simple(input,line_num)?;
                            match TimingType::parse(simple) {
                                Ok(s) => {
                                    res.t1=Some(s);
                                },
                                Err(e) => {
                                    println!("Line={line_num}; Key={key}; Value={simple}; Err={e}");
                                    res.add_undefine_attri(
                                        key,
                                        liberty_parser::AttriValue::Simple(simple.to_string())
                                    );
                                },
                            }
                        },
                        "t2" => {
                            let simple: &str;
                            (input,simple) = parser::simple(input,line_num)?;
                            match TimingType::parse(simple) {
                                Ok(s) => {
                                    res.t2=Some(s);
                                },
                                Err(e) => {
                                    println!("Line={line_num}; Key={key}; Value={simple}; Err={e}");
                                    res.add_undefine_attri(
                                        key,
                                        liberty_parser::AttriValue::Simple(simple.to_string())
                                    );
                                },
                            }
                        },
                        _ => {
                            let undefine: liberty_parser::AttriValue;
                            (input,undefine) = parser::undefine(input)?;
                            println!("Line={line_num}; Undefinde Error; Key={key};");
                            res.add_undefine_attri(key, undefine)
                        },
                    }
                },
            }
        }
    }
}
#[test]
fn pin_test(){
    let mut n= 1;
    let mut set = hashbrown::HashMap::new();
    println!("{:?}",Pin::parse_push2set(r#"(A){
        timing(w){
            t1: combinational;
        }
    }
    "#,&mut set,&mut n));
    println!("{n}");
    println!("{set:?}");
    println!("{:?}",Pin::parse_push2set(r#"(B){
        timing(w){
            t1: combinational;
        }
    }
    "#,&mut set,&mut n));
    println!("{n}");
    println!("{set:?}");
}
#[derive(Default,Debug)]
struct Pin{
    _undefined: UndefinedAttributes,
    timing: <Timing as GroupAttri>::Set,
}
#[derive(Hash)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
struct PinIdx{
    name: String,
}
impl GroupIdx<Pin> for PinIdx {
    #[inline]
    fn new<'a>(_: &Pin, mut title: Vec<&'a str>)-> Result<Self,IdxError<'a>> {
        let l=title.len();
        if l!=1{
            return Err(IdxError::TitleLenMismatch(1,l,title));
        }
        Ok(Self { name: title.pop().unwrap().to_string() })
    }
}
impl GroupAttri for Pin {
    type Idx=PinIdx;
    type Set=hashbrown::HashMap<Self::Idx,Self>;
    fn push_self<'a>(self, title: Vec<&'a str>, set: &mut Self::Set) -> Result<(), IdxError<'a>> {
        match Self::Idx::new(&self, title){
            Ok(idx) => 
                if let Some(_)= set.insert(idx,self){
                    Err(IdxError::RepeatIdx)
                }else{
                    Ok(())
                },
            Err(e) => Err(e),
        } 
    }
    fn add_undefine_attri(&mut self, key: &str, attri: liberty_parser::AttriValue) {
        self._undefined.push((key.to_string(),attri));
    }
}
impl GroupParser for Pin {
    fn parse_push2set<'a>(
        i: &'a str, set: &mut Self::Set, line_num: &mut usize
    ) -> nom::IResult<&'a str, (), nom::error::Error<&'a str>> {
        let (mut input,title) = parser::title(i,line_num)?;
        let mut res = Self::default();
        loop {
            match parser::key(input){
                Err(nom::Err::Error(_)) => {
                    (input,_) = parser::end_group(input)?;
                    if let Err(e) = res.push_self(title, set){
                        println!("Line={line_num}, error={e}");
                    }
                    return Ok((input, ()))
                },
                Err(e) => return Err(e),
                Ok((_input,key)) => {
                    input = _input;
                    match key {
                        "timing" => {
                            (input,_) = Timing::parse_push2set(input, &mut res.timing, line_num)?;
                            let n: usize;
                            (input,n) = parser::space_newline(input)?;
                            *line_num+=n;
                        },
                        _ => {
                            let undefine: liberty_parser::AttriValue;
                            (input,undefine) = parser::undefine(input)?;
                            res.add_undefine_attri(key, undefine)
                        },
                    }
                },
            }
        }
    }
}
#[test]
fn cell_test(){
    let mut n= 1;
    let mut set = hashbrown::HashMap::new();
    println!("{:?}",Cell::parse_push2set(r#"(INV){
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
    "#,&mut set,&mut n));
    println!("{n}");
    println!("{set:?}");
    println!("{:?}",Cell::parse_push2set(r#"(INV){
        pin(){
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
    "#,&mut set,&mut n));
    println!("{n}");
    println!("{set:?}");
}
#[derive(Default,Debug)]
struct Cell{
    _undefined: UndefinedAttributes,
    pin: <Pin as GroupAttri>::Set,
}

#[derive(Hash)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
struct CellIdx{
    name: String,
}
impl GroupIdx<Cell> for CellIdx {
    #[inline]
    fn new<'a>(_: &Cell, mut title: Vec<&'a str>)-> Result<Self,IdxError<'a>> {
        let l=title.len();
        if l!=1{
            return Err(IdxError::TitleLenMismatch(1,l,title));
        }
        Ok(Self { name: title.pop().unwrap().to_string() })
    }
}
impl GroupAttri for Cell {
    type Idx=CellIdx;
    type Set=hashbrown::HashMap<Self::Idx,Self>;
    fn push_self<'a>(self, title: Vec<&'a str>, set: &mut Self::Set) -> Result<(), IdxError<'a>> {
        match Self::Idx::new(&self, title){
            Ok(idx) => 
                if let Some(_)= set.insert(idx,self){
                    Err(IdxError::RepeatIdx)
                }else{
                    Ok(())
                },
            Err(e) => Err(e),
        } 
    }
    fn add_undefine_attri(&mut self, key: &str, attri: liberty_parser::AttriValue) {
        self._undefined.push((key.to_string(),attri));
    }
}
impl GroupParser for Cell {
    fn parse_push2set<'a>(
        i: &'a str, set: &mut Self::Set, line_num: &mut usize
    ) -> nom::IResult<&'a str, (), nom::error::Error<&'a str>> {
        let (mut input,title) = parser::title(i,line_num)?;
        let mut res = Self::default();
        loop {
            match parser::key(input){
                Err(nom::Err::Error(_)) => {
                    (input,_) = parser::end_group(input)?;
                    if let Err(e) = res.push_self(title, set){
                        println!("Line={line_num}, error={e}");
                    }
                    return Ok((input, ()))
                },
                Err(e) => return Err(e),
                Ok((_input,key)) => {
                    input = _input;
                    match key {
                        "pin" => {
                            (input,_) = Pin::parse_push2set(input, &mut res.pin, line_num)?;
                            let n: usize;
                            (input,n) = parser::space_newline(input)?;
                            *line_num+=n;
                        },
                        _ => {
                            let undefine: liberty_parser::AttriValue;
                            (input,undefine) = parser::undefine(input)?;
                            res.add_undefine_attri(key, undefine)
                        },
                    }
                },
            }
        }
    }
}