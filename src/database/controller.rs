use std::collections::HashMap;

use super::model::{
    DatabaseCommand,
    DatabaseResult,
    DatabaseValue,
    DataModel,
};

pub struct DatabaseController {
    inner_space: HashMap<String, DatabaseValue>,
}


impl DatabaseController {
    pub fn new() -> Self {
        DatabaseController {
            inner_space: HashMap::new()
        }
    }


    pub fn handle_command(&mut self, command: DatabaseCommand, data_model: DataModel) -> DatabaseResult {
        match command {
            DatabaseCommand::GET => {
                let value = self.get(data_model.get_key());
                DatabaseResult {
                    value: {
                        if value.is_some() {
                            let value = value.unwrap();
                            Some(value.clone())
                        } else {
                            None
                        }
                    },
                    err: None,
                }
            }
            DatabaseCommand::SET => {
                let key = data_model.get_key();
                if let Some(value) = data_model.get_value() {
                    self.create(key, value);
                    DatabaseResult {
                        value: None,
                        err: None,
                    }
                } else {
                    DatabaseResult {
                        value: None,
                        err: Some("Not found value to create".to_string()),
                    }
                }
            }
            DatabaseCommand::UPDATE => {
                let key = data_model.get_key();
                if let Some(value) = data_model.get_value() {
                    self.update(key, value);
                    DatabaseResult {
                        value: None,
                        err: None,
                    }
                } else {
                    DatabaseResult {
                        value: None,
                        err: Some("Not found value to create".to_string()),
                    }
                }
            }

            DatabaseCommand::DELETE => {
                let key = data_model.get_key();
                let returned_value = self.delete(key);
                DatabaseResult {
                    value: returned_value,
                    err: None,
                }
            }
        }
    }


    fn get(&self, key: String) -> Option<&DatabaseValue> {
        self.inner_space.get(&key)
    }


    fn create(&mut self, key: String, value: DatabaseValue) {
        self.inner_space.insert(key, value);
    }

    fn update(&mut self, key: String, value: DatabaseValue) {
        self.inner_space.insert(key, value);
    }

    fn delete(&mut self, key: String) -> Option<DatabaseValue> {
        self.inner_space.remove(&key)
    }
}