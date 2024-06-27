// //! cargo expand common::demo

// use crate::{
//   ast::{AttributeList, GroupAttri, GroupComments, GroupFn, NamedGroup},
//   cell::Statetable,
//   timing::TimingType,
//   GroupSet,
// };
// use crate::ArcStr

// #[derive(Default, Debug, Clone)]
// // #[derive(liberty_macros::Group)]
// #[derive(liberty_macros::Nothing)]
// pub(crate) struct Timing {
//   /// group undefined attributes
//   #[liberty(undefined)]
//   undefined: AttributeList,
//   /// group comments
//   #[liberty(comments)]
//   comments: GroupComments<Self>,
//   #[liberty(complex)]
//   values: Vec<f64>,
//   #[liberty(simple(type = Option))]
//   t1: Option<TimingType>,
//   #[liberty(simple(type = Option))]
//   t2: Option<TimingType>,
// }
// #[doc(hidden)]
// #[derive(Default, Debug, Clone)]
// pub struct TimingComments {
//   pub name: crate::ast::AttriComment,
//   pub _undefined_bgn: crate::ast::AttriComment,
//   pub _undefined_end: crate::ast::AttriComment,
//   pub values: crate::ast::AttriComment,
//   pub t1: crate::ast::AttriComment,
//   pub t2: crate::ast::AttriComment,
// }
// #[doc(hidden)]
// impl crate::ast::GroupAttri for Timing {
//   type Name = ();
//   type Comments = TimingComments;
//   #[inline]
//   fn name(&self) -> Self::Name {
//     ()
//   }
//   #[inline]
//   fn set_name(&mut self, name: Self::Name) {}
//   fn fmt_liberty<T: std::fmt::Write>(
//     &self,
//     key: &str,
//     f: &mut crate::ast::CodeFormatter<'_, T>,
//   ) -> std::fmt::Result {
//     use itertools::Itertools;
//     use std::fmt::Write;
//     f.write_fmt(format_args!("\n{0} () {{", key))?;
//     f.indent(1);
//     crate::ast::Format::liberty(&self.comments.values, "", f)?;
//     crate::ast::ComplexAttri::fmt_liberty(&self.values, "values", f)?;
//     if let Some(simple) = &self.t1 {
//       crate::ast::Format::liberty(&self.comments.t1, "", f)?;
//       crate::ast::SimpleAttri::fmt_liberty(simple, "t1", f)?;
//     }
//     if let Some(simple) = &self.t2 {
//       crate::ast::Format::liberty(&self.comments.t2, "", f)?;
//       crate::ast::SimpleAttri::fmt_liberty(simple, "t2", f)?;
//     }
//     if !self.undefined.is_empty() {
//       <crate::ast::AttriComment as crate::ast::Format>::liberty(
//         &self.comments._undefined_bgn,
//         "",
//         f,
//       )?;
//       crate::ast::liberty_attr_list(&self.undefined, f)?;
//       <crate::ast::AttriComment as crate::ast::Format>::liberty(
//         &self.comments._undefined_end,
//         "",
//         f,
//       )?;
//     }
//     f.dedent(1);
//     f.write_fmt(format_args!("\n}}"))
//   }
//   fn nom_parse<'a>(
//     i: &'a str,
//     line_num: &mut usize,
//   ) -> nom::IResult<&'a str, Result<Self, crate::ast::IdError>, nom::error::Error<&'a str>>
//   {
//     let (mut input, title) = crate::ast::parser::title(i, line_num)?;
//     let mut res = Self::default();
//     res
//       .comments
//       ._undefined_bgn
//       .push("Undefined attributes from here".into());
//     res
//       .comments
//       ._undefined_end
//       .push("Undefined attributes end here".into());
//     loop {
//       match crate::ast::parser::key(input) {
//         Err(nom::Err::Error(_)) => {
//           (input, _) = crate::ast::parser::end_group(input)?;
//           <Self as crate::ast::GroupFn>::post_process(&mut res);
//           return Ok((input, Ok(res)));
//         }
//         Err(e) => return Err(e),
//         Ok((_input, key)) => {
//           input = _input;
//           match key {
//             "values" => {
//               let complex_res: _;
//               (input, complex_res) =
//                 <_ as crate::ast::ComplexAttri>::nom_parse(input, line_num)?;
//               match complex_res {
//                 Ok(complex) => res.values = complex,
//                 Err(e) => {
//                   {
//                     // ::std::io::_print(format_args!(
//                     //   "Line={0}; Key={1}; Err={2}\n",
//                     //   line_num, key, e,
//                     // ));
//                   };
//                 }
//               }
//             }

//             // "t1" => {
//             //   let simple_res: _;
//             //   (input, simple_res) =
//             //     <_ as crate::ast::SimpleAttri>::nom_parse(input, line_num)?;
//             //   match simple_res {
//             //     Ok(simple) => {
//             //       res.t1 = Some(simple);
//             //     }
//             //     Err((e, undefined)) => {
//             //       {
//             //         // ::std::io::_print(format_args!(
//             //         //   "Line={0}; Key={1}; Value={2:?}; Err={3}\n",
//             //         //   line_num, key, undefined, e,
//             //         // ));
//             //       };
//             //       res.undefined.push((key.into(), undefined));
//             //     }
//             //   }
//             // }
//             "t2" => {
//               let simple_res: _;
//               (input, simple_res) =
//                 <_ as crate::ast::SimpleAttri>::nom_parse(input, line_num)?;
//               match simple_res {
//                 Ok(simple) => {
//                   res.t2 = Some(simple);
//                 }
//                 Err((e, undefined)) => {
//                   {
//                     // ::std::io::_print(format_args!(
//                     //   "Line={0}; Key={1}; Value={2:?}; Err={3}\n",
//                     //   line_num, key, undefined, e,
//                     // ));
//                   };
//                   res.undefined.push((ArcStr::from(key), undefined));
//                 }
//               }
//             }
//             _ => {
//               let undefined: crate::ast::AttriValue;
//               (input, undefined) = crate::ast::parser::undefine(input, line_num)?;
//               res.undefined.push((ArcStr::from(key), undefined));
//               let n: usize;
//               (input, n) = crate::ast::parser::comment_space_newline(input)?;
//               *line_num += n;
//             } // _ => {}
//           }
//         }
//       }
//     }
//   }
// }
// impl GroupFn for Timing {}
// // #[mut_set_derive::item(
// //   sort,
// //   macro(derive(Debug, Clone,Default);)
// // )]
// // #[derive(Default, Debug, Clone)]
// // #[derive(liberty_macros::Group)]
// // pub(crate) struct Pin {
// //   #[id]
// //   #[liberty(name)]
// //   name: ArcStr,
// //   /// group comments
// //   #[liberty(comments)]
// //   comments: GroupComments<Self>,
// //   /// group undefined attributes
// //   #[liberty(undefined)]
// //   undefined: AttributeList,
// //   #[liberty(group(type=Vec))]
// //   timing: Vec<Timing>,
// // }
// // impl GroupFn for Pin {}
// // #[mut_set_derive::item(
// //   sort,
// //   macro(derive(Debug, Clone,Default);)
// // )]
// // #[derive(Default, Debug, Clone)]
// // #[derive(liberty_macros::Group)]
// // pub(crate) struct FF {
// //   #[id]
// //   #[liberty(name)]
// //   var1: ArcStr,
// //   #[id]
// //   #[liberty(name)]
// //   var2: ArcStr,
// //   /// group comments
// //   #[liberty(comments)]
// //   comments: GroupComments<Self>,
// //   /// group undefined attributes
// //   #[liberty(undefined)]
// //   undefined: AttributeList,
// //   #[liberty(simple(type = Option))]
// //   next_state: Option<ArcStr>,
// // }
// // impl GroupFn for FF {}
// // impl NamedGroup for FF {
// //   #[inline]
// //   fn parse(mut v: Vec<ArcStr>) -> Result<Self::Name, crate::ast::IdError> {
// //     let l = v.len();
// //     if l != 2 {
// //       return Err(crate::ast::IdError::LengthDismatch(2, l, v));
// //     }
// //     if let Some(var2) = v.pop() {
// //       if let Some(var1) = v.pop() {
// //         Ok(Self::Name { var1, var2 })
// //       } else {
// //         Err(crate::ast::IdError::Other("Unkown pop error".into()))
// //       }
// //     } else {
// //       Err(crate::ast::IdError::Other("Unkown pop error".into()))
// //     }
// //   }
// //   #[inline]
// //   fn name2vec(name: Self::Name) -> Vec<ArcStr> {
// //     vec![name.var1, name.var2]
// //   }
// // }

// // #[derive(Default, Debug)]
// // #[derive(liberty_macros::Group)]
// // pub(crate) struct Cell {
// //   #[liberty(name)]
// //   name: ArcStr,
// //   /// group comments
// //   #[liberty(comments)]
// //   comments: GroupComments<Self>,
// //   /// group undefined attributes
// //   #[liberty(undefined)]
// //   undefined: AttributeList,
// //   #[liberty(simple(type = Option))]
// //   area: Option<f64>,
// //   #[liberty(group(type=Set))]
// //   ff: GroupSet<FF>,
// //   #[liberty(group(type=Set))]
// //   pin: GroupSet<Pin>,
// //   #[liberty(group(type = Option))]
// //   statetable: Option<Statetable>,
// // }
// // impl GroupFn for Cell {}
// // #[test]
// // fn timing_test() {
// //   let _ = crate::ast::test_parse_group::<Timing>(
// //     r#"(w){
// //         // www
// //         /* com
// //         ment2 */
// //         t1 : "combinational";
// //         values ( \
// //             1,"2,3",4,\ // comment1
// //             5,\ /* comment2 */
// //             6\ /* comment3 */
// //         );
// //     }
// //     "#,
// //   );
// //   let _ = crate::ast::test_parse_group::<Timing>(
// //     r#"( w ){
// //         t1: ombinational;
// //         t2: combinational;
// //         values ( \
// //             -1e2,"2,3,",\
// //             1,"2,3,",\
// //         );
// //         }
// //     "#,
// //   );
// // }

// // #[test]
// // fn pin_test() {
// //   let _ = crate::ast::test_parse_group::<Pin>(
// //     r#"(A){
// //         timing(w){
// //             t1: combinational;
// //         }
// //     }
// //     "#,
// //   );
// //   let _ = crate::ast::test_parse_group::<Pin>(
// //     r#"(B){
// //         timing(w){
// //             t1: combinational;
// //         }
// //     }
// //     "#,
// //   );
// // }

// // // FIXME!
// // #[test]
// // fn cell_test() {
// //   let _ = crate::ast::test_parse_group::<Cell>(
// //     r#"(INV){
// //         // should ok
// //         area : 5.4;
// //         // should ok
// //         ff(IQ,IQN){
// //           next_state: "!A";
// //         }
// //         // should ok
// //         pin(A){
// //           timing(w){
// //             t1: combinational;
// //           }
// //         }
// //         // should ok
// //         pin(Y){
// //             timing(){
// //                 // should error
// //                 t1: foo_error;
// //                 test_table (\
// //                     "1,2,",\
// //                     "4,5,6",\
// //                     4 , 5 , 6);
// //             }
// //         }
// //         statetable ("CLK EN SE",ENL) {
// //             table : "	H   L  L : - : L ,\
// //             H   L  H : - : H ,\
// //             H   H  L : - : H ,\
// //             H   H  H : - : H ,\
// //             L   -  - : - : N ";
// //         }
// //       }
// //     "#,
// //   );
// //   let (g, _) = &mut crate::ast::test_parse_group::<Cell>(
// //     r#"(INV){
// //         // should error
// //         area : 5.4;
// //         undefine_area : 5.4;
// //         // should error
// //         undefine_pin(C){
// //             timing(w){
// //                 t1: combinational;
// //             }
// //         }
// //         // should ok
// //         pin("A"){
// //             timing(w){
// //                 t1: combinational;
// //             }
// //         }
// //         pin("A"){
// //             timing(w){
// //                 t2: combinational;
// //             }
// //         }
// //         // should error
// //         pin(A,Y){
// //             timing(w){
// //                 t1: combinational;
// //             }
// //         }
// //     }
// //     "#,
// //   );
// //   g.comments.area.push("xc".to_owned());
// //   g.comments.area.push("xc".to_owned());
// //   let mut output = String::new();
// //   let mut f = crate::ast::CodeFormatter::new(&mut output, "| ");
// //   if let Err(e) = GroupAttri::fmt_liberty(g, std::any::type_name::<Cell>(), &mut f) {
// //     panic!("{e}");
// //   }
// //   println!("{}", output);
// // }
