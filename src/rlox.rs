

#[derive(Clone, Debug)]
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

pub enum RloxParseResult {
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
                RloxParseResult::Ignored => {},
                RloxParseResult::FoundToken(token) => self.tokens.push(token),
                RloxParseResult::Error(e) => {
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

    fn process_string_literal(&mut self) -> RloxParseResult {
        let mut string_value = String::new();
        while !self.is_at_end() && self.peek() != '"' {
            string_value.push(self.advance());
        }
        if self.is_at_end() {
            // Handle unterminated string error
            return RloxParseResult::Error("Unterminated string literal".to_string());
        }
        // Consume the closing "
        self.advance();
        let found_token = Token {
            rlox_token: RloxToken::String(string_value.clone()),
            lexeme: string_value,
            line: self.line,
            col: self.col,
        };
        self.add_token(found_token.clone());
        RloxParseResult::FoundToken(found_token)
    }

    fn process_identifier(&mut self) -> RloxParseResult {
        while !self.is_at_end() && self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        match text {
            "and" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::And, lexeme: text.to_string(), line: self.line, col: self.col }),
            "class" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Class, lexeme: text.to_string(), line: self.line, col: self.col }),
            "else" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Else, lexeme: text.to_string(), line: self.line, col: self.col }),
            "false" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::False, lexeme: text.to_string(), line: self.line, col: self.col }),
            "for" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::For, lexeme: text.to_string(), line: self.line, col: self.col }),
            "fun" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Fun, lexeme: text.to_string(), line: self.line, col: self.col }),
            "if" => return RloxParseResult::FoundToken(Token { rlox_token:  RloxToken::If, lexeme: text.to_string(), line: self.line, col: self.col }),
            "nil" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Nil, lexeme: text.to_string(), line: self.line, col: self.col }),
            "or" => return RloxParseResult::FoundToken(Token { rlox_token:  RloxToken::Or, lexeme: text.to_string(), line: self.line, col: self.col }),
            "print" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Print, lexeme: text.to_string(), line: self.line, col: self.col }),
            "return" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Return, lexeme: text.to_string(), line: self.line, col: self.col }),
            "super" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Super, lexeme: text.to_string(), line: self.line, col: self.col }),
            "this" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::This, lexeme: text.to_string(), line: self.line, col: self.col }),
            "true" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::True, lexeme: text.to_string(), line: self.line, col: self.col }),
            "var" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Var, lexeme: text.to_string(), line: self.line, col: self.col }),
            "while" => return RloxParseResult::FoundToken(Token { rlox_token: RloxToken::While, lexeme: text.to_string(), line: self.line, col: self.col }),
            _ => return RloxParseResult::FoundToken(Token {
                rlox_token: RloxToken::Identifier(text.to_string()),
                lexeme: text.to_string(),
                line: self.line,
                col: self.col,
            })
        }


        
    }

    pub fn scan_token(&mut self) -> RloxParseResult {
        let c = self.advance();
        let found_token = match c {
            '(' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::LeftParen, lexeme: "(".to_string(), line: self.line, col: self.col }),
            ')' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::RightParen, lexeme: ")".to_string(), line: self.line, col: self.col }),
            '{' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::LeftBrace, lexeme: "{".to_string(), line: self.line, col: self.col }),
            '}' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::RightBrace, lexeme: "}".to_string(), line: self.line, col: self.col }),
            ',' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Comma, lexeme: ",".to_string(), line: self.line, col: self.col }),
            '.' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Dot, lexeme: ".".to_string(), line: self.line, col: self.col }),
            '-' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Minus, lexeme: "-".to_string(), line: self.line, col: self.col }),
            '+' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Plus, lexeme: "+".to_string(), line: self.line, col: self.col }),
            ';' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Semicolon, lexeme: ";".to_string(), line: self.line, col: self.col }),
            '*' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Star, lexeme: "*".to_string(), line: self.line, col: self.col }),
            '!' => {
                if self.match_next('=') {
                    RloxParseResult::FoundToken(Token { rlox_token: RloxToken::BangEqual, lexeme: "!=".to_string(), line: self.line, col: self.col })
                } else {
                    RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Bang, lexeme: "!".to_string(), line: self.line, col: self.col })
                }
            }
            '=' => {
                if self.match_next('=') {
                    RloxParseResult::FoundToken( Token { rlox_token: RloxToken::EqualEqual, lexeme: "==".to_string(), line: self.line, col: self.col })  
                } else {
                    RloxParseResult::FoundToken( Token { rlox_token: RloxToken::Equal, lexeme: "=".to_string(), line: self.line, col: self.col })
                }
            }
            '<' => {
                if self.match_next('=') {
                    RloxParseResult::FoundToken( Token { rlox_token: RloxToken::LessEqual, lexeme: "<=".to_string(), line: self.line, col: self.col })
                } else {
                    RloxParseResult::FoundToken( Token { rlox_token: RloxToken::Less, lexeme: "<".to_string(), line: self.line, col: self.col })
                }
            }
            '>' => {
                if self.match_next('=') {
                    RloxParseResult::FoundToken(Token { rlox_token: RloxToken::GreaterEqual, lexeme: ">=".to_string(), line: self.line, col: self.col })
                } else {
                    RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Greater, lexeme: ">".to_string(), line: self.line, col: self.col })
                }
            }
            '/' => {
                if self.match_next('/') {
                    // This is a comment, skip until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    RloxParseResult::Ignored
                } else {
                    RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Slash, lexeme: "/".to_string(), line: self.line, col: self.col })
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
               RloxParseResult::Ignored
            }
            '\n' => {
                self.line += 1;
                self.col = 0;
                RloxParseResult::Ignored
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
                            RloxParseResult::FoundToken(Token {
                                rlox_token: RloxToken::Number(value),
                                lexeme: number,
                                line: self.line,
                                col: self.col,
                            })
                        }
                        Err(_) => {
                            RloxParseResult::Error("Invalid number format".to_string())
                        }
                    }
                } else if c.is_alphanumeric() {
                    self.process_identifier()
                }
                else {
                    RloxParseResult::Error(format!("Unexpected character: {}", c))    
                }   
            }               
        };

        return found_token;
        }
        
     
    }

