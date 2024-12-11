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
    Plus {
        lhs: Box::new(Int(4)),
        rhs: Box::new(Int(7)),
    },
    test_add
);
