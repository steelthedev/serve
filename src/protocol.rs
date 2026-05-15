#[derive(Debug)]
pub struct Response {
    pub status: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
}

impl Response {
    pub fn to_http_response(&self) -> String {
        let mut response = format!("HTTP/1.1 {}\r\n", self.status);

        // Add headers
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        // Auto content length
        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));

        // Empty line
        response.push_str("\r\n");

        // Body
        response.push_str(&self.body);

        response
    }
}

impl Request {
    pub fn from_raw(raw: &str) -> Self {
        let mut lines = raw.lines();

        // First line: GET /hello HTTP/1.1
        let request_line = lines.next().unwrap_or("");

        let mut request_parts = request_line.split_whitespace();

        let method = request_parts.next().unwrap_or("").to_string();

        let path = request_parts.next().unwrap_or("").to_string();

        let version = request_parts.next().unwrap_or("").to_string();

        let mut headers = Vec::new();

        // Remaining lines = headers
        for line in lines {
            // Empty line means end of headers
            if line.trim().is_empty() {
                break;
            }

            // Split "Host: localhost:8080"
            if let Some((key, value)) = line.split_once(":") {
                headers.push((key.trim().to_string(), value.trim().to_string()));
            }
        }

        Self {
            method,
            path,
            version,
            headers,
        }
    }
}
