//!
//!  This crate implement `liberty` data structre in Rust.
//!

pub mod wrapper;


use std::hash::Hash;

use nom::{
  error::{Error, ContextError, FromExternalError, ParseError}, 
  IResult, 
  combinator::map, 
  sequence::tuple,
  character::streaming::char, branch::alt, InputLength, InputTake, Compare, bytes::streaming::take_while, InputIter,
};

#[derive(Debug)]
pub struct Group<'a>{
  title: Vec<&'a str>,
  value: hashbrown::HashMap<&'a str,(Comment<'a>, AttriValue<'a>)>,
}

pub type SimpleSingle<'a> = &'a str;
pub type SimpleMulti<'a> = Vec<&'a str>;
pub type Complex1d<'a> = Vec<&'a str>;
pub type Complex2d<'a> = Vec<Vec<&'a str>>;
pub type Comment<'a> = Vec<&'a str>;

#[derive(Debug)]
pub enum AttriValue <'a> {
  SimpleSingle(SimpleSingle<'a>),
  SimpleMulti(SimpleMulti<'a>),
  Complex1d(Complex1d<'a>),
  Complex2d(Complex2d<'a>),
  GroupList(Vec<Group<'a>>),
}

pub fn space<'a>(i: &'a str) -> IResult<&'a str, (), Error<&'a str>> 
{
  map(take_while(move |c| " \t\r".contains(c)), |_|())(i)
}

pub fn space_newline<'a>(i: &'a str) -> IResult<&'a str, usize, Error<&'a str>> 
{
  map(take_while(move |c| " \t\r\n".contains(c)), |s: &str|s.chars().filter(|&x| x == '\n').count())(i)
}

fn attri_group(){}
fn attri_complex(){}
// trait Fn(&'a str) -> IResult<&'a str, Group, Error<&'a str>> {
    
// }
// use phf::phf_map;
// fn empty_group_parser<'a>(i: &'a str, line_num: &mut usize) -> IResult<&'a str, Group<'a>, Error<&'a str>>{
//   group_parser(Self::MAP)(i, line_num)
// }
pub fn group_parser<'a>(type_map: Option<phf::Map<&'static str, AttriType>>) -> 
  impl Fn(&'a str, &mut usize) -> IResult<&'a str, Group<'a>, Error<&'a str>>
{
  move |i: &'a str, line_num: &mut usize| {
    // find title
    let res_title = map(
      tuple((
        space,
        char('('),
        char(')'),
      )), 
      |x|todo!()
    ) (i);
    let mut g = Group{ title: todo!(), value: todo!() };
    let key = "www";
    match type_map {
        Some(map) => match map.get(key){
          Some(attri) => match attri {
              AttriType::SimpleSingle => todo!(),
              AttriType::SimpleMulti => todo!(),
              AttriType::Complex1d => todo!(),
              AttriType::Complex2d => todo!(),
              AttriType::ComplexSpecial => todo!(),
              AttriType::Group(child_parser) => {
                // child_parser()
              },
          },
          None => todo!(),
          // alt((
          //   AttriType::SimpleSingle => todo!(),
          //   AttriType::Complex1d => todo!(),
          //   AttriType::Group(child_parser) => {
          //     child_parser()
          //   },
          //   AttriType::ComplexSpecial => todo!(),
          //   AttriType::SimpleMulti => todo!(),
          // )),
      },
        None => todo!(),
    }
    
    // let tag_len = tag.input_len();
    // let t = tag.clone();

    // let res: IResult<_, _, Error> = match i.compare(t) {
    //   CompareResult::Ok => Ok(i.take_split(tag_len)),
    //   CompareResult::Incomplete => Err(Err::Incomplete(Needed::new(tag_len - i.input_len()))),
    //   CompareResult::Error => {
    //     let e: ErrorKind = ErrorKind::Tag;
    //     Err(Err::Error(Error::from_error_kind(i, e)))
    //   }
    // };
    // res
    todo!()
  }
}

// type GroupFn<'a> = fn(&'a str)->IResult<&'a str, Group, Error<&'a str>>;
pub type GroupFn = &'static dyn for<'a>Fn(&'a str, &mut usize) -> IResult<&'a str, Group<'a>, Error<&'a str>>;
// pub type GroupFn = Box<dyn for<'a>Fn(&'a str) -> IResult<&'a str, Group<'a>, Error<&'a str>>>;

// #[derive(Debug)]
pub enum AttriType {
  SimpleSingle,
  SimpleMulti,
  Complex1d,
  Complex2d,
  ComplexSpecial,
  Group(GroupFn),
}
// phf::Map<&'static str, AttriType>
// type Fn = for<'a> fn(&'a str) -> Result<(&'a str, T), nom::Err<nom::error::Error<&'a str>>>;
pub trait LibertyAttri {
  // const T: AttriType;
  fn parser<'a>(i: &'a str) -> IResult<&'a str, Self, Error<&str>> 
    where
      Self: Sized;
}


pub trait SimpleSingleAttri<'a>: From<SimpleSingle<'a>> {}
pub trait SimpleMultiAttri<'a>: From<SimpleMulti<'a>> {}
pub trait Complex1dAttri<'a>: From<Complex1d<'a>> {}
pub trait Complex2dAttri<'a>: From<Complex2d<'a>> {}
pub trait GroupAttri<'a>: From<Group<'a>> {}

// pub struct SimpleAttri;

// impl SimpleAttri {
//   #[inline]
//   pub fn single<'a,E>(
//     i: &'a str,
//   ) -> IResult<&'a str, &'a str, E> 
//   where
//     E: ParseError<&'a str> 
//       + ContextError<&'a str> 
//       + FromExternalError<&'a str, E>
//   {
//     map(
//       tuple((
//         char(':'),
//         wrapper::space_newline_complex,
//         alt((
//           wrapper::unquote,
//           wrapper::key,
//         )),
//         wrapper::space,
//         char(';'),
//       )),
//     |(_,_,s,_,_)| s,
//     )(i)
//   }
// }