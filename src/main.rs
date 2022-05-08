use std::io::{stdout, Write};

// Version
const VERSION: &str = "0.1.0";

fn main() {
    println!("Math evaluator v{}", VERSION);
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut buff = String::new();
        match std::io::stdin().read_line(&mut buff) {
            Ok(read) => {
                if read == 0 {
                    break;
                }
                let line = buff.trim();
                if line.is_empty() {
                    continue;
                }
                match line.parse::<f64>() {
                    Ok(n) => println!("{}",n),
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
