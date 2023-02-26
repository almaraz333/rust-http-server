use std::fs;

use crate::{server::Handler, http::{Request, Response, StatusCode, Method}};

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}",self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Weirdos coming in: {}", file_path);
                    None
                }
            },
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/test" => Response::new(StatusCode::Ok, Some("<h1>Test page</h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(p) => Response::new(StatusCode::Ok, Some(p)),
                    None => Response::new(StatusCode::NotFound, Some("Not found idiot".to_string()))
                }
            },
            _ => Response::new(StatusCode::NotFound, Some("Not found idiot".to_string()))

        }
    }
}