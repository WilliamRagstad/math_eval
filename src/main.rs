use std::{
    io::{stdout, Write},
    num::ParseFloatError,
    str::Chars,
};

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
                let line = remove_whitespace(&mut buff);
                if line.is_empty() {
                    continue;
                }
                println!("{}", eval(line));
            }
            Err(e) => println!("{}", e),
        }
    }
}

fn remove_whitespace(s: &mut String) -> &str {
    s.retain(|c| !c.is_whitespace());
    s.as_str()
}

fn eval(line: &str) -> f64 {
    match eval_expr(&mut line.chars(), 0) {
        Ok((value, next)) => {
            assert_eq!(next, '\0', "Unexpected trailing character '{}'", next);
            value
        }
        Err(e) => {
            println!("{}", e);
            0.0
        }
    }
}

fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' => 2,
        '/' => 3,
        '^' => 4,
        _ => 0,
    }
}

fn eval_expr(iter: &mut Chars, bp: u8) -> Result<(f64, char), ParseFloatError> {
    let (lhs, op) = eval_primary(iter)?;
    if op == '\0' || op == ')' {
        return Ok((lhs, op));
    }
    let op_bp = precedence(op);
    println!("{} has bp {}", op, op_bp);
    let (rhs, nc) = if bp > op_bp {
        // Evaluate this expression and then evaluate the next expression
        eval_primary(iter)
    } else {
        // Evaluate the next expression before this
        eval_expr(iter, op_bp)
    }?;
    println!("Evaluating: {} {} {}", lhs, op, rhs);
    return Ok((
        match op {
            '+' => lhs + rhs,
            '-' => lhs - rhs,
            '*' => lhs * rhs,
            '/' => lhs / rhs,
            '^' => lhs.powf(rhs),
            _ => panic!("Unknown operator: {}", op),
        },
        nc,
    ));
}

fn eval_primary(iter: &mut Chars) -> Result<(f64, char), ParseFloatError> {
    let c = iter.next().unwrap();
    if c.is_numeric() {
        return eval_num(iter, c);
    } else if c == '(' {
        let (result, nc) = eval_expr(iter, 0)?;
        assert_eq!(nc, ')', "Missing closing parenthesis");

        return Ok((result, iter.next().unwrap_or('\0')));
    } else {
        panic!("Unexpected character: {}", c);
    }
}

fn eval_num(iter: &mut Chars, c: char) -> Result<(f64, char), ParseFloatError> {
    // Parse a single number
    let mut num = String::new();
    assert!(c.is_numeric());
    num.push(c);
    let mut has_digit = false;
    while let Some(c) = iter.next() {
        if c.is_numeric() {
            num += &c.to_string();
        } else if !has_digit && c == '.' {
            has_digit = true;
            num += &c.to_string();
        } else {
            // println!("Found: {} (Next: {})", num, c);
            return Ok((num.parse::<f64>().unwrap(), c));
        }
    }
    // println!("Found: {} (Next: EOL)", num);
    return Ok((num.parse::<f64>().unwrap(), '\0'));
}
