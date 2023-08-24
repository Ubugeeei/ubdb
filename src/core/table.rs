#[derive(Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<(String, DataType)>,
    pub records: Vec<Record>,
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

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(i32),
    VarChar(String),
}

#[cfg(test)]
mod test {
    #[test]
    fn test_data_type_size() {
        use super::DataType;
        assert_eq!(DataType::Int.size(), 4);
        assert_eq!(DataType::VarChar(10).size(), 10);
        assert_eq!(DataType::VarChar(65535).size(), 65535);
    }

    #[test]
    fn test_data_type_as_bytes() {
        use super::{DataType, DataTypeByteMap};
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
}
