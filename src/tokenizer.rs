#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TokenTypes {
    OpenCurlyBrackets,
    ClosedCurlyBrackets,
    Colon,
    Number,
    String,
    False,
    True,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    token_type: TokenTypes,
    value: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tokens {
    token_stream: Vec<Token>,
}

pub fn tokens_from_stream(stream: &[u8]) -> Result<Tokens, &'static str> {
    Err("not implemented yet")
}
pub struct Tokenizer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input: input,
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn next_token(&mut self) -> Result<Token, &'static str> {
        match self.peek() {
            Some(b'{') => self.consume_single(TokenTypes::OpenCurlyBrackets),
            Some(b'}') => self.consume_single(TokenTypes::ClosedCurlyBrackets),
            Some(b':') => self.consume_single(TokenTypes::Colon),
            Some(b'f') | Some(b'F') => self.consume_single(TokenTypes::False),
            Some(b't') | Some(b'T') => self.consume_single(TokenTypes::True),
            Some(b'"') => self.consume_string(),
            Some(b'0'..=b'9') => self.consume_number(),
            _ => return Err("Invalid Token"),
        }
    }

    fn consume_single(&mut self, token_type: TokenTypes) -> Result<Token, &'static str> {
        let value = vec![self.peek().unwrap()];
        self.advance();
        Ok(Token {
            token_type: token_type,
            value: value,
        })
    }

    fn consume_string(&mut self) -> Result<Token, &'static str> {
        self.advance();

        let mut value = Vec::new();
        while let Some(b) = self.peek() {
            match b {
                b'"' => break,
                _ => {
                    if self.pos == self.input.len() - 1 {
                        return Err("Invalid string token");
                    }
                    value.push(b);
                }
            }

            self.advance();
        }

        Ok(Token {
            token_type: TokenTypes::String,
            value: value,
        })
    }

    fn consume_number(&mut self) -> Result<Token, &'static str> {
        let mut value = Vec::new();
        while let Some(b) = self.peek() {
            match b {
                b'0'..=b'9' => value.push(b),
                _ => break,
            }
            self.advance();
        }

        Ok(Token {
            token_type: TokenTypes::Number,
            value: value,
        })
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        Some(self.next_token())
    }
}

mod tokenizer_tests {
    use super::*;
    #[test]
    fn tokens_from_stream_valid_curly_brackets_tokenizer() {
        let stream: &[u8] = &[b'{', b'}'];
        let mut tokenizer_struct = Tokenizer::new(stream);

        assert_eq!(
            tokenizer_struct.next(),
            Some(Ok(Token {
                token_type: TokenTypes::OpenCurlyBrackets,
                value: vec![b'{']
            }))
        );

        assert_eq!(
            tokenizer_struct.next(),
            Some(Ok(Token {
                token_type: TokenTypes::ClosedCurlyBrackets,
                value: vec![b'}']
            }))
        );
    }

    #[test]
    fn tokens_from_stream_invalid_token() {
        let stream: &[u8] = &[b' '];
        let mut tokenizer_struct = Tokenizer::new(stream);

        assert_eq!(tokenizer_struct.next(), Some(Err("Invalid Token")));
    }

    #[test]
    fn tokens_from_stream_valid_string_token() {
        let stream: &[u8] = &[b'"', b'a', b'b', b'"'];
        let mut tokenizer_struct = Tokenizer::new(stream);

        assert_eq!(
            tokenizer_struct.next(),
            Some(Ok(Token {
                token_type: TokenTypes::String,
                value: vec![b'a', b'b']
            }))
        );
    }

    #[test]
    fn tokens_from_stream_invalid_string_token() {
        let stream: &[u8] = &[b'"', b'a', b'b'];
        let mut tokenizer_struct = Tokenizer::new(stream);

        assert_eq!(tokenizer_struct.next(), Some(Err("Invalid string token")));
    }

    #[test]
    fn tokens_from_stream_valid_number_token() {
        let stream: &[u8] = &[b'1', b'2', b'3', b'4'];
        let mut tokenizer_struct = Tokenizer::new(stream);

        assert_eq!(
            tokenizer_struct.next(),
            Some(Ok(Token {
                token_type: TokenTypes::Number,
                value: vec![b'1', b'2', b'3', b'4']
            }))
        );
    }
    #[test]
    fn tokens_from_stream_valid_bool_token() {
        let stream: &[u8] = &[b't', b'T', b'f', b'F'];
        let mut tokenizer_struct = Tokenizer::new(stream);

        assert_eq!(
            tokenizer_struct.next(),
            Some(Ok(Token {
                token_type: TokenTypes::True,
                value: vec![b't']
            }))
        );
        assert_eq!(
            tokenizer_struct.next(),
            Some(Ok(Token {
                token_type: TokenTypes::True,
                value: vec![b'T']
            }))
        );
        assert_eq!(
            tokenizer_struct.next(),
            Some(Ok(Token {
                token_type: TokenTypes::False,
                value: vec![b'f']
            }))
        );
        assert_eq!(
            tokenizer_struct.next(),
            Some(Ok(Token {
                token_type: TokenTypes::False,
                value: vec![b'F']
            }))
        );
    }
}
