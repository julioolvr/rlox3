use crate::rlox::chunk::Chunk;
use crate::rlox::disassembler::disassemble;
use crate::rlox::instruction::Instruction;
use crate::rlox::scanner::{Scanner, ScannerIterator};
use crate::rlox::token::{Token, TokenType};
use crate::rlox::value::Value;
use crate::rlox::vm::InterpretError;

pub fn compile(code: &str, chunk: &mut Chunk) -> Result<(), InterpretError> {
    let mut compiler = Compiler::new(code, chunk);
    compiler.compile()
}

struct Compiler<'code> {
    parser: Parser<'code>,
    chunk: &'code mut Chunk,
    scanner: ScannerIterator<'code>,
}

impl<'a> Compiler<'a> {
    fn new(code: &'a str, chunk: &'a mut Chunk) -> Compiler<'a> {
        let mut scanner = Scanner::new();

        Compiler {
            parser: Parser::new(),
            chunk,
            scanner: scanner.scan(code),
        }
    }

    fn compile(&mut self) -> Result<(), InterpretError> {
        self.advance();
        self.expression();
        self.end_compiler();

        if self.parser.had_error {
            Err(InterpretError::CompileError)
        } else {
            Ok(())
        }
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.take();

        loop {
            self.parser.current = self.scanner.next();

            match self.parser.current.as_ref() {
                Some(token) if token.token_type != TokenType::Error => break,
                Some(token) => {
                    let code = token.code;
                    self.error_at_current(code);
                }
                // TODO: Tried to advance past the end of the code
                // This can happen if the scanner keeps generating error tokens.
                // Right now, the only way to end up with a token of those is
                // with an unterminated string - and the compiler does not
                // compile strings, so there's really no way to run and test
                // this. Once strings are implemented I'll come back here and
                // properly handle the error.
                None => unimplemented!(),
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        match self.parser.current.as_ref() {
            Some(token) if token.token_type == token_type => self.advance(),
            _ => self.error_at_current(message),
        }
    }

    fn error(&mut self, message: &str) {
        let previous_token = self
            .parser
            .previous
            .as_ref()
            .expect("Tried to report error on previous token but there wasn't any");
        self.error_at(previous_token, message);
        self.mark_error();
    }

    fn error_at_current(&mut self, message: &str) {
        let current_token = self
            .parser
            .current
            .as_ref()
            .expect("Tried to report error on current token but there wasn't any");
        self.error_at(current_token, message);
        self.mark_error();
    }

    fn error_at(&self, token: &Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }

        eprint!("[line {}] Error", token.line);

        // The book here checks for Token::Eof which right now we don't have,
        // we might need it later.
        if token.token_type != TokenType::Error {
            eprint!(" at '{}'", token.code);
        }

        eprintln!(": {}", message);
    }

    fn mark_error(&mut self) {
        self.parser.had_error = true;
        self.parser.panic_mode = true;
    }

    fn emit_instruction(&mut self, instruction: Instruction) {
        let line = self.parser.previous.as_ref().unwrap().line;
        self.chunk.add_instruction(instruction, line);
    }

    fn end_compiler(&mut self) {
        self.emit_return();

        if cfg!(debug_assertions) && !self.parser.had_error {
            disassemble(&self.chunk, "code");
        }
    }

    fn emit_return(&mut self) {
        let line = self.parser.previous.as_ref().unwrap().line;
        self.chunk.add_instruction(Instruction::OpReturn, line);
    }

    fn emit_constant(&mut self, value: Value) {
        let line = self.parser.previous.as_ref().unwrap().line;
        let constant_index = self.chunk.add_constant(value);
        self.chunk
            .add_instruction(Instruction::OpConstant(constant_index), line);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        use TokenType::*;

        self.advance();

        match self.parser.previous.as_ref() {
            Some(token) if token.token_type == Minus => self.unary(),
            Some(token) if token.token_type == Bang => self.unary(),
            Some(token) if token.token_type == LeftParen => self.grouping(),
            Some(token) if token.token_type == Number => {
                let code = token.code;
                self.number(code);
            }
            Some(token)
                if token.token_type == False
                    || token.token_type == True
                    || token.token_type == Nil =>
            {
                self.literal()
            }
            _ => self.error("Expected prefix expression"),
        }

        while precedence
            <= self
                .parser
                .current
                .as_ref()
                .unwrap()
                .token_type
                .precedence()
        {
            self.advance();

            match self.parser.previous.as_ref() {
                Some(token)
                    if token.token_type == Minus
                        || token.token_type == Plus
                        || token.token_type == Slash
                        || token.token_type == Star
                        || token.token_type == Greater
                        || token.token_type == GreaterEqual
                        || token.token_type == Less
                        || token.token_type == LessEqual
                        || token.token_type == EqualEqual =>
                {
                    self.binary()
                }
                _ => self.error("Expected infix expression"),
            }
        }
    }

    fn number(&mut self, code: &str) {
        self.emit_constant(Value::Number(code.parse().unwrap()));
    }

    fn literal(&mut self) {
        match self.parser.previous.as_ref().unwrap().token_type {
            TokenType::False => self.emit_instruction(Instruction::OpFalse),
            TokenType::True => self.emit_instruction(Instruction::OpTrue),
            TokenType::Nil => self.emit_instruction(Instruction::OpNil),
            _ => unimplemented!(),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression");
    }

    fn unary(&mut self) {
        let operator_type = self
            .parser
            .previous
            .as_ref()
            .expect("Did not find previous token when parsing unary expression")
            .token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_instruction(Instruction::OpNegate),
            TokenType::Bang => self.emit_instruction(Instruction::OpNot),
            _ => unimplemented!(),
        }
    }

    fn binary(&mut self) {
        let operator_type = self
            .parser
            .previous
            .as_ref()
            .expect("Did not find previous token when parsing binary expression")
            .token_type;

        self.parse_precedence(operator_type.precedence().higher());

        match operator_type {
            TokenType::Plus => self.emit_instruction(Instruction::OpAdd),
            TokenType::Minus => self.emit_instruction(Instruction::OpSubtract),
            TokenType::Star => self.emit_instruction(Instruction::OpMultiply),
            TokenType::Slash => self.emit_instruction(Instruction::OpDivide),
            TokenType::BangEqual => {
                self.emit_instruction(Instruction::OpEqual);
                self.emit_instruction(Instruction::OpNot);
            }
            TokenType::EqualEqual => self.emit_instruction(Instruction::OpEqual),
            TokenType::Greater => self.emit_instruction(Instruction::OpGreater),
            TokenType::GreaterEqual => {
                self.emit_instruction(Instruction::OpLess);
                self.emit_instruction(Instruction::OpNot);
            }
            TokenType::Less => self.emit_instruction(Instruction::OpLess),
            TokenType::LessEqual => {
                self.emit_instruction(Instruction::OpGreater);
                self.emit_instruction(Instruction::OpNot);
            }
            _ => unimplemented!(),
        }
    }
}

struct Parser<'a> {
    previous: Option<Token<'a>>,
    current: Option<Token<'a>>,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    fn new() -> Parser<'a> {
        Parser {
            previous: None,
            current: None,
            had_error: false,
            panic_mode: false,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    fn higher(&self) -> Precedence {
        use Precedence::*;

        // TODO: Is this the best way to get the next higher precedence level?
        // TODO: Should Primary return Primary? Or should the whole function
        // return an Option<Precedence>?
        match self {
            None => Assignment,
            Assignment => Or,
            Or => And,
            And => Equality,
            Equality => Comparison,
            Comparison => Term,
            Term => Factor,
            Factor => Unary,
            Unary => Call,
            Call => Primary,
            Primary => Primary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Pending test - the compiler doesn't support strings for now
    // #[test]
    // fn test_compile_error() {
    //     let mut chunk = Chunk::new();
    //     let mut compiler = Compiler::new("\"unterminated string", &mut chunk);
    //     let result = compiler.compile();
    //     assert_eq!(result, Err(InterpretError::CompileError));
    // }

    #[test]
    fn test_compile_number() {
        let mut chunk = Chunk::new();
        let result = compile("123.4", &mut chunk);
        assert!(matches!(result, Ok(())));

        let mut instructions = chunk.instructions();
        let first_instruction = instructions.next().unwrap();
        assert!(matches!(first_instruction, &Instruction::OpConstant(_)));
        let constant_index = match first_instruction {
            Instruction::OpConstant(n) => n,
            _ => unreachable!(),
        };
        assert_eq!(chunk.constant_at(*constant_index), &Value::Number(123.4));

        assert!(matches!(
            instructions.next().unwrap(),
            &Instruction::OpReturn
        ));
    }

    #[test]
    fn test_compile_unary_operator() {
        let mut chunk = Chunk::new();
        let result = compile("-123.4", &mut chunk);
        assert!(matches!(result, Ok(())));

        let mut instructions = chunk.instructions();

        let constant_instruction = instructions.next().unwrap();
        assert!(matches!(constant_instruction, &Instruction::OpConstant(_)));
        let constant_index = match constant_instruction {
            Instruction::OpConstant(n) => n,
            _ => unreachable!(),
        };
        assert_eq!(chunk.constant_at(*constant_index), &Value::Number(123.4));

        let negate_instruction = instructions.next().unwrap();
        assert!(matches!(negate_instruction, &Instruction::OpNegate));

        assert!(matches!(
            instructions.next().unwrap(),
            &Instruction::OpReturn
        ));
    }

    #[test]
    fn test_compile_binary_operator() {
        let mut chunk = Chunk::new();
        let result = compile("1 + 2", &mut chunk);
        assert!(matches!(result, Ok(())));

        let mut instructions = chunk.instructions();

        let first_operand_instruction = instructions.next().unwrap();
        assert!(matches!(
            first_operand_instruction,
            &Instruction::OpConstant(_)
        ));
        let constant_index = match first_operand_instruction {
            Instruction::OpConstant(n) => n,
            _ => unreachable!(),
        };
        assert_eq!(chunk.constant_at(*constant_index), &Value::Number(1.0));

        let second_operand_instruction = instructions.next().unwrap();
        assert!(matches!(
            second_operand_instruction,
            &Instruction::OpConstant(_)
        ));
        let constant_index = match second_operand_instruction {
            Instruction::OpConstant(n) => n,
            _ => unreachable!(),
        };
        assert_eq!(chunk.constant_at(*constant_index), &Value::Number(2.0));

        let add_instruction = instructions.next().unwrap();
        assert!(matches!(add_instruction, &Instruction::OpAdd));

        assert!(matches!(
            instructions.next().unwrap(),
            &Instruction::OpReturn
        ));
    }

    #[test]
    fn test_arithmetic_precedence() {
        let mut chunk = Chunk::new();
        let result = compile("1 + 2 * 3", &mut chunk);
        assert!(matches!(result, Ok(())));

        let mut instructions = chunk.instructions();

        let first_operand_instruction = instructions.next().unwrap();
        assert!(matches!(
            first_operand_instruction,
            &Instruction::OpConstant(_)
        ));
        let constant_index = match first_operand_instruction {
            Instruction::OpConstant(n) => n,
            _ => unreachable!(),
        };
        assert_eq!(chunk.constant_at(*constant_index), &Value::Number(1.0));

        let second_operand_instruction = instructions.next().unwrap();
        assert!(matches!(
            second_operand_instruction,
            &Instruction::OpConstant(_)
        ));
        let constant_index = match second_operand_instruction {
            Instruction::OpConstant(n) => n,
            _ => unreachable!(),
        };
        assert_eq!(chunk.constant_at(*constant_index), &Value::Number(2.0));

        let third_operand_instruction = instructions.next().unwrap();
        assert!(matches!(
            third_operand_instruction,
            &Instruction::OpConstant(_)
        ));
        let constant_index = match third_operand_instruction {
            Instruction::OpConstant(n) => n,
            _ => unreachable!(),
        };
        assert_eq!(chunk.constant_at(*constant_index), &Value::Number(3.0));

        let multiply_instruction = instructions.next().unwrap();
        assert!(matches!(multiply_instruction, &Instruction::OpMultiply));

        let add_instruction = instructions.next().unwrap();
        assert!(matches!(add_instruction, &Instruction::OpAdd));

        assert!(matches!(
            instructions.next().unwrap(),
            &Instruction::OpReturn
        ));
    }

    #[test]
    fn test_grouping() {
        let mut chunk = Chunk::new();
        let result = compile("(1 + 2) * 3", &mut chunk);
        assert!(matches!(result, Ok(())));

        let mut instructions = chunk.instructions();

        let first_operand_instruction = instructions.next().unwrap();
        assert!(matches!(
            first_operand_instruction,
            &Instruction::OpConstant(_)
        ));
        let constant_index = match first_operand_instruction {
            Instruction::OpConstant(n) => n,
            _ => unreachable!(),
        };
        assert_eq!(chunk.constant_at(*constant_index), &Value::Number(1.0));

        let second_operand_instruction = instructions.next().unwrap();
        assert!(matches!(
            second_operand_instruction,
            &Instruction::OpConstant(_)
        ));
        let constant_index = match second_operand_instruction {
            Instruction::OpConstant(n) => n,
            _ => unreachable!(),
        };
        assert_eq!(chunk.constant_at(*constant_index), &Value::Number(2.0));

        let add_instruction = instructions.next().unwrap();
        assert!(matches!(add_instruction, &Instruction::OpAdd));

        let third_operand_instruction = instructions.next().unwrap();
        assert!(matches!(
            third_operand_instruction,
            &Instruction::OpConstant(_)
        ));
        let constant_index = match third_operand_instruction {
            Instruction::OpConstant(n) => n,
            _ => unreachable!(),
        };
        assert_eq!(chunk.constant_at(*constant_index), &Value::Number(3.0));

        let multiply_instruction = instructions.next().unwrap();
        assert!(matches!(multiply_instruction, &Instruction::OpMultiply));

        assert!(matches!(
            instructions.next().unwrap(),
            &Instruction::OpReturn
        ));
    }

    #[test]
    fn test_grouping_error() {
        let mut chunk = Chunk::new();
        let result = compile("(1 + 2", &mut chunk);
        assert!(matches!(result, Err(InterpretError::CompileError)));
    }
}
