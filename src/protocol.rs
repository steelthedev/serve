#[derive(Debug)]
pub struct Response {
    pub status: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    pub fn to_http_response(&self) -> String {
        let mut response = format!("HTTP/1.1 {}\r\n", self.status);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");

        response.push_str(&self.body);

        response
    }
}
