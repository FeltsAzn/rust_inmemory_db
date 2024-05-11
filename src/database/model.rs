pub(crate) enum DatabaseCommand {
    SET,
    GET,
    UPDATE,
    DELETE,
}

#[derive(Clone)]
pub(crate) enum DatabaseValue {
    Integer(i32),
    Float(f64),
    Str(String),
}

pub(crate) struct DataModel {
    key: String,
    value: Option<DatabaseValue>,
}


impl DataModel {
    pub fn new(key: String, value: Option<DatabaseValue>) -> Self {
        DataModel {
            key,
            value,
        }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_value(self) -> Option<DatabaseValue> {
        self.value
    }
}


pub(crate) struct DatabaseResult {
    pub value: Option<DatabaseValue>,
    pub err: Option<String>,
}