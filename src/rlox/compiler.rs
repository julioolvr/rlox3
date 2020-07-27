use crate::rlox::scanner::Scanner;

pub fn compile(code: &String) {
    let mut scanner = Scanner::new();
    let mut line = 0;

    for token in scanner.scan(code) {
        if token.line != line {
            print!("{: >4} ", token.line);
            line = token.line;
        } else {
            print!("   | ")
        }

        println!("{:?} {}", token.token_type, token.code);
    }
}
