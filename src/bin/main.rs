use std::io::{self, Write};
use veron::Veron;

fn main() {
    println!("Veron v0.0.0 - JavaScript Runtime");
    println!("Type 'exit' to quit");

    let mut runtime = Veron::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        match runtime.execute(&input) {
            Ok(result) => println!("{}", result),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
