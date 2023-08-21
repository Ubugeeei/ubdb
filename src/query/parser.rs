use std::fmt::Display;

use super::{
    ast::QueryStatement,
    lex::{Lexer, Token},
};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    NoArgsSetStatement,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "unexpected token: {:?}", token),
            ParseError::NoArgsSetStatement => write!(f, "no args set statement"),
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
            Token::Get => Ok(self.parse_get_statement()?),
            Token::Set => Ok(self.parse_set_statement()?),
            Token::Exit => Ok(self.parse_exit_statement()?),
            _ => Err(ParseError::UnexpectedToken(self.current_token.clone())),
        }
    }

    fn parse_get_statement(&mut self) -> Result<QueryStatement, ParseError> {
        self.next_token(); // slip get
        Ok(QueryStatement::Get)
    }

    fn parse_set_statement(&mut self) -> Result<QueryStatement, ParseError> {
        self.next_token(); // slip set

        let ret = match self.current_token {
            Token::Integer(value) => Ok(QueryStatement::Set(value)),
            _ => Err(ParseError::NoArgsSetStatement),
        };

        self.next_token(); // slip arg

        ret
    }

    fn parse_exit_statement(&mut self) -> Result<QueryStatement, ParseError> {
        self.next_token(); // slip exit
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

    #[test]
    fn test_parse() {
        let input = String::from("SET 1; GET; exit;");
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let statements = parser.parse().unwrap();
        assert_eq!(
            statements,
            vec![
                QueryStatement::Set(1),
                QueryStatement::Get,
                QueryStatement::Exit,
            ]
        );
    }

    #[test]
    fn test_parse_error() {
        // UnexpectedToken
        {
            let input = String::from("SET 1; GET; exit; aaa;");
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let err = parser.parse().unwrap_err();
            assert_eq!(
                err,
                ParseError::UnexpectedToken(Token::Ident("aaa".to_string()))
            );
        }

        // NoArgsSetStatement
        {
            let input = String::from("SET; GET; exit;");
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let err = parser.parse().unwrap_err();
            assert_eq!(err, ParseError::NoArgsSetStatement);
        }
    }
}
