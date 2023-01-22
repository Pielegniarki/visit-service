use enum_as_inner::EnumAsInner;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, EnumAsInner)]
#[serde(untagged)]
pub enum DatabaseId {
    Patient(u32),
    Doctor(String)
}

impl<'a> Into<DatabaseId> for &'a str {
    fn into(self) -> DatabaseId {
        let string = self.to_owned();
        match string.parse::<u32>() {
            Ok(int) => DatabaseId::Patient(int),
            Err(_) => DatabaseId::Doctor(string)
        }
    }
}

impl Into<DatabaseId> for String {
    fn into(self) -> DatabaseId {
        match self.parse::<u32>() {
            Ok(int) => DatabaseId::Patient(int),
            Err(_) => DatabaseId::Doctor(self)
        }
    }
}

impl Into<String> for DatabaseId {
    fn into(self) -> String {
        match self {
            DatabaseId::Doctor(string) => string,
            DatabaseId::Patient(int) => int.to_string()
        }
    }
}