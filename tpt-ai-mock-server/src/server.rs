use crate::{router::Router, RecordedResponse};

#[cfg(feature = "std")]
use alloc::format;
#[cfg(feature = "std")]
use alloc::string::{String, ToString};
#[cfg(feature = "std")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "std")]
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

/// Mock LLM server for testing.
pub struct MockServer {
    router: Router,
    #[cfg(feature = "std")]
    addr: Option<std::net::SocketAddr>,
}

impl MockServer {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            #[cfg(feature = "std")]
            addr: None,
        }
    }

    /// Queue a response for `POST /v1/chat/completions`. Multiple calls
    /// queue multiple responses served in order (FIFO), letting a test
    /// script a multi-turn conversation (e.g. a tool-call turn followed by
    /// a final-answer turn) against a single running server.
    pub fn add_response(&mut self, response: RecordedResponse) {
        self.router.add_route("/v1/chat/completions", response);
    }

    /// Start serving on an OS-assigned localhost port and return its
    /// address. Requests are served by a background task for the lifetime
    /// of the returned `MockServer` (or until the process exits); responses
    /// queued via [`MockServer::add_response`] before this call are served
    /// in order, one per matching request, then `404` for any further
    /// request to the same path.
    #[cfg(feature = "std")]
    pub async fn start(&mut self) -> Result<std::net::SocketAddr, crate::Error> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        self.addr = Some(addr);

        let router = Arc::new(Mutex::new(core::mem::take(&mut self.router)));
        tokio::spawn(async move {
            loop {
                let (stream, _) = match listener.accept().await {
                    Ok(pair) => pair,
                    Err(_) => break,
                };
                let router = Arc::clone(&router);
                tokio::spawn(async move {
                    let _ = handle_connection(stream, router).await;
                });
            }
        });

        Ok(addr)
    }

    #[cfg(feature = "std")]
    pub fn addr(&self) -> Option<std::net::SocketAddr> {
        self.addr
    }
}

/// Read one HTTP/1.1 request, validate and route it, and write back either
/// the matching [`RecordedResponse`] (as a plain JSON body, or as an SSE
/// stream if it carries event-typed chunks) or an error response.
#[cfg(feature = "std")]
async fn handle_connection(
    mut stream: tokio::net::TcpStream,
    router: Arc<Mutex<Router>>,
) -> std::io::Result<()> {
    let (path, body) = {
        let mut reader = BufReader::new(&mut stream);

        let mut request_line = String::new();
        reader.read_line(&mut request_line).await?;
        let path = request_line
            .split_whitespace()
            .nth(1)
            .unwrap_or("/")
            .to_string();

        let mut content_length: usize = 0;
        loop {
            let mut line = String::new();
            let n = reader.read_line(&mut line).await?;
            if n == 0 || line == "\r\n" || line == "\n" {
                break;
            }
            if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                content_length = value.trim().parse().unwrap_or(0);
            }
        }

        let mut body_bytes = alloc::vec![0u8; content_length];
        if content_length > 0 {
            reader.read_exact(&mut body_bytes).await?;
        }
        (path, String::from_utf8_lossy(&body_bytes).to_string())
    };

    if let Err(e) = crate::validation::validate_request(&body) {
        let msg = format!("{{\"error\":\"{e}\"}}");
        return write_response(&mut stream, 400, "application/json", &msg).await;
    }

    let response = router
        .lock()
        .expect("mock server router lock poisoned")
        .take(&path);
    match response {
        None => write_response(&mut stream, 404, "application/json", "{\"error\":\"no route\"}")
            .await,
        Some(recorded) if recorded.chunks.len() == 1 && recorded.chunks[0].event_type.is_none() => {
            write_response(&mut stream, 200, "application/json", &recorded.chunks[0].data).await
        }
        Some(recorded) => write_sse_response(&mut stream, &recorded.chunks).await,
    }
}

#[cfg(feature = "std")]
async fn write_response(
    stream: &mut tokio::net::TcpStream,
    status: u16,
    content_type: &str,
    body: &str,
) -> std::io::Result<()> {
    let status_text = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Error",
    };
    let response = format!(
        "HTTP/1.1 {status} {status_text}\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes()).await
}

#[cfg(feature = "std")]
async fn write_sse_response(
    stream: &mut tokio::net::TcpStream,
    chunks: &[crate::SseChunk],
) -> std::io::Result<()> {
    let header =
        "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\nconnection: close\r\n\r\n";
    stream.write_all(header.as_bytes()).await?;
    for chunk in chunks {
        let mut event = String::new();
        if let Some(event_type) = &chunk.event_type {
            event.push_str(&format!("event: {event_type}\n"));
        }
        event.push_str(&format!("data: {}\n\n", chunk.data));
        stream.write_all(event.as_bytes()).await?;
    }
    stream.write_all(b"data: [DONE]\n\n").await
}

impl Default for MockServer {
    fn default() -> Self {
        Self::new()
    }
}
