#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Select,
    Set,
    Exit,
    Integer(i32),
    Ident(String),
    Equal,
    Asterisk,
    Comma,
    SemiColon,
    Illegal,
    Eof,
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '\u{0}' => Token::Eof,
            '=' => Token::Equal,
            '*' => Token::Asterisk,
            ',' => Token::Comma,
            ';' => Token::SemiColon,
            '0'..='9' => Token::Integer(self.read_number()),
            _ => Self::word_to_token(&self.read_word()),
        };
        self.read_char();
        token
    }

    fn word_to_token(word: &str) -> Token {
        match word {
            "SELECT" | "select" => Token::Select,
            "SET" | "set" => Token::Set,
            "exit" => Token::Exit,
            _ => Token::Ident(word.to_string()),
        }
    }

    fn read_word(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        self.read_position -= 1;
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> i32 {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.read_position -= 1;
        self.input[position..self.position].parse().unwrap()
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_lexer() {
        use super::{Lexer, Token};
        let input = String::from("SET some_key = 1; SELECT some_key; SELECT *; exit;");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Token::Set);
        assert_eq!(lexer.next(), Token::Ident(String::from("some_key")));
        assert_eq!(lexer.next(), Token::Equal);
        assert_eq!(lexer.next(), Token::Integer(1));
        assert_eq!(lexer.next(), Token::SemiColon);
        assert_eq!(lexer.next(), Token::Select);
        assert_eq!(lexer.next(), Token::Ident(String::from("some_key")));
        assert_eq!(lexer.next(), Token::SemiColon);
        assert_eq!(lexer.next(), Token::Select);
        assert_eq!(lexer.next(), Token::Asterisk);
        assert_eq!(lexer.next(), Token::SemiColon);
        assert_eq!(lexer.next(), Token::Exit);
        assert_eq!(lexer.next(), Token::SemiColon);
        assert_eq!(lexer.next(), Token::Eof);
    }
}
