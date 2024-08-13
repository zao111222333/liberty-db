#[cfg(test)]
use core::hash::Hash;
use std::{collections::HashMap, hash::Hasher};

use crate::{expression::*, units};
// use test_log::test;
// use log::info;

fn nand_ab() -> BooleanExpression {
  let port_a: BooleanExpression = Port::new("A").into();
  let port_b = Port::new("B").into();
  let exp_not_a_and_b: BooleanExpression = FunctionExpression::new(
    vec![FunctionExpression::new(
      vec![port_a, port_b],
      vec![None, None],
      vec![logic::Operator2::And],
    )
    .into()],
    vec![Some(logic::Operator1::Not)],
    vec![],
  )
  .into();
  exp_not_a_and_b
}
fn _0() -> BooleanExpression {
  FunctionExpression::new(
    vec![Port::new("A").into()],
    vec![Some(logic::Operator1::Logic0)],
    vec![],
  )
  .into()
}
fn _1() -> BooleanExpression {
  FunctionExpression::new(
    vec![Port::new("A").into()],
    vec![Some(logic::Operator1::Logic1)],
    vec![],
  )
  .into()
}
fn and_ab() -> BooleanExpression {
  let port_a: BooleanExpression = Port::new("A").into();
  let port_b = Port::new("B").into();
  let exp_not_a_and_b: BooleanExpression = FunctionExpression::new(
    vec![FunctionExpression::new(
      vec![port_a, port_b],
      vec![None, None],
      vec![logic::Operator2::And],
    )
    .into()],
    vec![None],
    vec![],
  )
  .into();
  exp_not_a_and_b
}
fn and_ac() -> BooleanExpression {
  let port_a: BooleanExpression = Port::new("A").into();
  let port_c = Port::new("C").into();
  let exp_not_a_and_b: BooleanExpression = FunctionExpression::new(
    vec![FunctionExpression::new(
      vec![port_a, port_c],
      vec![None, None],
      vec![logic::Operator2::And],
    )
    .into()],
    vec![None],
    vec![],
  )
  .into();
  exp_not_a_and_b
}
fn and_ba() -> BooleanExpression {
  let port_a: BooleanExpression = Port::new("A").into();
  let port_b = Port::new("B").into();
  let exp_not_a_and_b: BooleanExpression = FunctionExpression::new(
    vec![FunctionExpression::new(
      vec![port_b, port_a],
      vec![None, None],
      vec![logic::Operator2::And],
    )
    .into()],
    vec![None],
    vec![],
  )
  .into();
  exp_not_a_and_b
}
fn and_aa() -> BooleanExpression {
  let port_a: BooleanExpression = Port::new("A").into();
  let port_b = Port::new("A").into();
  let exp_not_a_and_b: BooleanExpression = FunctionExpression::new(
    vec![FunctionExpression::new(
      vec![port_a, port_b],
      vec![None, None],
      vec![logic::Operator2::And],
    )
    .into()],
    vec![None],
    vec![],
  )
  .into();
  exp_not_a_and_b
}

fn get_hash<T: core::hash::Hash>(s: T) -> u64 {
  use std::collections::hash_map::DefaultHasher;
  let mut hasher = DefaultHasher::new();
  s.hash(&mut hasher);
  hasher.finish()
}
lazy_static::lazy_static! {
    static ref HIGH:      logic::State = logic::State::H;
    static ref LOW:       logic::State = logic::State::L;
    static ref RISE_NONE: logic::State = logic::State::R;
    static ref RISE_BASE: logic::State = logic::State::Edge(logic::Edge::Rise(
                                                            Some(logic::ChangePattern::new(
                                                                units::second(1.0),
                                                                units::second(1.0))
                                                            )));
    static ref RISE_BASE_E10: logic::State = logic::State::Edge(logic::Edge::Rise(
                                                            Some(logic::ChangePattern::new(
                                                                units::second(1.0 + 2e-10),
                                                                units::second(1.0 + 1e-10))
                                                            )));
    static ref RISE_BASE_E11: logic::State = logic::State::Edge(logic::Edge::Rise(
                                                            Some(logic::ChangePattern::new(
                                                                units::second(1.0 + 1e-11),
                                                                units::second(1.0 + 1e-11))
                                                            )));
}

#[test]
fn sort_hash_table() {
  println!("{}", and_ba().table().sort());
  println!("{}", and_ab().table().sort());
  println!("{}", and_ab().table());
  println!("{}", and_ba().table());
  assert_eq!(get_hash(and_ba().table()), get_hash(and_ba().table()));
  assert_eq!(get_hash(and_ab().table()), get_hash(and_ba().table()));
  assert_ne!(get_hash(and_ab().table()), get_hash(and_ac().table()));
  assert_eq!(get_hash(and_ba()), get_hash(and_ba()));
  assert_eq!(get_hash(and_ab()), get_hash(and_ba()));
  assert_ne!(get_hash(and_ab()), get_hash(and_ac()));
}

#[test]
fn change_pattern() {}
#[test]
fn logic_state() {
  assert_eq!(get_hash(*HIGH), get_hash(*HIGH));
  assert_eq!(get_hash(*RISE_NONE), get_hash(*RISE_NONE));
  assert_eq!(get_hash(*RISE_BASE), get_hash(*RISE_BASE));
  assert_ne!(get_hash(*HIGH), get_hash(*LOW));
  println!("{}", *RISE_BASE);
  println!("{}", *RISE_BASE_E10);
  println!("{}", *RISE_BASE_E11);
  // 1+1e-10 != 1
  assert_ne!(*RISE_BASE, *RISE_BASE_E10);
  assert_ne!(get_hash(*RISE_BASE), get_hash(*RISE_BASE_E10));
  // 1+1e-11 == 1
  assert_eq!(*RISE_BASE, *RISE_BASE_E11);
  // assert_eq!(get_hash(*RISE_BASE),get_hash(*RISE_BASE_E11));
}
#[test]
fn logic_vecter_as_key() {
  use HashMap;
  let mut pin_map = HashMap::new();
  let v = 12345.000;
  {
    let mut v_k1: logic::Vector = vec![].into();
    v_k1.push(logic::State::H);
    v_k1.push(logic::State::H);
    v_k1.push(logic::State::Edge(logic::Edge::Rise(Some(logic::ChangePattern::new(
      units::second(345670.7734567893456789456787777771),
      units::second(0.1),
    )))));
    v_k1.push(logic::State::F);
    let _ = pin_map.insert(v_k1, v);
  }
  {
    let mut v_k2: logic::Vector = vec![].into();
    v_k2.push(logic::State::H);
    v_k2.push(logic::State::H);
    v_k2.push(logic::State::Edge(logic::Edge::Rise(Some(logic::ChangePattern::new(
      units::second(345670.7734567893456789456787777771),
      units::second(0.1),
    )))));
    v_k2.push(logic::State::F);
    assert_eq!(Some(&v), pin_map.get(&v_k2));
  }
}

// #[test]
// fn logic_operation() {
//     use std::str::FromStr;
//     assert_eq!(logic::Operator2::from_str("&"), Ok(logic::Operator2::And));
//     assert_eq!(logic::Operator2::from_str("*"), Ok(logic::Operator2::And));
//     assert_eq!(logic::Operator2::from_str("|"), Ok(logic::Operator2::Or));
//     assert_eq!(logic::Operator2::from_str("+"), Ok(logic::Operator2::Or));
//     assert_eq!(logic::Operator2::from_str("^"), Ok(logic::Operator2::Xor));
//     let and = logic::Operator2::And;
//     let or = logic::Operator2::Or;
//     let xor = logic::Operator2::Xor;
//     assert_eq!(format!("{}",and), "&");
//     assert_eq!(format!("{}",or),  "|");
//     assert_eq!(format!("{}",xor), "^");
// }

#[test]
fn port_as_key() {
  let port_a1 = Port::new("A");
  let port_a2 = Port::new("A");
  println!("{:?}", port_a1.table());
  let mut map = HashMap::new();
  let v = 1;
  let _ = map.insert(port_a1, v);
  assert_eq!(Some(&v), map.get(&port_a2));
}
#[test]
fn port_compute() {
  let port_a1 = Port::new("A");
  let port_a2 = Port::new("A");
  let and = logic::Operator2::And;
  for (vec_in, state_out) in
    and.compute_table(&port_a1.table(), &port_a2.table()).table.iter()
  {
    println!("{:?} {:?}", vec_in, state_out);
  }
}
#[test]
fn expression_eq() {
  assert_eq!(and_ab(), and_ab());
  assert_eq!(and_ab(), and_ba());
  assert_ne!(and_ab(), nand_ab());
}
#[test]
fn static_expression() {
  println!("{}", _0().table());
  println!("{}", _1().table());
  println!("{}", and_aa().table());
}
#[test]
fn expression_nand_table() {
  // env_logger::init() ;
  println!("{}", Into::<BooleanExpression>::into(Port::new("A")).table());
  let exp_not_a_and_b = nand_ab();
  assert_eq!(format!("{}", exp_not_a_and_b), "!(A*B)");
  println!("**** Origin  ****************");
  let table = exp_not_a_and_b.table();
  println!("{table}");

  println!("**** Search1 ****************");
  println!(
    "{}",
    logic::Searcher::new(
      vec![
        (Port::new("A"), vec![logic::State::H],),
        (Port::new("C"), vec![logic::State::H],),
      ],
      Some(vec![logic::State::F]),
      vec![],
      None,
    )
    .search(&table)
  );

  println!("**** Search2 ****************");
  println!(
    "{}",
    logic::Searcher::new(
      vec![
        (Port::new("A"), vec![logic::State::H, logic::State::L,]),
        (Port::new("B"), vec![logic::State::F],)
      ],
      None,
      vec![],
      Some(vec![]),
    )
    .search(&table)
  );

  println!("**** Search3 ****************");
  println!(
    "{}",
    logic::Searcher::new(
      vec![(Port::new("A"), vec![logic::State::H])],
      None,
      vec![],
      Some(vec![logic::State::X]),
    )
    .search(&table)
  );

  println!("**** Search4 ****************");
  println!(
    "{}",
    logic::Searcher::new(
      vec![
        (Port::new("A"), vec![logic::State::L]),
        (Port::new("A"), vec![logic::State::H]),
      ],
      None,
      vec![
        (Port::new("B"), vec![logic::State::H]),
        (Port::new("B"), vec![logic::State::F]),
      ],
      None,
    )
    .search(&table)
  );

  println!("**** Search5 ****************");
  println!(
    "{}",
    logic::Searcher::new(
      vec![(Port::new("C"), vec![logic::State::H])],
      None,
      vec![],
      Some(vec![logic::State::H]),
    )
    .search(&table)
  );
}
// #[test]
// fn expression_length_check() {
//     use core::fmt::{self, write};
//     let right: BooleanExpression = FunctionExpression::new (
//         vec![Port::new("A").into(),Port::new("B").into()],
//         vec![logic::Operator2::And],
//     ).into();
//     assert_eq!(format!("{}",right), "(A&B)");
//     let mut output = String::new();
//     {
//         let wrong_on_sub_expression_vec:BooleanExpression = FunctionExpression::new (
//             vec![],
//             vec![logic::Operator2::And],
//         ).into();
//         if let Err(fmt::Error) = write(&mut output,
//                 format_args!("{}", wrong_on_sub_expression_vec)) {
//         }else{
//             panic!("\nIt should be error!\n {:?}\n", wrong_on_sub_expression_vec);
//         }
//     }
//     {
//         let wrong_on_operation_vec: BooleanExpression = FunctionExpression::new (
//             vec![Port::new("A").into(),Port::new("B").into()],
//             vec![],
//         ).into();
//         if let Err(fmt::Error) = write(&mut output,
//                 format_args!("{}", wrong_on_operation_vec)) {
//         }else{
//             panic!("\nIt should be error!\n {:?}\n", wrong_on_operation_vec);
//         }
//     }
// }
