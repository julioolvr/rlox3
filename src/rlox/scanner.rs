use crate::rlox::token::{Token, TokenType};

pub struct Scanner {}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn scan<'a>(&mut self, code: &'a String) -> ScannerIterator<'a> {
        ScannerIterator { code, start: 0, current: 0, line: 1 }
    }
}

pub struct ScannerIterator<'code> {
    code: &'code String,
    start: usize,
    current: usize,
    line: usize,
}

impl<'code> ScannerIterator<'code> {
    fn advance(&mut self) -> Option<&'code str> {
        let current = self.current;
        self.current += 1;

        if !self.is_at_end() {
            Some(&self.code[current..current + 1])
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&'code str> {
        if !self.is_at_end() {
            Some(&self.code[self.current..self.current + 1])
        } else {
            None
        }
    }

    fn peek_next(&self) -> Option<&'code str> {
        if self.current + 1 < self.code.len() {
            Some(&self.code[self.current + 1..self.current + 2])
        } else {
            None
        }
    }

    fn match_char(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek().map_or(true, |c| c != expected) {
            return false;
        }

        self.advance();
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }

    fn build_token(&self, code: &'code str, token_type: TokenType) -> Token<'code> {
        Token::new(code, self.line, token_type)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(" ") | Some("\r") | Some("\t") => {
                    self.advance();
                }
                Some("\n") => {
                    self.line += 1;
                    self.advance();
                }
                Some("/") => {
                    if let Some("/") = self.peek_next() {
                        // Started a comment
                        let mut next = self.peek();

                        while !self.is_at_end() && next != Some("\n") {
                            self.advance();
                            next = self.peek();
                        }
                    } else {
                        break
                    }
                }
                _ => break
            }
        }
    }
}

impl<'code> Iterator for ScannerIterator<'code> {
    type Item = Token<'code>;

    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.current;
        self.skip_whitespace();

        let c = self.advance();

        match c {
            Some("(") => Some(self.build_token("(", TokenType::LeftParen)),
            Some(")") => Some(self.build_token(")", TokenType::RightParen)),
            Some("{") => Some(self.build_token("{", TokenType::LeftBrace)),
            Some("}") => Some(self.build_token("}", TokenType::RightBrace)),
            Some(";") => Some(self.build_token(";", TokenType::Semicolon)),
            Some(",") => Some(self.build_token(",", TokenType::Comma)),
            Some(".") => Some(self.build_token(".", TokenType::Dot)),
            Some("-") => Some(self.build_token("-", TokenType::Minus)),
            Some("+") => Some(self.build_token("+", TokenType::Plus)),
            Some("/") => Some(self.build_token("/", TokenType::Slash)),
            Some("*") => Some(self.build_token("*", TokenType::Star)),
            Some("!") => if self.match_char("=") {
                Some(self.build_token("!=", TokenType::BangEqual))
            } else {
                Some(self.build_token("!", TokenType::Bang))
            }
            Some("=") => if self.match_char("=") {
                Some(self.build_token("==", TokenType::EqualEqual))
            } else {
                Some(self.build_token("=", TokenType::Equal))
            }
            Some("<") => if self.match_char("=") {
                Some(self.build_token("<=", TokenType::LessEqual))
            } else {
                Some(self.build_token("<", TokenType::Less))
            }
            Some(">") => if self.match_char("=") {
                Some(self.build_token(">=", TokenType::GreaterEqual))
            } else {
                Some(self.build_token("=", TokenType::Greater))
            }
            Some(_) => panic!("Unexpected character"),
            None => None
        }
    }
}
