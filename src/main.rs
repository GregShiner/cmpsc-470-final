use std::{
    collections::HashMap,
    env,
    fs::read_to_string,
    io::{self, Write},
};

pub mod interp;
pub mod parse;
#[cfg(test)]
pub mod test_parse;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        let input = read_to_string(filename)?;
        match parse::Exp::try_from(&input[..]) {
            Ok(exp) => match interp::interp(exp, &mut HashMap::new()) {
                Ok(value) => println!("{:?}", value),
                Err(e) => println!("Evaluation error: {}", e),
            },
            Err(e) => println!("Parse error: {}", e),
        }
        return Ok(());
    }
    println!("Welcome to the expression evaluator REPL!");
    println!("Enter expressions to evaluate them, or 'exit' to quit.");
    println!("Example expressions:");
    println!("  5");
    println!("  (+ 4 5)");
    println!("  (* 3 (+ 2 4))");
    println!("  (if (> 5 3) 1 2)");
    println!();

    loop {
        // Print prompt and flush to ensure it appears
        print!("> ");
        io::stdout().flush()?;

        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        // Check for exit command
        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // Parse and evaluate the input
        match parse::Exp::try_from(input) {
            Ok(exp) => match interp::interp(exp, &mut HashMap::new()) {
                Ok(value) => println!("{:?}", value),
                Err(e) => println!("Evaluation error: {}", e),
            },
            Err(e) => println!("Parse error: {}", e),
        }
    }

    Ok(())
}
