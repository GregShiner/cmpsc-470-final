pub mod parse;
#[cfg(test)]
pub mod test_parse;

fn main() -> Result<(), parse::ParseError> {
    let inputs: Vec<&str> = vec!["5", "4.5", "(+ 4 5)"];
    for input in inputs {
        println!("{:?}\n{:?}\n", input, parse::Exp::try_from(input)?);
    }
    Ok(())
}
