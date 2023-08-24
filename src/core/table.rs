#[derive(Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<(String, DataType)>,
    pub records: Vec<Record>,
}
impl Table {
    pub fn new(name: String, columns: Vec<(String, DataType)>, records: Vec<Record>) -> Self {
        // TODO: validation
        Self {
            name,
            columns,
            records,
        }
    }

    pub fn insert(&mut self, record: Record) {
        self.records.push(record);
    }
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    VarChar(u16),
}
impl DataType {
    pub fn size(&self) -> usize {
        match self {
            DataType::Int => 4,
            DataType::VarChar(size) => *size as usize,
        }
    }
}



#[derive(Debug, PartialEq)]
pub struct Record {
    pub values: Vec<Value>,
}
impl Record {
    pub fn new(values: Vec<Value>) -> Self {
        Self { values }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(i32),
    VarChar(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_data_type_size() {
        assert_eq!(DataType::Int.size(), 4);
        assert_eq!(DataType::VarChar(10).size(), 10);
        assert_eq!(DataType::VarChar(65535).size(), 65535);
    }

    
}
