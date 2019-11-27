use std::fs;

// const delims_all: [char; 36] = [
//     '!', '`', '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '-', '+', '=', '{', '}',
//     '[', ']', '|', '\\', ':', ';', '\'', '"', '<', '>', ',', '.', '?', '/', '\t', ' ', '\n',
// ];

const DELIMS: [char; 7] = ['\t', ' ', '\n', '"', '\'', '(', ')'];

#[derive(Debug)]
pub enum TokenType {
    LParen,
    RParen,
    Whitespace(char),
    NumberInteger(i64),
    NumberFloat(f64),
    Boolean(bool),
    Ident(String),
}

pub struct Token {
    token: TokenType,
    position: Position,
}

#[derive(Copy, Clone)]
pub struct Position {
    offset: i64,
    line: i64,
    column: i64,
}

pub struct Tokenizer {
    filename: String,
    source: String,
    tokens: Vec<Token>,
    state: Position,
}

impl Tokenizer {
    pub fn from(filename: String) -> Tokenizer {
        Tokenizer {
            filename: filename,
            source: String::from(""),
            tokens: vec![],
            state: Position {
                line: 1,
                column: 0,
                offset: 0,
            },
        }
    }

    fn add_delim(&mut self, delim: char) {
        match delim {
            '(' => self.tokens.push(Token {
                token: TokenType::LParen,
                position: self.state,
            }),
            ')' => self.tokens.push(Token {
                token: TokenType::RParen,
                position: self.state,
            }),
            ' ' | '\n' | '\t' => self.tokens.push(Token {
                token: TokenType::Whitespace(delim),
                position: self.state,
            }),
            _ => println!("Shouldn't have happened"),
        }
    }

    fn add_ident_token(&mut self, token: &String) {
        if token == "" {
            return;
        }
        let int_result = token.parse::<i64>();
        if int_result.is_ok() {
            self.tokens.push(Token {
                token: TokenType::NumberInteger(int_result.unwrap()),
                position: self.state,
            });
            return;
        }

        let float_result = token.parse::<f64>();
        if float_result.is_ok() {
            self.tokens.push(Token {
                token: TokenType::NumberFloat(float_result.unwrap()),
                position: self.state,
            });
            return;
        }

        if token == "true" || token == "false" {
            let result = if token == "true" { true } else { false };
            self.tokens.push(Token {
                token: TokenType::Boolean(result),
                position: self.state,
            });
            return;
        }

        self.tokens.push(Token {
            token: TokenType::Ident(token.to_string()),
            position: self.state,
        });
        return;
    }

    pub fn tokenize(&mut self) {
        let source =
            fs::read_to_string(&self.filename).expect("Something went wrong reading the file");
        let mut temp = String::from("");
        let chars = source.chars();
        for c in chars {
            // update overall iterator position
            self.state.offset += 1;
            // update column position
            self.state.column += 1;
            // change the state if a
            // new line is encountered
            if c == '\n' {
                self.state.line += 1;
                self.state.column = 0;
            }

            if !DELIMS.contains(&c) {
                &temp.push(c);
                continue;
            }

            &self.add_ident_token(&temp);
            temp = "".to_string();
            &self.add_delim(c);
        }
        self.source = source;
        self.tokens
            .iter()
            .for_each(|x| println!("{}:{} {:?}", x.position.line, x.position.column, x.token));
    }
}
