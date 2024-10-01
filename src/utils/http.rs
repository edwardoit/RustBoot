use std::collections::HashMap;

/// Enum for HTTP Methods
#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
}

/// Enum for HTTP Versions
#[derive(Debug, Clone)]
pub enum HttpVersion {
    HTTP10,
    HTTP11,
    HTTP20,
}

/// Struct to represent HTTP headers (key-value pairs)
#[derive(Debug, Clone)]
pub struct HttpHeaders {
    pub headers: HashMap<String, String>,
}

impl HttpHeaders {
    pub fn new() -> Self {
        HttpHeaders {
            headers: HashMap::new(),
        }
    }

    /// Adds a header to the collection
    // Correct: `&str` is sized, it's a reference to `str`, if there is an explicit type assignation str obtain a compile time error ( not sized!)
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    /// Retrieves a header by key
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
}

/// Enum for HTTP Body types (text, JSON, binary)
#[derive(Debug, Clone)]
pub enum HttpBody {
    Text(String),
    Json(String),
    Binary(Vec<u8>),
    Empty,  // GET req
}

/// Struct for the HTTP Request
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub uri: String,
    pub version: HttpVersion,
    pub headers: HttpHeaders,
    pub body: HttpBody,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, uri: &str, version: HttpVersion, headers: HttpHeaders, body: HttpBody) -> Self {
        HttpRequest {
            method,
            uri: uri.to_string(),
            version,
            headers,
            body,
        }
    }

}

/// Struct for the HTTP Response
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status_code: u16,
    pub status_message: String,
    pub headers: HttpHeaders,
    pub body: HttpBody,
}

impl HttpResponse {
    pub fn new(version: HttpVersion, status_code: u16, status_message: &str, headers: HttpHeaders, body: HttpBody) -> Self {
        HttpResponse {
            version,
            status_code,
            status_message: status_message.to_string(),
            headers,
            body,
        }
    }

    pub fn as_string(&self) -> String {
        // Create the status line
        let status_line = format!(
            "{:?} {} {}\r\n",
            self.version,
            self.status_code,
            self.status_message
        );

        // Create the headers
        let mut headers_string = String::new();
        for (key, value) in &self.headers.headers {
            headers_string.push_str(&format!("{}: {}\r\n", key, value));
        }

        // Handle the body content based on its type
        let body = match &self.body {
            HttpBody::Text(text) => text.clone(), // Assuming `HttpBody::Text`
            HttpBody::Json(json) => json.to_string(), // Assuming `HttpBody::Json`
            HttpBody::Binary(v8) => {
                // Convert Vec<u8> to String and handle errors
                String::from_utf8(v8.clone()).unwrap_or_else(|e| {
                    eprintln!("Failed to convert Vec<u8> to String: {}", e);
                    String::new()
                })
            }
            _ => String::new(),
        };

        // Create the full response by concatenating status line, headers, and body
        format!(
            "{}{}Content-Length: {}\r\n\r\n{}",
            status_line,
            headers_string,
            body.len(),
            body
        )
    }
}