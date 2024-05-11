use std::fs;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use threadpool::ThreadPool;

use crate::{Manager, ProcessingResult, RequestParsedValue};

pub(crate) struct GateListener {
    tcp_listener: TcpListener,
}


impl GateListener {
    pub fn new(port: u16) -> Self {
        GateListener {
            tcp_listener: TcpListener::bind(format!("0.0.0.0:{}", port))
                .expect(format!("Can't bind {}. Possibly port is busy by another application", port).as_str())
        }
    }

    pub fn listen(&self, scheduler: ThreadPool, handler: Manager) {
        let shared_manager = Arc::new(handler);

        for stream in self.tcp_listener.incoming() {
            let stream = stream.unwrap();
            let manager = Arc::clone(&shared_manager);
            scheduler.execute(move || {
                let result = manager.process_request(&stream);
                Self::send_response(stream, result);
            });
        }
    }

    fn send_response(mut stream: TcpStream, result: ProcessingResult) {
        println!("{:?}", result);
        println!("{}", result.skipped());
        let (status_code, contents) = if result.skipped() {
            ("HTTP/1.1 200 OK", Self::index_page())
        } else {
            match result.error {
                None => {
                    let value = Self::get_value_string(&result);
                    ("HTTP/1.1 200 OK", Self::success_page(result.command.unwrap(), value))
                }
                Some(err) => ("HTTP/1.1 400 BAD REQUEST", Self::bad_request_page(err)),
            }
        };

        let length = contents.len();

        let response = format!(
            "{status_code}\r\n\
            Content-Length: {length}\r\n\
            Content-Type: text/html;\r\n\r\n\
            {contents}"
        );
        stream.write_all(response.as_bytes()).expect("Can't send response");
    }

    fn get_value_string(result: &ProcessingResult) -> Option<String> {
        if result.value.is_some() {
            let val_string = match result.value.as_ref().unwrap() {
                RequestParsedValue::Integer(int) => format!("i32 val: {}", int),
                RequestParsedValue::Float(float) => format!("f64 val: {}", float),
                RequestParsedValue::Str(string) => format!("String val: {}", string)
            };
            Some(val_string)
        } else {
            None
        }
    }

    fn index_page() -> String {
        let response_file = "templates/index.html";
        fs::read_to_string(response_file)
            .expect("Can't open file.")
    }

    fn success_page(command: String, value: Option<String>) -> String {
        let response_file = "templates/200.html";
        let mut contents = fs::read_to_string(response_file)
            .expect("Can't open file.");
        contents = contents.replace("COMMAND", command.as_str());
        if let Some(val) = value {
            contents = contents
                .replace("was successful. Returned value is VALUE",
                         format!("was successful. Returned value is {}", val)
                             .as_str());
        } else {
            contents = contents.replace("Returned value is VALUE", "");
        }
        contents
    }

    fn bad_request_page(error: String) -> String {
        let response_file = "templates/400.html";
        let mut contents = fs::read_to_string(response_file)
            .expect("Can't open file.");
        contents = contents.replace("ERROR", error.as_str());
        contents
    }
}