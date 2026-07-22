use crate::RecordedResponse;

/// Simple router for the mock server.
pub struct Router {
    routes: alloc::vec::Vec<Route>,
}

struct Route {
    path: alloc::string::String,
    response: RecordedResponse,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: alloc::vec::Vec::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, response: RecordedResponse) {
        self.routes.push(Route {
            path: alloc::string::String::from(path),
            response,
        });
    }

    #[allow(dead_code)]
    pub fn find(&self, path: &str) -> Option<&RecordedResponse> {
        self.routes
            .iter()
            .find(|r| r.path == path)
            .map(|r| &r.response)
    }

    /// Remove and return the first recorded response for `path` (FIFO per
    /// path). Lets a test queue several responses on the same path — e.g. a
    /// tool-call turn followed by a final-answer turn — by calling
    /// [`Router::add_route`] multiple times before serving requests.
    ///
    /// Only called from the `std`-gated server loop; unused (and so
    /// `#[allow(dead_code)]`) without the `std` feature.
    #[cfg_attr(not(feature = "std"), allow(dead_code))]
    pub fn take(&mut self, path: &str) -> Option<RecordedResponse> {
        let index = self.routes.iter().position(|r| r.path == path)?;
        Some(self.routes.remove(index).response)
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
