use crate::{Error, RecordedResponse, router::Router};

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

    pub fn add_response(&mut self, response: RecordedResponse) {
        self.router.add_route("/v1/chat/completions", response);
    }

    #[cfg(feature = "std")]
    pub async fn start(&mut self) -> Result<std::net::SocketAddr, Error> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        self.addr = Some(addr);
        Ok(addr)
    }

    pub fn addr(&self) -> Option<std::net::SocketAddr> {
        self.addr
    }
}

impl Default for MockServer {
    fn default() -> Self {
        Self::new()
    }
}
