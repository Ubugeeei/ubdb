use std::fmt::Display;

use super::{
    ast::{QueryStatement, Value},
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
            Token::Update => Ok(self.parse_update_statement()?),
            Token::Create => Ok(self.parse_create_table_statement()?),
            Token::Exit => Ok(self.parse_exit_statement()?),
            _ => Err(ParseError::UnexpectedToken(self.current_token.clone())),
        }
    }

    fn parse_select_statement(&mut self) -> Result<QueryStatement, ParseError> {
        self.next_token(); // skip select
        let (table_name, is_all, columns, cond) = self.parse_select_arg()?;
        Ok(QueryStatement::Select(table_name, is_all, columns, cond))
    }

    fn parse_select_arg(
        &mut self,
    ) -> Result<(String, bool, Vec<String>, Option<(String, Value)>), ParseError> {
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

        if self.current_token != Token::From {
            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
        }
        self.next_token(); // skip from

        let table_name = self.parse_ident()?;

        // where
        if self.current_token != Token::Where {
            return Ok((table_name, is_all, columns, None));
        }
        self.next_token(); // skip where
                           // cond
        let key = self.parse_ident()?;
        if self.current_token != Token::Equal {
            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
        }
        self.next_token(); // skip =
        let value = self.parse_value()?;
        let cond = (key, value);

        Ok((table_name, is_all, columns, Some(cond)))
    }

    fn parse_update_statement(&mut self) -> Result<QueryStatement, ParseError> {
        let mut assignments = Vec::new();
        self.next_token(); // skip update

        let table_name = self.parse_ident()?;

        if self.current_token != Token::Set {
            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
        }
        self.next_token(); // skip set

        let key = self.parse_ident()?;
        if self.current_token != Token::Equal {
            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
        }
        self.next_token(); // skip =
        let value = self.parse_value()?;
        assignments.push((key, value));

        while self.current_token == Token::Comma {
            self.next_token(); // skip ,
            let key = self.parse_ident()?;
            if self.current_token != Token::Equal {
                return Err(ParseError::UnexpectedToken(self.current_token.clone()));
            }
            self.next_token(); // skip =
            let value = self.parse_value()?;
            assignments.push((key, value));
        }

        // where
        if self.current_token != Token::Where {
            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
        }
        self.next_token(); // skip where
                           // cond
        let key = self.parse_ident()?;
        if self.current_token != Token::Equal {
            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
        }
        self.next_token(); // skip =
        let value = self.parse_value()?;

        Ok(QueryStatement::Update(
            table_name,
            assignments,
            (key, value),
        ))
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        match self.current_token.to_owned() {
            Token::Integer(value) => {
                self.next_token(); // skip value
                Ok(Value::Int(value))
            }
            Token::String(value) => {
                self.next_token(); // skip value
                Ok(Value::VarChar(value))
            }
            _ => Err(ParseError::UnexpectedToken(self.current_token.clone())),
        }
    }

    fn parse_create_table_statement(&mut self) -> Result<QueryStatement, ParseError> {
        self.next_token(); // skip create
        if self.current_token != Token::Table {
            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
        }
        self.next_token(); // skip table

        let table_name = self.parse_ident()?;

        if self.current_token != Token::LParen {
            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
        }
        self.next_token(); // skip (

        let mut columns = Vec::new();
        loop {
            let column_name = self.parse_ident()?;
            let data_type = self.parse_data_type()?;
            columns.push((column_name, data_type));
            if self.current_token == Token::Comma {
                self.next_token(); // skip ,
            } else {
                break;
            }
        }
        if self.current_token != Token::RParen {
            return Err(ParseError::UnexpectedToken(self.current_token.clone()));
        }
        self.next_token(); // skip )

        Ok(QueryStatement::CreateTable(table_name, columns))
    }

    fn parse_data_type(&mut self) -> Result<super::ast::DataType, ParseError> {
        match self.current_token.to_owned() {
            Token::Int => {
                self.next_token(); // skip int
                Ok(super::ast::DataType::Int)
            }
            Token::VarChar => {
                self.next_token(); // skip varchar
                if self.current_token != Token::LParen {
                    return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                }
                self.next_token(); // skip (
                let length = self.parse_int()?;
                if self.current_token != Token::RParen {
                    return Err(ParseError::UnexpectedToken(self.current_token.clone()));
                }
                self.next_token(); // skip )
                Ok(super::ast::DataType::VarChar(length as u16))
            }
            _ => Err(ParseError::UnexpectedToken(self.current_token.clone())),
        }
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
    use crate::query::ast::DataType;

    use super::*;

    fn parse(input: String) -> Result<Vec<QueryStatement>, ParseError> {
        let lexer = Lexer::new(input);
        Parser::new(lexer).parse()
    }

    #[test]
    fn test_parse_select_single() {
        let statements = parse(String::from("SELECT foo FROM user;")).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Select(String::from("user"), false, vec!["foo".to_string(),], None)
        );
    }

    #[test]
    fn test_parse_select_multi() {
        let statements = parse(String::from("SELECT foo, bar FROM user;")).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Select(
                String::from("user"),
                false,
                vec!["foo".to_string(), "bar".to_string()],
                None
            )
        );
    }

    #[test]
    fn test_parse_select_where() {
        let statements = parse(String::from("SELECT foo, bar FROM users WHERE id = 1;")).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Select(
                String::from("users"),
                false,
                vec!["foo".to_string(), "bar".to_string()],
                Some(("id".to_string(), Value::Int(1)))
            )
        );
    }

    #[test]
    fn test_parse_select_all() {
        {
            let statements = parse(String::from("SELECT * FROM user;")).unwrap();
            assert_eq!(statements.len(), 1);
            assert_eq!(
                statements[0],
                QueryStatement::Select(String::from("user"), true, vec![], None)
            );
        }
        {
            let statements = parse(String::from("SELECT *, foo FROM user;")).unwrap();
            assert_eq!(statements.len(), 1);
            assert_eq!(
                statements[0],
                QueryStatement::Select(String::from("user"), true, vec!["foo".to_string()], None)
            );
        }
        {
            let statements = parse(String::from("SELECT foo, * FROM user;")).unwrap();
            assert_eq!(statements.len(), 1);
            assert_eq!(
                statements[0],
                QueryStatement::Select(String::from("user"), true, vec!["foo".to_string()], None)
            );
        }
    }

    #[test]
    fn test_parse_set_single() {
        let statements =
            parse(String::from("UPDATE user SET name = 'mike' WHERE id = 1;")).unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Update(
                String::from("user"),
                vec![("name".to_string(), Value::VarChar("mike".to_string()))],
                ("id".to_string(), Value::Int(1))
            )
        );
    }

    #[test]
    fn test_parse_set_multi() {
        let statements = parse(String::from(
            "UPDATE user SET foo = 1, bar = 999 WHERE name = 'mike';",
        ))
        .unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::Update(
                String::from("user"),
                vec![
                    ("foo".to_string(), Value::Int(1)),
                    ("bar".to_string(), Value::Int(999))
                ],
                ("name".to_string(), Value::VarChar("mike".to_string()))
            )
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
    }

    #[test]
    fn test_parse_create_table() {
        let statements = parse(String::from(
            "CREATE TABLE users (id INT, name VARCHAR(255));",
        ))
        .unwrap();
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            QueryStatement::CreateTable(
                "users".to_string(),
                vec![
                    ("id".to_string(), DataType::Int),
                    ("name".to_string(), DataType::VarChar(255)),
                ]
            )
        );
    }
}
