#[derive(Debug)]
pub struct Query {
    pub body: Vec<QueryStatement>,
}

#[derive(Debug, PartialEq)]
pub enum QueryStatement {
    Set(i32),
    Get,
    Exit,
}
