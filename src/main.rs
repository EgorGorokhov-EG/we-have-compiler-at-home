mod lexer;
mod types;

use lexer::Lexer;
use types::Token;

fn main() {
    let source: Vec<char> = String::from("100 + 1 = 0 + 2 * 1\n").chars().collect();
    let mut l = Lexer::create(source);

    let result: Vec<Token> = l.run();
    
    print!("{:?}", result);
}
