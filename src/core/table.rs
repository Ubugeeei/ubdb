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

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut b = vec![];

        // name
        b.extend_from_slice(&(self.name.len() as u8).to_be_bytes());
        b.extend_from_slice(self.name.as_bytes());

        // columns
        b.extend_from_slice(&(self.columns.len() as u16).to_be_bytes());
        for (column_name, data_type) in self.columns.iter() {
            b.extend_from_slice(&(column_name.len() as u16).to_be_bytes());
            b.extend_from_slice(column_name.as_bytes());
            b.extend_from_slice(&data_type.as_bytes());
        }

        // records
        b.extend_from_slice(&(self.records.len() as u16).to_be_bytes());
        for record in self.records.iter() {
            // b.extend_from_slice(&(record.values.len() as u16).to_be_bytes());
            for (idx, value) in record.values.iter().enumerate() {
                b.extend_from_slice(&value.as_bytes(&self.columns[idx].1));
            }
        }
        b
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

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            DataType::Int => vec![DataTypeByteMap::INT],
            DataType::VarChar(size) => {
                let mut b = vec![DataTypeByteMap::VARCHAR];
                b.extend_from_slice(&size.to_be_bytes());
                b
            }
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod DataTypeByteMap {
    pub const INT: u8 = 0;
    pub const VARCHAR: u8 = 10;
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

impl Value {
    pub fn as_bytes(&self, data_type: &DataType) -> Vec<u8> {
        match self {
            Value::Int(value) => value.to_be_bytes().to_vec(),
            Value::VarChar(value) => match data_type {
                DataType::VarChar(size) => {
                    let mut b = vec![];
                    b.extend_from_slice(value.as_bytes());
                    b.resize(*size as usize, 0);
                    b
                }
                _ => panic!("data type mismatch"),
            },
        }
    }
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

    #[test]
    fn test_data_type_as_bytes() {
        assert_eq!(DataType::Int.as_bytes(), vec![DataTypeByteMap::INT]);
        assert_eq!(
            DataType::VarChar(00).as_bytes(),
            vec![DataTypeByteMap::VARCHAR, 0x00, 0x00]
        );
        assert_eq!(
            DataType::VarChar(10).as_bytes(),
            vec![DataTypeByteMap::VARCHAR, 0x00, 0x0a]
        );
        assert_eq!(
            DataType::VarChar(65535).as_bytes(),
            vec![DataTypeByteMap::VARCHAR, 0xff, 0xff]
        );
    }

    #[test]
    fn test_value_as_bytes() {
        assert_eq!(
            Value::Int(1).as_bytes(&DataType::Int),
            vec![0x00, 0x00, 0x00, 0x01]
        );

        assert_eq!(
            Value::VarChar(String::from("hello")).as_bytes(&DataType::VarChar(10)),
            vec![0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn test_table_as_bytes() {
        let mut user_table = Table::new(
            String::from("user"),
            vec![
                (String::from("id"), DataType::Int),
                (String::from("name"), DataType::VarChar(10)),
            ],
            vec![],
        );

        user_table.insert(Record::new(vec![
            Value::Int(1),
            Value::VarChar(String::from("alice")),
        ]));

        user_table.insert(Record::new(vec![
            Value::Int(2),
            Value::VarChar(String::from("bob")),
        ]));

        assert_eq!(
            Table::new(
                String::from("user"),
                vec![
                    (String::from("id"), DataType::Int),
                    (String::from("name"), DataType::VarChar(10)),
                ],
                vec![
                    Record::new(vec![Value::Int(1), Value::VarChar(String::from("alice")),]),
                    Record::new(vec![Value::Int(2), Value::VarChar(String::from("bob")),]),
                ],
            )
            .as_bytes(),
            vec![
                0x04, // name length
                0x75, 0x73, 0x65, 0x72, // user
                0x00, 0x02, // 2 columns
                0x00, 0x02, // column name length
                0x69, 0x64, // id
                0x00, // int
                0x00, 0x04, // column name length
                0x6e, 0x61, 0x6d, 0x65, // name
                0x0a, 0x00, 0x0a, // varchar(10)
                0x00, 0x02, // records length
                0x00, 0x00, 0x00, 0x01, // 1
                0x61, 0x6c, 0x69, 0x63, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, // alice
                0x00, 0x00, 0x00, 0x02, // 2
                0x62, 0x6f, 0x62, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bob
            ]
        )
    }
}
