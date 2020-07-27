use crate::rlox::token::{Token, TokenType};

pub struct Scanner {}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn scan<'a>(&mut self, code: &'a String) -> ScannerIterator<'a> {
        ScannerIterator {
            code,
            start: 0,
            current: 0,
            line: 1,
        }
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

    fn string(&mut self) -> Token<'code> {
        while self.peek() != Some("\"") && !self.is_at_end() {
            if self.peek() == Some("\n") {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return self.build_token("Unterminated string.", TokenType::Error);
        }

        // Closing quote
        self.advance();
        self.build_token(&self.code[self.start..self.current], TokenType::String)
    }

    fn number(&mut self) -> Token<'code> {
        while self.peek().map_or(false, |digit| is_digit(digit)) {
            self.advance();
        }

        if self.peek() == Some(".") && self.peek_next().map_or(false, |digit| is_digit(digit)) {
            self.advance();

            while self.peek().map_or(false, |digit| is_digit(digit)) {
                self.advance();
            }
        }

        self.build_token(&self.code[self.start..self.current], TokenType::Number)
    }

    fn identifier_or_keyword(&mut self) -> Token<'code> {
        while self
            .peek()
            .map_or(false, |alpha| is_alpha(alpha) || is_digit(alpha))
        {
            self.advance();
        }

        self.build_identifier_or_keyword_token()
    }

    fn build_identifier_or_keyword_token(&self) -> Token<'code> {
        match &self.code[self.start..self.start + 1] {
            "a" => self.check_keyword(1, "nd", TokenType::And),
            "c" => self.check_keyword(1, "lass", TokenType::Class),
            "e" => self.check_keyword(1, "lse", TokenType::Else),
            "i" => self.check_keyword(1, "f", TokenType::If),
            "n" => self.check_keyword(1, "il", TokenType::Nil),
            "o" => self.check_keyword(1, "r", TokenType::Or),
            "p" => self.check_keyword(1, "rint", TokenType::Print),
            "r" => self.check_keyword(1, "eturn", TokenType::Return),
            "s" => self.check_keyword(1, "uper", TokenType::Super),
            "v" => self.check_keyword(1, "ar", TokenType::Var),
            "w" => self.check_keyword(1, "hile", TokenType::While),
            "f" if self.current - self.start > 1 => {
                match &self.code[self.start + 1..self.start + 2] {
                    "a" => self.check_keyword(2, "lse", TokenType::False),
                    "o" => self.check_keyword(2, "or", TokenType::For),
                    "u" => self.check_keyword(2, "un", TokenType::Fun),
                    _ => self.build_identifier_token(),
                }
            }
            "t" if self.current - self.start > 1 => {
                match &self.code[self.start + 1..self.start + 2] {
                    "h" => self.check_keyword(2, "is", TokenType::This),
                    "r" => self.check_keyword(2, "ue", TokenType::True),
                    _ => self.build_identifier_token(),
                }
            }
            _ => self.build_identifier_token(),
        }
    }

    fn build_identifier_token(&self) -> Token<'code> {
        self.build_token(&self.code[self.start..self.current], TokenType::Identifier)
    }

    fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> Token<'code> {
        if self.current - self.start == rest.len() + start
            && &self.code[self.start + start..self.current] == rest
        {
            self.build_token(&self.code[self.start..self.current], token_type)
        } else {
            self.build_identifier_token()
        }
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
                        break;
                    }
                }
                _ => break,
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
            Some("!") => {
                if self.match_char("=") {
                    Some(self.build_token("!=", TokenType::BangEqual))
                } else {
                    Some(self.build_token("!", TokenType::Bang))
                }
            }
            Some("=") => {
                if self.match_char("=") {
                    Some(self.build_token("==", TokenType::EqualEqual))
                } else {
                    Some(self.build_token("=", TokenType::Equal))
                }
            }
            Some("<") => {
                if self.match_char("=") {
                    Some(self.build_token("<=", TokenType::LessEqual))
                } else {
                    Some(self.build_token("<", TokenType::Less))
                }
            }
            Some(">") => {
                if self.match_char("=") {
                    Some(self.build_token(">=", TokenType::GreaterEqual))
                } else {
                    Some(self.build_token("=", TokenType::Greater))
                }
            }
            Some("\"") => Some(self.string()),
            Some(alpha) if is_alpha(alpha) => Some(self.identifier_or_keyword()),
            Some(digit) if is_digit(digit) => Some(self.number()),
            Some(_) => panic!("Unexpected character"),
            None => None,
        }
    }
}

fn is_digit(possible_digit: &str) -> bool {
    match possible_digit {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
        _ => false,
    }
}

fn is_alpha(possible_alpha: &str) -> bool {
    match possible_alpha {
        "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o"
        | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "A" | "B" | "C"
        | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q"
        | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "_" => true,
        _ => false,
    }
}
