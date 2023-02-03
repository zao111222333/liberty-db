// #[cfg(not(test))]
// use log::{info, warn};
 
// #[cfg(test)]
// use std::{println as info, println as warn};
mod for_boolean_expression{
    use std::fmt::{Formatter, Display};
    use hashbrown::HashMap;
    use log::{warn, info};

    use crate::expression::{*, boolean_expression::{LogicVector, BooleanExpressionLike}};

    #[test]
    fn logic_operation() {
        use std::str::FromStr;
        assert_eq!(LogicOperation::from_str("&"), Ok(LogicOperation::And));
        assert_eq!(LogicOperation::from_str("*"), Ok(LogicOperation::And));
        assert_eq!(LogicOperation::from_str("|"), Ok(LogicOperation::Or));
        assert_eq!(LogicOperation::from_str("+"), Ok(LogicOperation::Or));
        assert_eq!(LogicOperation::from_str("^"), Ok(LogicOperation::Xor));
        let and = LogicOperation::And;
        let or = LogicOperation::Or;
        let xor = LogicOperation::Xor;
        assert_eq!(format!("{}",and), "&");
        assert_eq!(format!("{}",or),  "|");
        assert_eq!(format!("{}",xor), "^");
    }

    #[test]
    fn logic_state() {
        use std::str::FromStr;
        let h = LogicState::High;
        match h {
            LogicState::Unknown => (),
            LogicState::High => (),
            LogicState::Low => (),
            LogicState::Fall(_) => (),
            LogicState::Rise(_) => (),
            LogicState::HighImpedance => todo!(),
        }
    }
    #[test_log::test]
fn it_works() {
  info!("Checking whether it still works...");
  let a = 4;
  let b = 2 + 2== a;
  assert!(b);
  info!("Looks good!");
}
    // #[test_env_log::test]
    #[test]
    fn expression_nand() {
        warn!("[warn] ");
        let test_once = |v: [LogicState;2]|->LogicState{
            let mut port_a = Port::new("A");
            let mut port_b = Port::new("B");
            port_a.set_state(v[0]);
            port_b.set_state(v[1]);
            let exp_a_and_b = BooleanExpression::new (
                vec![Box::new(port_a),Box::new(port_b)], 
                vec![false,false],
                vec![LogicOperation::And],
            );
            let mut exp_not_a_and_b = BooleanExpression::new (
                vec![Box::new(exp_a_and_b)], 
                vec![true],
                vec![],
            );
            assert_eq!(format!("{}",exp_not_a_and_b), "!(A&B)");
            match exp_not_a_and_b.get_state(){
                Ok(s) => return s,
                Err(err) => panic!("{err}"),
            }
        };
        let test_v = vec![
            ([LogicState::High,       LogicState::High],LogicState::Low ),
            ([LogicState::Low,        LogicState::High],LogicState::High),
            ([LogicState::High,       LogicState::Low ],LogicState::High),
            ([LogicState::Low,        LogicState::Low ],LogicState::High),
            ([LogicState::Rise(None), LogicState::High],LogicState::Fall(None)),
            ([LogicState::Rise(ChangePattern::new(1.0, 2.0)),LogicState::High],LogicState::Fall(None)),
            ([LogicState::Fall(None), LogicState::High],LogicState::Rise(None)),
        ];
        for (test_in,want) in test_v.iter(){
            let got = test_once(*test_in);
            assert_eq!(*want,got);
        }
        
    }
    #[test]
    fn logic_vecter_as_key() {
        let mut pin_map= HashMap::new();
        let v = 12345.000;
        {
            let mut v_k1 = LogicVector::new();
            v_k1.state_vec.push(LogicState::High);
            v_k1.state_vec.push(LogicState::High);
            v_k1.state_vec.push(LogicState::Rise(ChangePattern::new(345670.7734567893456789456787777771,0.1)));
            v_k1.state_vec.push(LogicState::Fall(None));
            println!("{v_k1}");
            let _ = pin_map.insert(v_k1, v);
        }
        {
            let mut v_k2 = LogicVector::new();
            v_k2.state_vec.push(LogicState::High);
            v_k2.state_vec.push(LogicState::High);
            v_k2.state_vec.push(LogicState::Rise(ChangePattern::new(345670.7734567893456789456787777771,0.1)));
            v_k2.state_vec.push(LogicState::Fall(None));
            println!("{v_k2}");
            assert_eq!(Some(&v),pin_map.get(&v_k2));
        }
    }
    #[test]
    fn expression_length_check() {
        use std::fmt::{self, write};
        let right = BooleanExpression::new (
            vec![Box::new(Port::new("A")),Box::new(Port::new("B"))], 
            vec![true,true],
            vec![LogicOperation::And],
        );
        assert_eq!(format!("{}",right), "A&B");

        let mut output = String::new();
        {
            let wrong_on_not_invert_vec = BooleanExpression::new (
                vec![Box::new(Port::new("A")),Box::new(Port::new("B"))], 
                vec![true,true,true],
                vec![LogicOperation::And],
            );
            if let Err(fmt::Error) = write(&mut output, 
                format_args!("{}", wrong_on_not_invert_vec)) {
            }else{
                panic!("\nIt should be error!\n {:?}\n", wrong_on_not_invert_vec);
            }
        }
        {
            let wrong_on_sub_expression_vec = BooleanExpression::new (
                vec![], 
                vec![true,true],
                vec![LogicOperation::And],
            );
            if let Err(fmt::Error) = write(&mut output, 
                    format_args!("{}", wrong_on_sub_expression_vec)) {
            }else{
                panic!("\nIt should be error!\n {:?}\n", wrong_on_sub_expression_vec);
            }
        }
        {
            let wrong_on_operation_vec = BooleanExpression::new (
                vec![Box::new(Port::new("A")),Box::new(Port::new("B"))], 
                vec![true,true],
                vec![],
            );
            if let Err(fmt::Error) = write(&mut output, 
                    format_args!("{}", wrong_on_operation_vec)) {
            }else{
                panic!("\nIt should be error!\n {:?}\n", wrong_on_operation_vec);
            }
        }
        println!("Pass!")
    }
}