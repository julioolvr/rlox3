#[derive(Debug)]
pub struct Token<'code> {
    pub code: &'code str,
    pub line: usize,
    pub token_type: TokenType,
}

impl<'code> Token<'code> {
    pub fn new(code: &'code str, line: usize, token_type: TokenType) -> Token {
        Token { code, line, token_type }
    }
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error,
    Eof
}
