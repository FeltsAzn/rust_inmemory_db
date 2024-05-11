use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::{DatabaseCommand, DatabaseController, DatabaseResult, DatabaseValue, DataModel, RequestParser};

use super::parser::{ParseResult, RequestParsedValue};

pub struct Manager {
    database: Arc<Mutex<DatabaseController>>,
    parser: RequestParser,
}


#[derive(Debug)]
pub struct ProcessingResult {
    pub command: Option<String>,
    pub value: Option<RequestParsedValue>,
    pub error: Option<String>,
}

impl ProcessingResult {
    pub fn skipped(&self) -> bool {
        return self.command.is_none() && self.error.is_none();
    }
}


impl Manager {
    pub fn new(database: DatabaseController, parser: RequestParser) -> Self {
        Manager {
            database: Arc::new(Mutex::new(database)),
            parser,
        }
    }

    pub fn process_request(&self, mut stream: &TcpStream) -> ProcessingResult {
        let mut buffer = [0; 512];
        let _ = stream.read(&mut buffer);
        let request = String::from_utf8_lossy(&buffer[..]);

        let parse_result = self.parse(request.to_string());

        if parse_result.error.is_none() && parse_result.command.is_some() {
            let command = parse_result.command.as_ref().unwrap().clone();
            let db_response = self.handle_by_db(parse_result);
            ProcessingResult {
                command: Some(command),
                value: Self::translate_db_value(db_response.value),
                error: db_response.err,
            }
        } else if parse_result.error.is_none() && parse_result.command.is_none() {
            ProcessingResult {
                command: None,
                value: None,
                error: None,
            }
        } else {
            let description = parse_result.error.unwrap();
            println!("Unhandled request. Description: {}", description);
            ProcessingResult {
                command: parse_result.command,
                value: parse_result.value,
                error: Some(description),
            }
        }
    }

    fn parse(&self, request: String) -> ParseResult {
        self.parser.parse_request(request)
    }

    fn handle_by_db(&self, parse_result: ParseResult) -> DatabaseResult {
        let mut database = self.database.lock().unwrap();

        let command = match parse_result.command.unwrap().as_str() {
            "GET" => DatabaseCommand::GET,
            "SET" => DatabaseCommand::SET,
            "UPDATE" => DatabaseCommand::UPDATE,
            "DELETE" => DatabaseCommand::DELETE,
            _ => return DatabaseResult {
                value: None,
                err: Some(String::from("Command not found")),
            }
        };
        let translated_val = Self::translate_parsed_value(parse_result.value);
        let data_model = DataModel::new(parse_result.key.unwrap(), translated_val);
        database.handle_command(command, data_model)
    }

    fn translate_parsed_value(parsed_value: Option<RequestParsedValue>) -> Option<DatabaseValue> {
        if let Some(value) = parsed_value {
            match value {
                RequestParsedValue::Integer(i) => Some(DatabaseValue::Integer(i)),
                RequestParsedValue::Float(f) => Some(DatabaseValue::Float(f)),
                RequestParsedValue::Str(s) => Some(DatabaseValue::Str(s))
            }
        } else {
            None
        }
    }

    fn translate_db_value(parsed_value: Option<DatabaseValue>) -> Option<RequestParsedValue> {
        if let Some(value) = parsed_value {
            match value {
                DatabaseValue::Integer(i) => Some(RequestParsedValue::Integer(i)),
                DatabaseValue::Float(f) => Some(RequestParsedValue::Float(f)),
                DatabaseValue::Str(s) => Some(RequestParsedValue::Str(s))
            }
        } else {
            None
        }
    }
}