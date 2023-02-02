mod for_boolean_expression{
    use std::fmt::{Formatter, Display};
    use hashbrown::HashMap;

    use crate::expression::{*, boolean_expression::LogicVector};

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

    #[test]
    fn expression_nand() {
        let port_a = Port::new("A");
        let port_b = Port::new("B");
        let exp_a_and_b = BooleanExpression::new (
            vec![Box::new(port_a),Box::new(port_b)], 
            vec![true,true],
            vec![LogicOperation::And],
        );
        let exp_not_a_and_b = BooleanExpression::new (
            vec![Box::new(exp_a_and_b)], 
            vec![false],
            vec![],
        );
        assert_eq!(format!("{}",exp_not_a_and_b), "!(A&B)");
    }
    #[test]
    fn logic() {
        let mut v = LogicVector::new();
        println!("{v}");
        v.state_vec.push(LogicState::High);
        v.state_vec.push(LogicState::High);
        v.state_vec.push(LogicState::Rise(ChangePattern::new(345670.7734567893456789456787777771,0.1)));
        v.state_vec.push(LogicState::Fall(None));
        println!("{v}")
         // let mut pin_map= HashMap::new();
        // pin_map.insert(LogicVector{
        //     state_vec: vec![LogicState::High],
        // }, 1);
        // for (viking, health) in &pin_map {
        //     println!("{:?} has {} hp", viking, health);
        // }
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