mod parser;
mod tokenizer;
mod ast;
use tokenizer::{ Tokenizer};

fn main() {
    let intput: &[u8] = &[b'{', b'}', b':', b'f', b'F', b't', b'T', b'"', b'a', b'b', b'c',b'"', b'1', b'2'];
    let mut token_stream=Tokenizer::new(intput);
    while let Some(token_result)= token_stream.next(){
        match token_result {
            Ok(token)=> println!("{:?}", token),
            Err(msg)=> println!("msg: {:?}",msg)
        }
    }
}
