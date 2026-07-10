

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
        operator: RloxBinaryOperator,
        right: Box<RloxExpression>,
    },
    Unary {
        operator: RloxUnaryOperator,
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
    RloxExpression(Box<RloxExpression>),
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn previous(&mut self) -> Token {
        return self.tokens.get(self.current-1).unwrap().clone();
    }

    fn peek(&mut self) -> Token {
        return self.tokens.get(self.current).unwrap().clone();
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
        return self.peek().rlox_token == token_type;
    }

    fn match_tokens(&mut self, token_types : Vec<RloxToken>) -> bool {
        for token_type in token_types {
            if(self.check(token_type)) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    pub fn parse_comparison(&mut self) -> Result<RloxExpression, String> {
        let mut expr = term();

        while self.match_tokens(vec!(RloxToken::Greater, RloxToken::GreaterEqual, RloxToken::Less, RloxToken::LessEqual))
        {
            let operator = self.previous();
            let right = term();
            expr = RloxExpression::Binary { left: expr, operator, right}
        }

        return expr;
    }

    pub fn parse_equality(&mut self) -> Result<RloxExpression, String> {
        let mut expr = parse_comparison();

        while(self.match_tokens(vec!(RloxToken::BangEqual, RloxToken::EqualEqual)))
        {
            let operator = self.previous();
            let right = self.parse_comparison();
            expr = RloxExpression::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        return expr;
    }

    pub fn parse_expression(&mut self) -> Result<RloxExpression, String> {
        // Implement the parsing logic here
        self.parse_equality()
    }
}