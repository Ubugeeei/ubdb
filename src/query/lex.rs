#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    // keywords
    Select,
    From,
    Update,
    Set,
    Exit,
    Create,
    Table,
    Int,
    VarChar,

    // values
    Integer(i32),
    Ident(String),
    String(String),

    // symbols
    Equal,
    Asterisk,
    Comma,
    SemiColon,
    LParen,
    RParen,

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
            '(' => Token::LParen,
            ')' => Token::RParen,
            '0'..='9' => self.read_number(),
            '\'' => self.read_string(),
            _ => Self::word_to_token(&self.read_word()),
        };
        self.read_char();
        token
    }

    fn word_to_token(word: &str) -> Token {
        match word {
            "SELECT" | "select" => Token::Select,
            "FROM" | "from" => Token::From,
            "UPDATE" | "update" => Token::Update,
            "SET" | "set" => Token::Set,
            "CREATE" | "create" => Token::Create,
            "TABLE" | "table" => Token::Table,
            "INT" | "int" => Token::Int,
            "VARCHAR" | "varchar" => Token::VarChar,
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

    fn read_number(&mut self) -> Token {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.read_position -= 1;
        Token::Integer(self.input[position..self.position].parse().unwrap())
    }

    fn read_string(&mut self) -> Token {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '\'' {
                break;
            }
        }
        Token::String(self.input[position..self.position].to_string())
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
        let input = String::from("UPDATE user SET name = 'mike'; SELECT name FROM user; SELECT * FROM user; exit; CREATE TABLE user (id INT, name VARCHAR);");
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next(), Token::Update);
        assert_eq!(lexer.next(), Token::Ident(String::from("user")));
        assert_eq!(lexer.next(), Token::Set);
        assert_eq!(lexer.next(), Token::Ident(String::from("name")));
        assert_eq!(lexer.next(), Token::Equal);
        assert_eq!(lexer.next(), Token::String(String::from("mike")));
        assert_eq!(lexer.next(), Token::SemiColon);

        assert_eq!(lexer.next(), Token::Select);
        assert_eq!(lexer.next(), Token::Ident(String::from("name")));
        assert_eq!(lexer.next(), Token::From);
        assert_eq!(lexer.next(), Token::Ident(String::from("user")));
        assert_eq!(lexer.next(), Token::SemiColon);

        assert_eq!(lexer.next(), Token::Select);
        assert_eq!(lexer.next(), Token::Asterisk);
        assert_eq!(lexer.next(), Token::From);
        assert_eq!(lexer.next(), Token::Ident(String::from("user")));
        assert_eq!(lexer.next(), Token::SemiColon);

        assert_eq!(lexer.next(), Token::Exit);
        assert_eq!(lexer.next(), Token::SemiColon);

        assert_eq!(lexer.next(), Token::Create);
        assert_eq!(lexer.next(), Token::Table);
        assert_eq!(lexer.next(), Token::Ident(String::from("user")));
        assert_eq!(lexer.next(), Token::LParen);
        assert_eq!(lexer.next(), Token::Ident(String::from("id")));
        assert_eq!(lexer.next(), Token::Int);
        assert_eq!(lexer.next(), Token::Comma);
        assert_eq!(lexer.next(), Token::Ident(String::from("name")));
        assert_eq!(lexer.next(), Token::VarChar);
        assert_eq!(lexer.next(), Token::RParen);
        assert_eq!(lexer.next(), Token::SemiColon);

        assert_eq!(lexer.next(), Token::Eof);
    }
}
