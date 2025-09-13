mod parser;
mod tokenizer;
use tokenizer::{tokens_from_stream, TokenTypes};
fn main() {
    let tokens = tokens_from_stream(&[123, 1]);
    println!("Hello, world!");
    println!("{:?}", tokens)
}
