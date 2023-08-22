#[derive(Debug)]
pub struct Query(Vec<QueryStatement>);

#[derive(Debug, PartialEq)]
pub enum QueryStatement {
    Select { is_all: bool, columns: Vec<String> },
    Set(Vec<Assignment>),
    Exit,
}

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub column: String,
    pub value: String,
}
