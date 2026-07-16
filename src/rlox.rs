use std::{fmt::format, vec};

use crate::rlox::{RloxExpression::Primary, RloxPrimaryExpression::{Grouping, Number}, RloxStatement::PrintStatment};



#[derive(Clone, Debug, PartialEq)]
pub enum RloxToken {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, 
    Less, LessEqual,

    Identifier(String), String(String), Number(f64),

    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

pub enum RloxScanResult {
    Ignored,
    FoundToken(Token),
    Error(String)
}
#[derive(Clone)]
pub struct Token {
    pub rlox_token: RloxToken,
    pub lexeme: String,
    pub line: usize,
    pub col: usize
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FoundToken {{ rlox_token: {:?}, lexeme: {}, line: {}, col: {} }}", self.rlox_token, self.lexeme, self.line, self.col)
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FoundToken {{ rlox_token: {:?}, lexeme: {}, line: {}, col: {} }}", self.rlox_token, self.lexeme, self.line, self.col)
    }
}


pub struct RloxScanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    pub line: usize,
    pub col: usize,
}

impl RloxScanner {
    pub fn new(source: String) -> Self 
    {
        RloxScanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn scan_tokens(&mut self, raise_error: fn(usize, usize, &str)) 
    {
        while !self.is_at_end() {
            let token = self.scan_token();
            let _ = match token {
                RloxScanResult::Ignored => {},
                RloxScanResult::FoundToken(token) => self.tokens.push(token),
                RloxScanResult::Error(e) => {
                    // Handle error, e.g., log it or raise an error
                    raise_error(self.line, self.col, &e);
                }
            };
        };
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        self.col += 1;
        c
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        self.col += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn process_string_literal(&mut self) -> RloxScanResult {
        self.start = self.current - 1; // Include the opening quote
        let mut string_value = String::new();
        while !self.is_at_end() && self.peek() != '"' {
            string_value.push(self.advance());
        }
        if self.is_at_end() {
            // Handle unterminated string error
            return RloxScanResult::Error("Unterminated string literal".to_string());
        }
        // Consume the closing "
        self.advance();
        let found_token = Token {
            rlox_token: RloxToken::String(string_value.clone()),
            lexeme: string_value,
            line: self.line,
            col: self.col,
        };
        RloxScanResult::FoundToken(found_token)
    }

    fn process_identifier(&mut self) -> RloxScanResult {
        self.start = self.current - 1; // Include the first character of the identifier
        while !self.is_at_end() && self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        match text {
            "and" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::And, lexeme: text.to_string(), line: self.line, col: self.col }),
            "class" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Class, lexeme: text.to_string(), line: self.line, col: self.col }),
            "else" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Else, lexeme: text.to_string(), line: self.line, col: self.col }),
            "false" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::False, lexeme: text.to_string(), line: self.line, col: self.col }),
            "for" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::For, lexeme: text.to_string(), line: self.line, col: self.col }),
            "fun" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Fun, lexeme: text.to_string(), line: self.line, col: self.col }),
            "if" => return RloxScanResult::FoundToken(Token { rlox_token:  RloxToken::If, lexeme: text.to_string(), line: self.line, col: self.col }),
            "nil" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Nil, lexeme: text.to_string(), line: self.line, col: self.col }),
            "or" => return RloxScanResult::FoundToken(Token { rlox_token:  RloxToken::Or, lexeme: text.to_string(), line: self.line, col: self.col }),
            "print" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Print, lexeme: text.to_string(), line: self.line, col: self.col }),
            "return" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Return, lexeme: text.to_string(), line: self.line, col: self.col }),
            "super" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Super, lexeme: text.to_string(), line: self.line, col: self.col }),
            "this" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::This, lexeme: text.to_string(), line: self.line, col: self.col }),
            "true" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::True, lexeme: text.to_string(), line: self.line, col: self.col }),
            "var" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Var, lexeme: text.to_string(), line: self.line, col: self.col }),
            "while" => return RloxScanResult::FoundToken(Token { rlox_token: RloxToken::While, lexeme: text.to_string(), line: self.line, col: self.col }),
            _ => return RloxScanResult::FoundToken(Token {
                rlox_token: RloxToken::Identifier(text.to_string()),
                lexeme: text.to_string(),
                line: self.line,
                col: self.col,
            })
        }


        
    }

    pub fn scan_token(&mut self) -> RloxScanResult {
        let c = self.advance();
        let found_token = match c {
            '(' => RloxScanResult::FoundToken(Token { rlox_token: RloxToken::LeftParen, lexeme: "(".to_string(), line: self.line, col: self.col }),
            ')' => RloxScanResult::FoundToken(Token { rlox_token: RloxToken::RightParen, lexeme: ")".to_string(), line: self.line, col: self.col }),
            '{' => RloxScanResult::FoundToken(Token { rlox_token: RloxToken::LeftBrace, lexeme: "{".to_string(), line: self.line, col: self.col }),
            '}' => RloxScanResult::FoundToken(Token { rlox_token: RloxToken::RightBrace, lexeme: "}".to_string(), line: self.line, col: self.col }),
            ',' => RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Comma, lexeme: ",".to_string(), line: self.line, col: self.col }),
            '.' => RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Dot, lexeme: ".".to_string(), line: self.line, col: self.col }),
            '-' => RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Minus, lexeme: "-".to_string(), line: self.line, col: self.col }),
            '+' => RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Plus, lexeme: "+".to_string(), line: self.line, col: self.col }),
            ';' => RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Semicolon, lexeme: ";".to_string(), line: self.line, col: self.col }),
            '*' => RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Star, lexeme: "*".to_string(), line: self.line, col: self.col }),
            '!' => {
                if self.match_next('=') {
                    RloxScanResult::FoundToken(Token { rlox_token: RloxToken::BangEqual, lexeme: "!=".to_string(), line: self.line, col: self.col })
                } else {
                    RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Bang, lexeme: "!".to_string(), line: self.line, col: self.col })
                }
            }
            '=' => {
                if self.match_next('=') {
                    RloxScanResult::FoundToken( Token { rlox_token: RloxToken::EqualEqual, lexeme: "==".to_string(), line: self.line, col: self.col })  
                } else {
                    RloxScanResult::FoundToken( Token { rlox_token: RloxToken::Equal, lexeme: "=".to_string(), line: self.line, col: self.col })
                }
            }
            '<' => {
                if self.match_next('=') {
                    RloxScanResult::FoundToken( Token { rlox_token: RloxToken::LessEqual, lexeme: "<=".to_string(), line: self.line, col: self.col })
                } else {
                    RloxScanResult::FoundToken( Token { rlox_token: RloxToken::Less, lexeme: "<".to_string(), line: self.line, col: self.col })
                }
            }
            '>' => {
                if self.match_next('=') {
                    RloxScanResult::FoundToken(Token { rlox_token: RloxToken::GreaterEqual, lexeme: ">=".to_string(), line: self.line, col: self.col })
                } else {
                    RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Greater, lexeme: ">".to_string(), line: self.line, col: self.col })
                }
            }
            '/' => {
                if self.match_next('/') {
                    // This is a comment, skip until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    RloxScanResult::Ignored
                } else {
                    RloxScanResult::FoundToken(Token { rlox_token: RloxToken::Slash, lexeme: "/".to_string(), line: self.line, col: self.col })
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
               RloxScanResult::Ignored
            }
            '\n' => {
                self.line += 1;
                self.col = 0;
                RloxScanResult::Ignored
            }
            '"' => self.process_string_literal(),
            _ => {

               if  c.is_digit(10) {
                    let mut number = c.to_string();
                    while !self.is_at_end() && self.peek().is_digit(10) {
                        number.push(self.advance());
                    }
                    if !self.is_at_end() && self.peek() == '.' {
                        number.push(self.advance());
                        while !self.is_at_end() && self.peek().is_digit(10) {
                            number.push(self.advance());
                        }
                    }
                    let num_value = number.parse();
                    match num_value {
                        Ok(value) => {
                            RloxScanResult::FoundToken(Token {
                                rlox_token: RloxToken::Number(value),
                                lexeme: number,
                                line: self.line,
                                col: self.col,
                            })
                        }
                        Err(_) => {
                            RloxScanResult::Error("Invalid number format".to_string())
                        }
                    }
                } else if c.is_alphanumeric() {
                    self.process_identifier()
                }
                else {
                    RloxScanResult::Error(format!("Unexpected character: {}", c))    
                }   
            }               
        };

        return found_token;
        }
        
     
    }



#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
#[derive(Debug, Clone)]
pub enum RloxBinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, Clone)]
pub enum RloxUnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub enum RloxExpression {
    Binary {
        left: Box<RloxExpression>,
        operator: Token,
        right: Box<RloxExpression>,
    },
    Unary {
        operator: Token,
        right: Box<RloxExpression>,
    },
    Primary(RloxPrimaryExpression),
    // Add other expression types here (e.g., Binary, Unary, etc.)
}

#[derive(Debug, Clone)]
pub enum RloxPrimaryExpression {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Grouping(Box<RloxExpression>),
}

#[derive(Debug, Clone)]
pub struct RloxEvalError {
    pub token: Token,
    pub message: String
}

#[derive(Debug, Clone, PartialEq)]
pub enum RloxValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil
}


impl RloxValue {
    pub fn is_truthy(&self) -> bool 
    {
        match self {
            RloxValue::Number(n) => *n == 0.0f64,
            RloxValue::String(s) => !s.is_empty(),
            RloxValue::Bool(b) => *b,
            RloxValue::Nil => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct RloxParseError {
    pub message: String,
    pub token: Token
}

pub trait Evaluateable
{
    fn evaluate(&self) -> Result<RloxValue,RloxEvalError>;
}

impl Evaluateable for RloxPrimaryExpression {
    fn evaluate(&self) -> Result<RloxValue,RloxEvalError> {
        match self {
            Number(n) => Ok(RloxValue::Number(*n)),
            RloxPrimaryExpression::String(s) => Ok(RloxValue::String(s.clone())),
            RloxPrimaryExpression::Bool(b) => Ok(RloxValue::Bool(*b)),
            RloxPrimaryExpression::Nil => Ok(RloxValue::Nil),
            Grouping(expr) => expr.evaluate()
        }
    }
}

impl Evaluateable for RloxExpression {
    fn evaluate(&self) -> Result<RloxValue,RloxEvalError>
    {
        match self {
            Primary(prim) => {
                prim.evaluate()
            },
            RloxExpression::Binary { left, operator, right } => {
                let left_val = match left.evaluate() {
                    Ok(l) => l,
                    Err(e) => return Err(RloxEvalError { token: operator.clone(), message: format!("Expected expresion as left operand of {:?} found {:?}", operator, e) })
                };

                // alternative more concise approach can be:
                let right_val = right.evaluate().map_err(|e| RloxEvalError {
                    token: operator.clone(),
                    message: format! ("Expected expression as right operand of {:?} found {:?}", operator, e)
                })?;
                
                match operator.rlox_token {
                    RloxToken::And => {
                        return Ok(RloxValue::Bool(left_val.is_truthy() && right_val.is_truthy()));
                    }
                    RloxToken::Greater => {
                        match (&left_val, &right_val) {
                            (RloxValue::Number(l), RloxValue::Number(r)) => Ok(RloxValue::Bool(l > r)),
                            _ => Err(RloxEvalError {
                                token: operator.clone(),
                                message: format! ("Can only compare numbers, not {:?} and {:?}", left_val, right_val)
                            })
                        }
                    }
                    RloxToken::GreaterEqual => {
                        match (&left_val, &right_val) {
                            (RloxValue::Number(l), RloxValue::Number(r)) => Ok(RloxValue::Bool(l >= r)),
                            _ => Err(RloxEvalError {
                                token: operator.clone(),
                                message: format! ("Can only compare numbers, not {:?} and {:?}", left_val, right_val)
                            })
                        }
                    }
                    RloxToken::Or => {
                        return Ok(RloxValue::Bool(left_val.is_truthy() || right_val.is_truthy()));
                    }
                    RloxToken::Equal => {
                        Err(RloxEvalError { token: operator.clone(), message: format!("Binary operator {:?} not implemented", operator.rlox_token) })
                    }
                    RloxToken::EqualEqual => {
                        return Ok(RloxValue::Bool(left_val == right_val));
                    }
                    RloxToken::BangEqual => {
                        return Ok(RloxValue::Bool(left_val != right_val));
                    }
                    RloxToken::Less => {
                        match (&left_val, &right_val) {
                            (RloxValue::Number(l), RloxValue::Number(r)) => Ok(RloxValue::Bool(l < r)),
                            _ => Err(RloxEvalError {
                                token: operator.clone(),
                                message: format! ("Can only compare numbers, not {:?} and {:?}", left_val, right_val)
                            })
                        }
                    }
                    RloxToken::LessEqual => {
                        match (&left_val, &right_val) {
                            (RloxValue::Number(l), RloxValue::Number(r)) => Ok(RloxValue::Bool(l <= r)),
                            _ => Err(RloxEvalError {
                                token: operator.clone(),
                                message: format! ("Can only compare numbers, not {:?} and {:?}", left_val, right_val)
                            })
                        }
                    }
                    RloxToken::Plus => {
                        match (&left_val, &right_val) {
                            (RloxValue::Number(l), RloxValue::Number(r)) => Ok(RloxValue::Number(l + r)),
                            _ => Err(RloxEvalError {
                                token: operator.clone(),
                                message: format! ("Operator + not defined for {:?} and {:?}", left_val, right_val)
                            })
                        }
                    }
                    RloxToken::Minus => {
                        match (&left_val, &right_val) {
                            (RloxValue::Number(l), RloxValue::Number(r)) => Ok(RloxValue::Number(l / r)),
                            _ => Err(RloxEvalError {
                                token: operator.clone(),
                                message: format! ("Operator - not defined for {:?} and {:?}", left_val, right_val)
                            })
                        }
                    }
                    RloxToken::Star => {
                        match (&left_val, &right_val) {
                            (RloxValue::Number(l), RloxValue::Number(r)) => Ok(RloxValue::Number(l * r)),
                            _ => Err(RloxEvalError {
                                token: operator.clone(),
                                message: format! ("Operator * not defined for {:?} and {:?}", left_val, right_val)
                            })
                        }
                    }
                    RloxToken::Slash => {
                        match (&left_val, &right_val) {
                            (RloxValue::Number(l), RloxValue::Number(r)) => Ok(RloxValue::Number(l / r)),
                            _ => Err(RloxEvalError {
                                token: operator.clone(),
                                message: format! ("Operator / not defined for {:?} and {:?}", left_val, right_val)
                            })
                        }
                    }
                    _ => Err(RloxEvalError { token: operator.clone(), message: format!("Unexpected binary operator {:?}", operator.rlox_token) })
                }
            },
            RloxExpression::Unary { operator, right } => {
                match operator.rlox_token {
                    RloxToken::Bang => {
                        Err(RloxEvalError { token: operator.clone(), message: format!("Unary operator {:?} not implemented", operator.rlox_token) })
                    },
                    RloxToken::Minus => {
                        Err(RloxEvalError { token: operator.clone(), message: format!("Unary operator {:?} not implemented", operator.rlox_token) })
                    },
                    _ => Err(RloxEvalError { token: operator.clone(), message: format!("Unexpected unary operator {:?}, expected ! or -", operator.rlox_token) })
                }
            }
        }
    }
}

impl Parser {

    pub fn new(tokens: Vec<Token>, error_func:fn(usize, usize, &str) ) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<RloxStatement>, RloxParseError> {
        let mut statements: Vec<RloxStatement> = Vec::new();
        while !self.is_at_end() {
            let new_stmt = self.parse_statement();
            match new_stmt {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(e) => {
                    println!("{:?}",e);
                    return Err(e);
                }
                
            }
        }
        return Ok(statements);
    }

    fn synchronize(&mut self)
    {
        self.advance();

        while !self.is_at_end() {
            if self.previous().rlox_token == RloxToken::Semicolon {return}

            match self.peek().rlox_token {
            RloxToken::Class | RloxToken::Fun | RloxToken::Var | RloxToken::For
            | RloxToken::If | RloxToken::While | RloxToken::Print | RloxToken::Return => {return;}
            _ => { self.advance(); }
            }
        }
       
    }

    fn previous(&mut self) -> Token {
        return self.tokens.get(self.current-1).unwrap_or(&Token { rlox_token:RloxToken::Eof, lexeme: "".to_string(), line: 1, col: 1 }).clone();
    }

    fn peek(&mut self) -> Token {
        return self.tokens.get(self.current).unwrap_or(&Token { rlox_token:RloxToken::Eof, lexeme: "".to_string(), line: 1, col: 1 }).clone()
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek().rlox_token == RloxToken::Eof;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end()
        {
            self.current+=1;
        }
        return self.previous()
    }

    fn check(&mut self, token_type: RloxToken) -> bool {
        if self.is_at_end()
        {
            return false;
        }
        let current_token = self.peek();
        match (current_token.rlox_token, token_type) {
            (RloxToken::Number(_), RloxToken::Number(_)) => true,
            (RloxToken::String(_), RloxToken::String(_)) => true,
            (token, to_match)=> token == to_match
        }
    }

    fn match_tokens(&mut self, token_types : Vec<RloxToken>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn consume(&mut self, token_type: RloxToken, err_msg: String) -> Result<Token, RloxParseError>
    {
        if self.check(token_type) {
            return Ok(self.advance())
        }

        return Err(RloxParseError { message: err_msg, token: self.peek() })
    }

    fn parse_primary(&mut self) -> Result<RloxExpression, RloxParseError>
    {
      

        if self.match_tokens(vec![RloxToken::False])
        {
            return Ok(RloxExpression::Primary(RloxPrimaryExpression::Bool(false)))
        }
        if self.match_tokens(vec![RloxToken::True])
        {
            return Ok(RloxExpression::Primary(RloxPrimaryExpression::Bool(true)))
        }
        if self.match_tokens(vec![RloxToken::Nil])
        {
            return Ok(RloxExpression::Primary(RloxPrimaryExpression::Nil))
        }

        if self.match_tokens(vec![RloxToken::String("".to_string()),RloxToken::Number(0.0)])
        {
            let prev_token = self.previous();
            return match prev_token.rlox_token {
                RloxToken::String(s) => Ok(RloxExpression::Primary(RloxPrimaryExpression::String(s))),
                RloxToken::Number(n) => Ok(RloxExpression::Primary(RloxPrimaryExpression::Number(n))),
                _ => Err(RloxParseError { message: "Unexpected parse error. Expect number or string".to_string(), token: self.peek() })
            }
        }

        if self.match_tokens(vec![RloxToken::LeftParen]) 
        {
            let expr = self.parse_expression()?;
            self.consume(RloxToken::RightParen, "Expect ')' after expression.".to_string())?;
            return Ok(RloxExpression::Primary(RloxPrimaryExpression::Grouping(Box::new(expr))))
        }
        return Err(RloxParseError { message: "Unexpected parse error. Expect expression".to_string(), token: self.peek() });
    }

    fn parse_unary(&mut self) -> Result<RloxExpression, RloxParseError>
    {
        if self.match_tokens(vec![RloxToken::Bang, RloxToken::Minus]) {
            let operator = self.previous();
            let right = self.parse_unary()?;
            return Ok(RloxExpression::Unary { operator, right: Box::new(right) })
        }

        return self.parse_primary();
    }

    fn parse_factor(&mut self) -> Result<RloxExpression, RloxParseError>
    {
        let mut expr  = self.parse_unary()?;

        while self.match_tokens(vec![RloxToken::Slash, RloxToken::Star]) {
            let operator = self.previous();
            let right = self.parse_unary()?;
            expr = RloxExpression::Binary { left: Box::new(expr), operator, right: Box::new(right) }
        }
        return Ok(expr);
    }

    fn parse_term(&mut self) -> Result<RloxExpression, RloxParseError> {
        let mut expr = self.parse_factor()?;

        while self.match_tokens(vec!(RloxToken::Minus, RloxToken::Plus)) {
            let operator = self.previous();
            let right = self.parse_factor()?;
            expr = RloxExpression::Binary { left: Box::new(expr), operator, right: Box::new(right) }
        }

        return Ok(expr);
    }

    fn parse_comparison(&mut self) -> Result<RloxExpression, RloxParseError> {
        let mut expr = self.parse_term()?;

        while self.match_tokens(vec!(RloxToken::Greater, RloxToken::GreaterEqual, RloxToken::Less, RloxToken::LessEqual))
        {
            let operator = self.previous();
            let right = self.parse_term()?;
            expr = RloxExpression::Binary { left: Box::new(expr), operator, right: Box::new(right)}
        }

        return Ok(expr);
    }

    fn parse_equality(&mut self) -> Result<RloxExpression, RloxParseError> {
        let mut expr = self.parse_comparison()?;

        while self.match_tokens(vec!(RloxToken::BangEqual, RloxToken::EqualEqual))
        {
            let operator = self.previous();
            let right = self.parse_comparison()?;
            expr = RloxExpression::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        return Ok(expr);
    }

    fn parse_expression(&mut self) -> Result<RloxExpression, RloxParseError> {
        // Implement the parsing logic here
        self.parse_equality()
    }

    fn parse_print_statement(&mut self) -> Result<RloxStatement, RloxParseError>
    {
        let value = self.parse_expression()?;
        self.consume(RloxToken::Semicolon, "Expect ';' after value.".to_string());
        return Ok(RloxStatement::PrintStatment(value))
    }

    fn parse_expression_statement(&mut self) -> Result<RloxStatement, RloxParseError>
    {
        let expr = self.parse_expression()?;
        self.consume(RloxToken::Semicolon, "Expect ';' after expression.".to_string());
        return Ok(RloxStatement::ExpressionStatement(expr))
    }

    fn parse_statement(&mut self) -> Result<RloxStatement, RloxParseError> {
        if self.match_tokens(vec![RloxToken::Print]) {
            return self.parse_print_statement()
        }
        return self.parse_expression_statement();
    }
}


pub enum RloxStatement {
    ExpressionStatement(RloxExpression),
    PrintStatment(RloxExpression)
}

pub fn interpret(statemet: RloxStatement)
{
    match statemet {
        RloxStatement::PrintStatment(val) => {
            val.evaluate();
        }
        RloxStatement::ExpressionStatement(expr) => {
            let val = expr.evaluate();
            if let Ok(ok_val) = val {
                println!("{:?}",ok_val);
            }
        }
    }
}