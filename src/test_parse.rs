use crate::parse;
use crate::parse::Exp::*;

macro_rules! make_testcase_add {
    ($input:expr, $output:expr, $testname:ident) => {
        #[test]
        fn $testname() -> Result<(), parse::ParseError> {
            assert_eq!(parse::Exp::try_from($input)?, $output);
            Ok(())
        }
    };
}

make_testcase_add!("5", Int(5), test_int);
