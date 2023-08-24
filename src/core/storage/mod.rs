mod flush;
mod load;

pub struct Storage {
    pub storage_dir: String,
}

impl Storage {
    const STORAGE_FILE_EXT: &'static str = "ubdb";

    pub fn new(storage_path: String) -> Self {
        Self {
            storage_dir: storage_path,
        }
    }

    fn get_table_storage_path(&self, table_name: &str) -> String {
        format!(
            "{}/{}.{}",
            self.storage_dir,
            table_name,
            Self::STORAGE_FILE_EXT
        )
    }
}

impl Storage {}

#[allow(non_snake_case)]
pub(crate) mod DataTypeByteMap {
    pub const INT: u8 = 0;
    pub const VARCHAR: u8 = 10;
}
