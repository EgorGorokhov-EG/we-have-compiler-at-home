use core::panic;

use crate::types::Token;

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    is_finished: bool,
    state: LexerState,
}

impl Lexer {
    pub fn create(source: Vec<char>) -> Lexer {

        let (is_finished, state) = 
            match source.get(0) {
                Some(first_char) => 
                    (false, LexerState { current_position: 0, current_char: *first_char }),
                None => 
                    (true, LexerState { current_position: 0, current_char: '\0' })
            };
        

        Lexer {
            source,
            is_finished,
            state,
        }
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        loop {
            match self.is_finished {
                false => result.push(self.get_token()),
                true => return result
            }
        }
    }

    // Updates the current state with { current_position+1, next peeked char } if this next char is present
    // and set lexing_stage to End if we've reached end of the file(no next char)
    fn move_to_next_char(&mut self) {
        match self.peek_next_char() {
            Some(next_char) => {
                let next_position = self.state.current_position + 1;
                self.state = LexerState { current_position: next_position, current_char: *next_char };
            }
            None => self.is_finished = true
        }
    }

    // Used to peek what is the next char and check if it's the end of the source
    fn peek_next_char(&self) -> Option<&char> {
        self.source.get(self.state.current_position + 1)
    }

    // Skip all whitespace characters except a new_line
    fn move_to_next_non_whitespace_char(&mut self) {
        // Also check if we've reached the end of the file
        while " \t\r".contains(self.state.current_char) && !self.is_finished {
            self.move_to_next_char();
        }
    }

    fn get_token(&mut self) -> Token {
        // Tries to get a number token, panics if can't tokenize it as a number 
        // Modifies current state of the lexer by moving to the next char
        fn get_number_token(lexer: &mut Lexer) -> Token {
            let mut number_string = String::from(lexer.state.current_char);

            loop {
                match lexer.peek_next_char() {
                    Some(next_char) if next_char.is_numeric() => {
                        lexer.move_to_next_char();
                        number_string.push(lexer.state.current_char);
                    }
                    _ => return Token::Number(number_string),
                }
            }
        }

        // Tries to get a keyword token, panics if can't tokenize it as a keyword
        // Modifies current state of the lexer by moving to the next char
        fn get_keyword_token(lexer: &mut Lexer) -> Token {

            fn keyword_token_from_string(string: &str) -> Option<Token> {
                match string {
                    "LET" => Some(Token::Let(string.to_string())),
                    _ => None
                }
            }

            let mut keyword_string = String::from(lexer.state.current_char);

            loop {
                match lexer.peek_next_char() {
                    Some(next_char) if next_char.is_alphabetic() => {
                        lexer.move_to_next_char();
                        keyword_string.push(lexer.state.current_char);
                    }
                    _ => match keyword_token_from_string(keyword_string.as_str()) {
                        Some(keyword_token) => return keyword_token,
                        None => panic!("Invalid KEYWORD {} !", keyword_string)
                    }
                }                
            }
        }

        let token = match self.state.current_char {
            '\n' => Token::Newline(String::from(self.state.current_char)),
            '+' => Token::Plus(String::from(self.state.current_char)),
            '-' => Token::Minus(String::from(self.state.current_char)),
            '*' => Token::Asterisk(String::from(self.state.current_char)),
            '=' => Token::EQ(String::from(self.state.current_char)),
            c if c.is_numeric() => get_number_token(self),
            // If it's a sequence of alphabetic chars, it should be a keyword
            c if c.is_alphabetic() && self.peek_next_char().filter(|&c| c.is_alphabetic()).is_some() => get_keyword_token(self),
            // If it's a single char, it should be an ident(var)
            c if c.is_alphabetic() => Token::Var(String::from(c)),
            _ => Token::InvalidToken(String::from(self.state.current_char))
        };

        // After all tokenizations we stay on the last char of the parsed token,
        // we need to call this method after the tokenization to move to the next char(to parse next token)
        self.move_to_next_char();
        self.move_to_next_non_whitespace_char();

        return token
    }
}

#[derive(Debug)]
struct LexerState {
    pub current_position: usize,
    pub current_char: char,
}
