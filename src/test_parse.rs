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
parse_testcase!(
    "Word",
    Id("Word".to_string()),
    test_id
);

parse_testcase!(
    "(= 5 5)",
    Eq{
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