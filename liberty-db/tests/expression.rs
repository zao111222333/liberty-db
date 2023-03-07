#[cfg(test)]

use std::hash::Hash;
use std::hash::Hasher;

use liberty_db::{expression::*, units};
use test_log::test;
use log::info;

fn nand_ab()->BooleanExpression{
    let port_a: BooleanExpression = Port::new("A").into();
    let port_b = Port::new("B").into();
    let exp_not_a_and_b: BooleanExpression = FunctionExpression::new(
        vec![FunctionExpression::new (
            vec![port_a,port_b],
            vec![None,None],
            vec![LogicOperator2::And],
        ).into()],
        vec![Some(LogicOperator1::Not)],
        vec![],
    ).into();
    exp_not_a_and_b
}
fn _0 ()->BooleanExpression{
    FunctionExpression::new(
        vec![Port::new("A").into()],
        vec![Some(LogicOperator1::Logic0)],
        vec![],
    ).into()
}
fn _1 ()->BooleanExpression{
    FunctionExpression::new(
        vec![Port::new("A").into()],
        vec![Some(LogicOperator1::Logic1)],
        vec![],
    ).into()
}
fn and_ab()->BooleanExpression{
    let port_a: BooleanExpression = Port::new("A").into();
    let port_b = Port::new("B").into();
    let exp_not_a_and_b: BooleanExpression = FunctionExpression::new(
        vec![FunctionExpression::new (
            vec![port_a,port_b],
            vec![None,None],
            vec![LogicOperator2::And],
        ).into()],
        vec![None],
        vec![],
    ).into();
    exp_not_a_and_b
}
fn and_ac()->BooleanExpression{
    let port_a: BooleanExpression = Port::new("A").into();
    let port_c = Port::new("C").into();
    let exp_not_a_and_b: BooleanExpression = FunctionExpression::new(
        vec![FunctionExpression::new (
            vec![port_a,port_c],
            vec![None,None],
            vec![LogicOperator2::And],
        ).into()],
        vec![None],
        vec![],
    ).into();
    exp_not_a_and_b
}
fn and_ba()->BooleanExpression{
    let port_a: BooleanExpression = Port::new("A").into();
    let port_b = Port::new("B").into();
    let exp_not_a_and_b: BooleanExpression = FunctionExpression::new(
        vec![FunctionExpression::new (
            vec![port_b,port_a],
            vec![None,None],
            vec![LogicOperator2::And],
        ).into()],
        vec![None],
        vec![],
    ).into();
    exp_not_a_and_b
}
fn and_aa()->BooleanExpression{
    let port_a: BooleanExpression = Port::new("A").into();
    let port_b = Port::new("A").into();
    let exp_not_a_and_b: BooleanExpression = FunctionExpression::new(
        vec![FunctionExpression::new (
            vec![port_a,port_b],
            vec![None,None],
            vec![LogicOperator2::And],
        ).into()],
        vec![None],
        vec![],
    ).into();
    exp_not_a_and_b
}


fn get_hash<T: Hash>(s: T)->u64{
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
lazy_static::lazy_static!{
    static ref HIGH:      LogicState = LogicState::H;
    static ref LOW:       LogicState = LogicState::L;
    static ref RISE_NONE: LogicState = LogicState::R;
    static ref RISE_BASE: LogicState = LogicState::Edge(EdgeState::Rise(
                                                            Some(ChangePattern::new(
                                                                units::second(1.0),
                                                                units::second(1.0))
                                                            )));
    static ref RISE_BASE_E10: LogicState = LogicState::Edge(EdgeState::Rise(
                                                            Some(ChangePattern::new(
                                                                units::second(1.0 + 2e-10),
                                                                units::second(1.0 + 1e-10))
                                                            )));
    static ref RISE_BASE_E11: LogicState = LogicState::Edge(EdgeState::Rise(
                                                            Some(ChangePattern::new(
                                                                units::second(1.0 + 1e-11),
                                                                units::second(1.0 + 1e-11))
                                                            )));
}

#[test]
fn sort_hash_table() {
    println!("{}",and_ba().to_table().sort());
    println!("{}",and_ab().to_table().sort());
    println!("{}",and_ab().to_table());
    println!("{}",and_ba().to_table());
    assert_eq!(get_hash(and_ba().to_table()),get_hash(and_ba().to_table()));
    assert_eq!(get_hash(and_ab().to_table()),get_hash(and_ba().to_table()));
    assert_ne!(get_hash(and_ab().to_table()),get_hash(and_ac().to_table()));
    assert_eq!(get_hash(and_ba()),get_hash(and_ba()));
    assert_eq!(get_hash(and_ab()),get_hash(and_ba()));
    assert_ne!(get_hash(and_ab()),get_hash(and_ac()));
}

#[test]
fn change_pattern() {
}
#[test]
fn logic_state() {
    assert_eq!(get_hash(*HIGH),get_hash(*HIGH));
    assert_eq!(get_hash(*RISE_NONE),get_hash(*RISE_NONE));
    assert_eq!(get_hash(*RISE_BASE),get_hash(*RISE_BASE));
    assert_ne!(get_hash(*HIGH),get_hash(*LOW));
    println!("{}", *RISE_BASE);
    println!("{}", *RISE_BASE_E10);
    println!("{}", *RISE_BASE_E11);
    // 1+1e-10 != 1
    assert_ne!(*RISE_BASE, *RISE_BASE_E10);
    assert_ne!(get_hash(*RISE_BASE),get_hash(*RISE_BASE_E10));
    // 1+1e-11 == 1
    assert_eq!(*RISE_BASE, *RISE_BASE_E11);
    assert_eq!(get_hash(*RISE_BASE),get_hash(*RISE_BASE_E11));
}
#[test]
fn logic_vecter_as_key() {
    use hashbrown::HashMap;
    let mut pin_map= HashMap::new();
    let v = 12345.000;
    {
        let mut v_k1:LogicVector = vec![].into();
        v_k1.push(LogicState::H);
        v_k1.push(LogicState::H);
        v_k1.push(LogicState::Edge(EdgeState::Rise(
            Some(ChangePattern::new(
                units::second(345670.7734567893456789456787777771),
                units::second(0.1))
            ))
        ));
        v_k1.push(LogicState::F);
        let _ = pin_map.insert(v_k1, v);
    }
    {
        let mut v_k2:LogicVector = vec![].into();
        v_k2.push(LogicState::H);
        v_k2.push(LogicState::H);
        v_k2.push(LogicState::Edge(EdgeState::Rise(
            Some(ChangePattern::new(
                units::second(345670.7734567893456789456787777771),
                units::second(0.1))
            ))
        ));
        v_k2.push(LogicState::F);
        assert_eq!(Some(&v),pin_map.get(&v_k2));
    }
}



// #[test]
// fn logic_operation() {
//     use std::str::FromStr;
//     assert_eq!(LogicOperator2::from_str("&"), Ok(LogicOperator2::And));
//     assert_eq!(LogicOperator2::from_str("*"), Ok(LogicOperator2::And));
//     assert_eq!(LogicOperator2::from_str("|"), Ok(LogicOperator2::Or));
//     assert_eq!(LogicOperator2::from_str("+"), Ok(LogicOperator2::Or));
//     assert_eq!(LogicOperator2::from_str("^"), Ok(LogicOperator2::Xor));
//     let and = LogicOperator2::And;
//     let or = LogicOperator2::Or;
//     let xor = LogicOperator2::Xor;
//     assert_eq!(format!("{}",and), "&");
//     assert_eq!(format!("{}",or),  "|");
//     assert_eq!(format!("{}",xor), "^");
// }

#[test]
fn port_as_key() {
    let port_a1 = Port::new("A");
    let port_a2 = Port::new("A");
    println!("{:?}",port_a1.to_table());
    let mut map = hashbrown::HashMap::new();
    let v = 1;
    let _ = map.insert(port_a1, v);
    assert_eq!(
        Some(&v),
        map.get(&port_a2)
    );
}
#[test]
fn port_compute() {
    let port_a1 = Port::new("A");
    let port_a2 = Port::new("A");
    let and = LogicOperator2::And;
    for (vec_in,state_out) in and.compute_table(
        &port_a1.to_table(), 
        &port_a2.to_table(),
    ).table.iter(){
        println!("{:?} {:?}", vec_in,state_out);
    }
}
#[test]
fn expression_eq() {
    assert_eq!(and_ab(),and_ab());
    assert_eq!(and_ab(),and_ba());
    assert_ne!(and_ab(),nand_ab());
}
#[test]
fn static_expression() {
    println!("{}",_0().to_table());
    println!("{}",_1().to_table());
    println!("{}",and_aa().to_table());
}
#[test]
fn expression_nand_table() {
    env_logger::init() ;
    println!("{}",Into::<BooleanExpression>::into(Port::new("A")).to_table());
    let exp_not_a_and_b=nand_ab();
    assert_eq!(format!("{}",exp_not_a_and_b), "!(A*B)");
    println!("**** Origin  ****************");
    let table = exp_not_a_and_b.to_table();
    println!("{table}");

    println!("**** Search1 ****************");
    println!("{}", Searcher::new(
        vec![
            (Port::new("A"),vec![ LogicState::H ],),
            (Port::new("C"),vec![ LogicState::H ],),
        ],
        Some(vec![LogicState::F ]),
        vec![],
        None,
    ).search(&table));

    println!("**** Search2 ****************");
    println!("{}", Searcher::new(
        vec![
            (Port::new("A"),
            vec![ 
                LogicState::H,
                LogicState::L,
            ]), 
            (Port::new("B"),
            vec![ LogicState::F ],
            )
            ], 
        None,
        vec![],
        Some(vec![]), 
    ).search(&table));

    println!("**** Search3 ****************");
    println!("{}", Searcher::new(
        vec![(Port::new("A"),
        vec![ LogicState::H ])], 
        None,
        vec![],
        Some(vec![ LogicState::X ]),
    ).search(&table));

    println!("**** Search4 ****************");
    println!("{}", Searcher::new(
        vec![
            (Port::new("A"),vec![LogicState::L]),
            (Port::new("A"),vec![LogicState::H]),
            ], 
        None,
        vec![
            (Port::new("B"),vec![ LogicState::H]),
            (Port::new("B"),vec![ LogicState::F]),
            ],
        None,
    ).search(&table));

    println!("**** Search5 ****************");
    println!("{}", Searcher::new(
        vec![(Port::new("C"),
        vec![ LogicState::H ])], 
        None,
        vec![],
        Some(vec![ LogicState::H ]),
    ).search(&table));
}
// #[test]
// fn expression_length_check() {
//     use std::fmt::{self, write};
//     let right: BooleanExpression = FunctionExpression::new (
//         vec![Port::new("A").into(),Port::new("B").into()],
//         vec![LogicOperator2::And],
//     ).into();
//     assert_eq!(format!("{}",right), "(A&B)");
//     let mut output = String::new();
//     {
//         let wrong_on_sub_expression_vec:BooleanExpression = FunctionExpression::new (
//             vec![], 
//             vec![LogicOperator2::And],
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
