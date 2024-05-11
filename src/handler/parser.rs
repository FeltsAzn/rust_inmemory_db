use std::fmt::{Display, Formatter};

use regex::Captures;

#[derive(Debug)]
pub(crate) enum RequestParsedValue {
    Integer(i32),
    Float(f64),
    Str(String),
}


struct ParsedBodyDTO {
    command: String,
    key: String,
    value: Option<RequestParsedValue>,
}

impl ParsedBodyDTO {
    fn new(command: String, key: String, value: Option<RequestParsedValue>) -> Self {
        Self {
            command,
            key,
            value,
        }
    }
}

impl Display for ParsedBodyDTO {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "command: {}, key: {}, value: {:?}", self.command, self.key, self.value)
    }
}

#[derive(Debug)]
pub(crate) struct ParseResult {
    pub headers: Vec<String>,
    pub command: Option<String>,
    pub key: Option<String>,
    pub value: Option<RequestParsedValue>,
    pub error: Option<String>,
}

pub(crate) struct RequestParser {}


impl RequestParser {
    pub fn new() -> Self {
        RequestParser {}
    }

    pub fn parse_request(&self, request: String) -> ParseResult {
        let (headers, body) = Self::separate(request);
        println!("{}\n{}", headers, body);
        let parsed_headers = Self::extract_headers(headers);

        if parsed_headers[0].contains("GET /") {
            return ParseResult {
                headers: parsed_headers,
                command: None,
                key: None,
                value: None,
                error: None,
            };
        }

        let parsed_body = Self::extract_body(body);

        match parsed_body {
            Ok(result) => ParseResult {
                headers: parsed_headers,
                command: Some(result.command),
                key: Some(result.key),
                value: result.value,
                error: None,
            },
            Err(err) => ParseResult {
                headers: parsed_headers,
                command: None,
                key: None,
                value: None,
                error: Some(err),
            }
        }
    }

    fn separate(request: String) -> (String, String) {
        let parts = request.split("\r\n\r\n").collect::<Vec<&str>>();

        (parts[0].to_string(), parts[1].to_string())
    }

    fn extract_headers(headers_block: String) -> Vec<String> {
        headers_block.split("\n")
            .map(|el| el.to_string())
            .collect::<Vec<String>>()
    }

    fn extract_body(body_block: String) -> Result<ParsedBodyDTO, String> {
        if body_block.len() == 0 {
            return Err("Empty body!".to_string());
        };

        let re = regex::Regex::new(
            r#"^\{\s*"request":\s*"([^"]*)",\s*"key":\s*"([^"]*)"(?:,\s*"value":\s*([^}]*))?}"#,
        ).unwrap();
        let captures = re.captures(body_block.as_str().trim());

        if let Some(captures) = captures {
            let command = Self::parse_command(&captures)?;
            let key = Self::parse_key(&captures)?;
            let parsed_value = Self::parse_value(&captures)?;

            Ok(ParsedBodyDTO::new(command, key, parsed_value))
        } else {
            Err("Invalid body format!".to_string())
        }
    }

    fn parse_command(captures: &Captures) -> Result<String, String> {
        if let Some(c) = captures.get(1) {
            Ok(c.as_str().to_string())
        } else {
            return Err("Can't parse 'request' keyword from body".to_string());
        }
    }

    fn parse_key(captures: &Captures) -> Result<String, String> {
        if let Some(k) = captures.get(2) {
            Ok(k.as_str().to_string())
        } else {
            return Err("Can't parse 'key' keyword from body".to_string());
        }
    }

    fn parse_value(captures: &Captures) -> Result<Option<RequestParsedValue>, String> {
        if let Some(v) = captures.get(3) {
            let raw_value = v.as_str();
            let parsed_value =
                if raw_value.starts_with('"') && raw_value.ends_with('"') {
                    RequestParsedValue::Str(raw_value[1..raw_value.len() - 1].to_string())
                } else if let Ok(integer) = raw_value.parse::<i32>() {
                    RequestParsedValue::Integer(integer)
                } else if let Ok(float) = raw_value.parse::<f64>() {
                    RequestParsedValue::Float(float)
                } else {
                    return Err(format!("Wrong raw_value type: {raw_value}"));
                };
            Ok(Some(parsed_value))
        } else {
            Ok(None)
        }
    }
}