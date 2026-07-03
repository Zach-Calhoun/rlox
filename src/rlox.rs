use core::num;

use crate::rlox;

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
    pub fn new(source: String) -> Self {
        RloxScanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            col: 1,
        }
    }

    fn scan_tokens(&mut self, raise_error: fn(usize, usize, &str)) {
        while !self.isAtEnd() {
            let token = self.scan_token();
            match token {
                RloxParseResult::Ignored => {},
                RloxParseResult::FoundToken(token) => self.tokens.push(token),
                RloxParseResult::Error(e) => {
                    // Handle error, e.g., log it or raise an error
                    raise_error(self.line, self.col, &e);
                }
            } 
        }
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        self.col += 1;
        c
    }

    fn addToken(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.isAtEnd() {
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
        if self.isAtEnd() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn process_string_literal(&mut self) -> Result<Token, String> {
        let mut string_value = String::new();
        while !self.isAtEnd() && self.peek() != '"' {
            string_value.push(self.advance());
        }
        if self.isAtEnd() {
            // Handle unterminated string error
            return Err("Unterminated string literal".to_string());
        }
        // Consume the closing "
        self.advance();
        let found_token = Token {
            rlox_token: RloxToken::String(string_value.clone()),
            lexeme: string_value,
            line: self.line,
            col: self.col,
        };
        self.addToken(found_token.clone());
        Ok(found_token)
    }

    pub fn scan_token(&mut self) {
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
            '/' => RloxParseResult::FoundToken(Token { rlox_token: RloxToken::Slash, lexeme: "/".to_string(), line: self.line, col: self.col }),
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
                    while self.peek() != '\n' && !self.isAtEnd() {
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
                    while !self.isAtEnd() && self.peek().is_digit(10) {
                        number.push(self.advance());
                    }
                    if !self.isAtEnd() && self.peek() == '.' {
                        number.push(self.advance());
                        while !self.isAtEnd() && self.peek().is_digit(10) {
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
                            RloxParseResult::Err("Invalid number format".to_string());
                        }
                    }
                } else {
                    RloxParseResult::Err(format!("Unexpected character: {}", c));    
                }   
            }               
        };

        return found_token;
        }
    }
}
