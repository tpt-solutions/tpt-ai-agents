use crate::RecordedResponse;

/// Simple router for the mock server.
pub struct Router {
    routes: alloc::vec::Vec<Route>,
}

#[allow(dead_code)]
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
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
