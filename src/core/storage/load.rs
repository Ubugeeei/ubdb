use crate::core::table::{DataType, Record, Table, Value};

use super::{DataTypeByteMap, Storage};

impl Storage {
    pub fn load(&self, table_name: &str) -> Option<Table> {
        let path = self.get_table_storage_path(table_name);
        let bytes = std::fs::read(path).unwrap_or(vec![]);
        if bytes.is_empty() {
            return None;
        }
        Some(Self::bytes_to_table(&bytes)) // TODO: error handling
    }

    fn bytes_to_table(bytes: &[u8]) -> Table {
        let mut offset = 0;

        // name
        let name_len = u8::from_be_bytes([bytes[offset]]);
        offset += 1;
        let name = String::from_utf8(bytes[offset..offset + name_len as usize].to_vec()).unwrap();
        offset += name_len as usize;

        // columns
        let columns_len = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;
        let mut columns = vec![];
        for _ in 0..columns_len {
            let column_name_len = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
            offset += 2;
            let column_name =
                String::from_utf8(bytes[offset..offset + column_name_len as usize].to_vec())
                    .unwrap();
            offset += column_name_len as usize;
            let (data_type, data_type_size) = Self::bytes_to_data_type(&bytes[offset..]);
            offset += data_type_size;
            columns.push((column_name, data_type));
        }

        // records
        let records_len = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;
        let mut records = vec![];
        for _ in 0..records_len {
            let mut values = vec![];
            for column in columns.iter() {
                let value = Self::bytes_to_value(&bytes[offset..], &column.1);
                offset += value.1;
                values.push(value.0);
            }
            records.push(Record::new(values));
        }

        Table::new(name, columns, records)
    }

    fn bytes_to_data_type(bytes: &[u8]) -> (DataType, usize) {
        match bytes[0] {
            DataTypeByteMap::INT => (DataType::Int, 1),
            DataTypeByteMap::VARCHAR => {
                let size = u16::from_be_bytes([bytes[1], bytes[2]]);
                (DataType::VarChar(size), 3)
            }
            _ => panic!("invalid data type"),
        }
    }

    fn bytes_to_value(bytes: &[u8], data_type: &DataType) -> (Value, usize) {
        match data_type {
            DataType::Int => (
                Value::Int(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
                4,
            ),
            DataType::VarChar(_size) => {
                let value_len = u16::from_be_bytes([bytes[0], bytes[1]]);
                let value = String::from_utf8(bytes[2..2 + value_len as usize].to_vec()).unwrap();
                (Value::VarChar(value), 2 + value_len as usize)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bytes_to_value() {
        assert_eq!(
            Storage::bytes_to_value(&[0x000, 0x000, 0x000, 0x01], &DataType::Int),
            (Value::Int(1), 4)
        );
        assert_eq!(
            Storage::bytes_to_value(&[0x00, 0x01, 0x61], &DataType::VarChar(8)),
            (Value::VarChar(String::from("a")), 3)
        );
    }

    #[test]
    fn test_bytes_to_data_type() {
        assert_eq!(
            Storage::bytes_to_data_type(&[DataTypeByteMap::INT]),
            (DataType::Int, 1)
        );
        assert_eq!(
            Storage::bytes_to_data_type(&[DataTypeByteMap::VARCHAR, 0x00, 0x01]),
            (DataType::VarChar(1), 3)
        );
    }

    #[test]
    fn test_load() {
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
            Storage::bytes_to_table(&[
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
                0x00, 0x05, // alice length
                0x61, 0x6c, 0x69, 0x63, 0x65, // alice
                0x00, 0x00, 0x00, 0x02, // 2
                0x00, 0x03, // bob length
                0x62, 0x6f, 0x62, // bob
            ]),
            user_table
        );
    }
}
