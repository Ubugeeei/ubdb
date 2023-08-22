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
            Token::Select => Ok(self.parse_select_statement()?),
            Token::Set => Ok(self.parse_set_statement()?),
            Token::Exit => Ok(self.parse_exit_statement()?),
            _ => Err(ParseError::UnexpectedToken(self.current_token.clone())),
        }
    }

    fn parse_select_statement(&mut self) -> Result<QueryStatement, ParseError> {
        self.next_token(); // skip select
        let (is_all, columns) = self.parse_select_arg()?;
        Ok(QueryStatement::Select { is_all, columns })
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
        self.next_token(); // skip set

        todo!();

        // let ret = match self.current_token {
        //     Token::Integer(value) => Ok(QueryStatement::Select(value)),
        //     _ => Err(ParseError::NoArgsSetStatement),
        // };

        // self.next_token(); // skip arg

        // ret
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

    #[test]
    fn test_parse_select_single() {
        let input = String::from("SELECT foo;");
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let statements = parser.parse().unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Select {
                is_all: false,
                columns: vec!["foo".to_string(),]
            }
        );
    }

    #[test]
    fn test_parse_select_multi() {
        let input = String::from("SELECT foo, bar;");
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let statements = parser.parse().unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Select {
                is_all: false,
                columns: vec!["foo".to_string(), "bar".to_string()]
            }
        );
    }

    #[test]
    fn test_parse_select_all() {
        {
            let input = String::from("SELECT *;");
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let statements = parser.parse().unwrap();
            assert_eq!(statements.len(), 1);
            assert_eq!(
                statements[0],
                QueryStatement::Select {
                    is_all: true,
                    columns: vec![]
                }
            );
        }
        {
            let input = String::from("SELECT *, foo;");
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let statements = parser.parse().unwrap();
            assert_eq!(statements.len(), 1);
            assert_eq!(
                statements[0],
                QueryStatement::Select {
                    is_all: true,
                    columns: vec!["foo".to_string()]
                }
            );
        }
        {
            let input = String::from("SELECT foo, *;");
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let statements = parser.parse().unwrap();
            assert_eq!(statements.len(), 1);
            assert_eq!(
                statements[0],
                QueryStatement::Select {
                    is_all: true,
                    columns: vec!["foo".to_string()]
                }
            );
        }
    }

    // #[test]
    // fn test_parse_set_single() {
    //     let input = String::from("SET foo = 1;");
    //     let lexer = Lexer::new(input);
    //     let mut parser = Parser::new(lexer);
    //     let statements = parser.parse().unwrap();
    //     assert_eq!(statements.len(), 1);
    //     assert_eq!(
    //         statements[0],
    //         QueryStatement::Select {
    //             is_all: false,
    //             columns: vec![]
    //         }
    //     );
    // }

    // #[test]
    // fn test_parse_set_multi() {
    //     let input = String::from("SET foo = 1 , bar = 999;");
    //     let lexer = Lexer::new(input);
    //     let mut parser = Parser::new(lexer);
    //     let statements = parser.parse().unwrap();
    //     assert_eq!(statements.len(), 1);
    //     assert_eq!(
    //         statements[0],
    //         QueryStatement::Select {
    //             is_all: false,
    //             columns: vec![]
    //         }
    //     );
    // }

    // #[test]
    // fn test_parse_error() {
    //     // UnexpectedToken
    //     {
    //         let input = String::from("SET 1;");
    //         let lexer = Lexer::new(input);
    //         let mut parser = Parser::new(lexer);
    //         let err = parser.parse().unwrap_err();
    //         assert_eq!(
    //             err,
    //             ParseError::UnexpectedToken(Token::Ident("SET".to_string()))
    //         );
    //     }

    //     // NoArgsSetStatement
    //     {
    //         let input = String::from("SET; GET; exit;");
    //         let lexer = Lexer::new(input);
    //         let mut parser = Parser::new(lexer);
    //         let err = parser.parse().unwrap_err();
    //         assert_eq!(err, ParseError::NoArgsSetStatement);
    //     }
    // }
}
