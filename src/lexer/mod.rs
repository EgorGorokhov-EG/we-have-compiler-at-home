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

    fn get_token(&mut self) -> Token {
        let token = match self.state.current_char {
            token_text @ '\n' => Token::Newline(String::from(token_text)),
            token_text @ '+' => Token::Plus(String::from(token_text)),
            token_text @ '-' => Token::Minus(String::from(token_text)),
            token_text @ '*' => Token::Asterisk(String::from(token_text)),
            token_text @ '=' => Token::EQ(String::from(token_text)),
            token_text if token_text.is_numeric() => get_number_token(self),
            token_text => Token::InvalidToken(String::from(token_text))
        };

        // Tokenize the whole number
        // Modifies current state of the lexer by moving to the next char while the next char is a numeric
        // TODO: Make it work for floating point numbers
        fn get_number_token(lexer: &mut Lexer) -> Token {
            let mut number_string = String::from("");
            number_string.push(lexer.state.current_char);

            loop {
                match lexer.peek_next_char() {
                    Some(next_char) if next_char.is_numeric() => {
                        lexer.move_to_next_char();
                        number_string.push(lexer.state.current_char);
                    }
                    _ => return Token::Number(number_string)
                }
            }
        }

        // After all tokenizations we stay on the last char of the parsed token,
        // we need to call this method after the tokenization to move to the next char(to parse next token)
        self.move_to_next_char();

        return token
    }
}

#[derive(Debug)]
struct LexerState {
    pub current_position: usize,
    pub current_char: char,
}
