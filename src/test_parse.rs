use crate::parse;
use crate::parse::Exp::*;
use std::boxed::Box;

macro_rules! parse_testcase {
    ($input:expr, $output:expr, $testname:ident) => {
        #[test]
        fn $testname() -> Result<(), parse::ParseError> {
            assert_eq!(parse::Exp::try_from($input)?, $output);
            Ok(())
        }
    };
}

parse_testcase!("5", Int(5), test_int);
parse_testcase!("5.4", Float(5.4), test_float);
parse_testcase!(
    "(+ 4 7)",
    Add {
        lhs: Box::new(Int(4)),
        rhs: Box::new(Int(7)),
    },
    test_add
);

parse_testcase!(
    "(* 6 7)",
    Mult {
        lhs: Box::new(Int(6)),
        rhs: Box::new(Int(7)),
    },
    test_multiply
);
parse_testcase!("Word", Id("Word".to_string()), test_id);

parse_testcase!(
    "(= 5 5)",
    Eq {
        lhs: Box::new(Int(5)),
        rhs: Box::new(Int(5)),
    },
    test_eq
);

parse_testcase!(
    "(if (= 5 5) 1 0)",
    If {
        cond: Box::new(Eq {
            lhs: Box::new(Int(5)),
            rhs: Box::new(Int(5)),
        }),
        lhs: Box::new(Int(1)),
        rhs: Box::new(Int(0)),
    },
    test_if
);


#[test]
fn debug_id_test() {
    let word = String::from("Word");
    let exp = Id(word);
    assert_eq!(format!("{:?}", exp), "Id(Word)");
}

/// Checks that a Num expression gets correctly formatted
#[test]
fn debug_num_test() {
    // Instantiates a number expression containing a 5
    let exp = Int(5);
    // format! in this case will convert an object into its debug representation as defined in
    // the fmt function.
    // The assert will panic if the Exp object does not format correctly
    assert_eq!(format!("{:?}", exp), "Int(5)");
}
#[test]
fn debug_plus_test() {
    let lhs = Int(5);
    let rhs = Int(8);
    let plus_exp = Add {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    assert_eq!(format!("{:?}", plus_exp), "Add(Int(5), Int(8))");
}

#[test]
fn debug_mult_test() {
    let lhs = Int(5);
    let rhs = Int(8);
    let mult_exp = Mult {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    assert_eq!(format!("{:?}", mult_exp), "Mult(Int(5), Int(8))");
}
#[test]
fn debug_lambda_test() {
    let symbol = String::from("Word");
    let body = Int(8);
    let lambda_exp = Lambda {
        arg: symbol,
        body: Box::new(body),
    };

    assert_eq!(format!("{:?}", lambda_exp), "Lambda(Word, Int(8))");
}
#[test]
fn debug_if_test() {
    let cond = Bool(true);
    let lhs = Int(5);
    let rhs = Int(8);
    let if_exp = If {
        cond: Box::new(cond),
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    assert_eq!(format!("{:?}", if_exp), "If(Bool(true), Int(5), Int(8))");
}
#[test]
fn debug_eq_test() {
    let lhs = Int(5);
    let rhs = Int(8);
    let eq_exp = Eq {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    assert_eq!(format!("{:?}", eq_exp), "Eq(Int(5), Int(8))");
}

#[test]
fn debug_begin_test() {
    let begin_exp = Begin(vec![Int(5), Int(8)]);

    assert_eq!(format!("{:?}", begin_exp), "Begin(Int(5), Int(8))");
}

#[test]
fn debug_ref_test() {
    let ref_exp = Ref(Box::new(Int(5)));

    assert_eq!(format!("{:?}", ref_exp), "Ref(Int(5))");
}

#[test]
fn debug_mut_ref_test() {
    let mut_ref_exp = MutRef(Box::new(Int(5)));

    assert_eq!(format!("{:?}", mut_ref_exp), "MutRef(Int(5))");
}

#[test]
fn debug_box_test() {
    let box_exp = Box(Box::new(Int(5)));

    assert_eq!(format!("{:?}", box_exp), "Box(Int(5))");
}

#[test]
fn debug_unbox_test() {
    let unbox_exp = Unbox(Box::new(Int(5)));

    assert_eq!(format!("{:?}", unbox_exp), "Unbox(Int(5))");
}

#[test]
fn debug_deref_test() {
    let deref_exp = Deref(Box::new(Ref(Box::new(Int(5)))));

    assert_eq!(format!("{:?}", deref_exp), "Deref(Ref(Int(5)))");
}

#[test]
fn debug_set_test() {
    let lhs = MutRef(Box::new(Int(5)));
    let rhs = Int(10);
    let set_exp = Set {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };

    assert_eq!(format!("{:?}", set_exp), "Set(MutRef(Int(5)), Int(10))");
}

#[test]
fn debug_display_test() {
    let display_exp = Display(Box::new(Int(5)));

    assert_eq!(format!("{:?}", display_exp), "Display(Int(5))");
}

#[test]
fn debug_debug_test() {
    let debug_exp = Debug(Box::new(Int(5)));

    assert_eq!(format!("{:?}", debug_exp), "Debug(Int(5))");
}

#[test]
fn debug_app_test() {
    let func = Id("func".to_string());
    let arg = Int(5);
    let app_exp = App {
        func: Box::new(func),
        arg: Box::new(arg),
    };

    assert_eq!(format!("{:?}", app_exp), "App(Id(func), Int(5))");
}
