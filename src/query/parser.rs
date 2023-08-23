use std::fmt::Display;

use super::{
    ast::QueryStatement,
    lex::{Lexer, Token},
};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "unexpected token: {:?}", token),
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse(&mut self) -> Result<Vec<QueryStatement>, ParseError> {
        let mut statements = Vec::new();
        while self.current_token != Token::Eof {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<QueryStatement, ParseError> {
        match self.current_token {
            Token::Select => Ok(self.parse_select_statement()?),
            Token::Set => Ok(self.parse_set_statement()?),
            Token::Exit => Ok(self.parse_exit_statement()?),
            _ => Err(ParseError::UnexpectedToken(self.current_token.clone())),
        }
    }

    fn parse_select_statement(&mut self) -> Result<QueryStatement, ParseError> {
        self.next_token(); // skip select
        let (is_all, columns) = self.parse_select_arg()?;
        Ok(QueryStatement::Select(is_all, columns))
    }

    fn parse_select_arg(&mut self) -> Result<(bool, Vec<String>), ParseError> {
        let mut is_all = false;
        let mut columns = Vec::new();

        if self.current_token == Token::Asterisk {
            is_all = true;
            self.next_token(); // skip *
        } else {
            match &self.current_token {
                Token::Ident(name) => {
                    columns.push(name.clone());
                    self.next_token() // skip name
                }
                _ => return Err(ParseError::UnexpectedToken(self.current_token.clone())),
            }
        }

        while self.current_token == Token::Comma {
            self.next_token(); // skip ,
            if self.current_token == Token::Asterisk {
                is_all = true;
                self.next_token(); // skip *
            } else {
                match &self.current_token {
                    Token::Ident(name) => {
                        columns.push(name.clone());
                        self.next_token() // skip name
                    }
                    _ => return Err(ParseError::UnexpectedToken(self.current_token.clone())),
                }
            }
        }

        Ok((is_all, columns))
    }

    fn parse_set_statement(&mut self) -> Result<QueryStatement, ParseError> {
        let mut assignments = Vec::new();
        self.next_token(); // skip set

        let key = self.parse_ident()?;
        if self.current_token != Token::Equal {
            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
        }
        self.next_token(); // skip =
        let value = self.parse_int()?;
        assignments.push((key, value));

        while self.current_token == Token::Comma {
            self.next_token(); // skip ,
            let key = self.parse_ident()?;
            if self.current_token != Token::Equal {
                return Err(ParseError::UnexpectedToken(self.current_token.clone()));
            }
            self.next_token(); // skip =
            let value = self.parse_int()?;
            assignments.push((key, value));
        }

        Ok(QueryStatement::Set(assignments))
    }

    fn parse_ident(&mut self) -> Result<String, ParseError> {
        match self.current_token.to_owned() {
            Token::Ident(name) => {
                self.next_token(); // skip name
                Ok(name)
            }
            _ => Err(ParseError::UnexpectedToken(self.current_token.clone())),
        }
    }

    fn parse_int(&mut self) -> Result<i32, ParseError> {
        match self.current_token.to_owned() {
            Token::Integer(value) => {
                self.next_token(); // skip value
                Ok(value)
            }
            _ => Err(ParseError::UnexpectedToken(self.current_token.clone())),
        }
    }

    fn parse_exit_statement(&mut self) -> Result<QueryStatement, ParseError> {
        self.next_token(); // skip exit
        Ok(QueryStatement::Exit)
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse(input: String) -> Result<Vec<QueryStatement>, ParseError> {
        let lexer = Lexer::new(input);
        Parser::new(lexer).parse()
    }

    #[test]
    fn test_parse_select_single() {
        let statements = parse(String::from("SELECT foo;")).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Select(false, vec!["foo".to_string(),])
        );
    }

    #[test]
    fn test_parse_select_multi() {
        let statements = parse(String::from("SELECT foo, bar;")).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Select(false, vec!["foo".to_string(), "bar".to_string()])
        );
    }

    #[test]
    fn test_parse_select_all() {
        {
            let statements = parse(String::from("SELECT *;")).unwrap();
            assert_eq!(statements.len(), 1);
            assert_eq!(statements[0], QueryStatement::Select(true, vec![]));
        }
        {
            let statements = parse(String::from("SELECT *, foo;")).unwrap();
            assert_eq!(statements.len(), 1);
            assert_eq!(
                statements[0],
                QueryStatement::Select(true, vec!["foo".to_string()])
            );
        }
        {
            let statements = parse(String::from("SELECT foo, *;")).unwrap();
            assert_eq!(statements.len(), 1);
            assert_eq!(
                statements[0],
                QueryStatement::Select(true, vec!["foo".to_string()])
            );
        }
    }

    #[test]
    fn test_parse_set_single() {
        let statements = parse(String::from("SET foo = 1;")).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Set(vec![("foo".to_string(), 1)])
        );
    }

    #[test]
    fn test_parse_set_multi() {
        let statements = parse(String::from("SET foo = 1, bar = 999;")).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Set(vec![("foo".to_string(), 1), ("bar".to_string(), 999)])
        );
    }

    #[test]
    fn test_parse_error() {
        {
            let err = parse(String::from("SELECT;")).unwrap_err();
            assert_eq!(err, ParseError::UnexpectedToken(Token::SemiColon));
        }
        {
            let err = parse(String::from("SELECT 1;")).unwrap_err();
            assert_eq!(err, ParseError::UnexpectedToken(Token::Integer(1)));
        }
        {
            let err = parse(String::from("SET a;")).unwrap_err();
            assert_eq!(err, ParseError::UnexpectedToken(Token::SemiColon));
        }
        {
            let err = parse(String::from("SET 1;")).unwrap_err();
            assert_eq!(err, ParseError::UnexpectedToken(Token::Integer(1)));
        }
        {
            let err = parse(String::from("SET;")).unwrap_err();
            assert_eq!(err, ParseError::UnexpectedToken(Token::SemiColon));
        }
    }
}
