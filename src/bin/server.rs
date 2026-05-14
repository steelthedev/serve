use serve::protocol::Response;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server running on http://127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Socket closed
                    Ok(0) => return,

                    Ok(n) => {
                        // Convert request to string
                        let request = String::from_utf8_lossy(&buf[0..n]);

                        println!("Incoming Request:\n{}", request);

                        // Response object
                        let response = Response {
                            status: "200 OK".to_string(),
                            headers: vec![
                                ("Content-Type".to_string(), "text/plain".to_string()),
                                ("Content-Length".to_string(), "11".to_string()),
                            ],
                            body: "Hello World".to_string(),
                        };

                        // Convert struct into raw HTTP response
                        let http_response = response.to_http_response();

                        // Send response
                        if let Err(e) = socket.write_all(http_response.as_bytes()).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }

                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
