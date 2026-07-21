use crate::SseEvent;

/// SSE parser that handles partial/chunked JSON.
pub struct SseParser {
    buffer: alloc::string::String,
    event_type: Option<alloc::string::String>,
    data: alloc::string::String,
}

impl SseParser {
    pub fn new() -> Self {
        Self {
            buffer: alloc::string::String::new(),
            event_type: None,
            data: alloc::string::String::new(),
        }
    }

    pub fn feed(&mut self, chunk: &str) -> Option<SseEvent> {
        self.buffer.push_str(chunk);

        while let Some(line_end) = self.buffer.find('\n') {
            let line_end_incl = line_end + 1;
            let line = {
                let raw = &self.buffer[..line_end];
                let trimmed = raw.trim_end_matches('\r');
                alloc::string::String::from(trimmed)
            };
            self.buffer.drain(..line_end_incl);

            if line.is_empty() {
                let event = SseEvent {
                    event_type: self.event_type.take(),
                    data: core::mem::take(&mut self.data),
                };
                if !event.data.is_empty() {
                    return Some(event);
                }
            } else if let Some(rest) = line.strip_prefix("event:") {
                self.event_type = Some(alloc::string::String::from(rest.trim()));
            } else if let Some(rest) = line.strip_prefix("data:") {
                if !self.data.is_empty() {
                    self.data.push('\n');
                }
                self.data.push_str(rest.trim());
            }
        }

        None
    }

    pub fn flush(&mut self) -> Option<SseEvent> {
        if self.data.is_empty() {
            None
        } else {
            Some(SseEvent {
                event_type: self.event_type.take(),
                data: core::mem::take(&mut self.data),
            })
        }
    }
}

impl Default for SseParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_event() {
        let mut parser = SseParser::new();
        let event = parser.feed("data: {\"text\": \"hello\"}\n\n");
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.data, "{\"text\": \"hello\"}");
    }

    #[test]
    fn test_parse_chunked_data() {
        let mut parser = SseParser::new();
        assert!(parser.feed("data: {").is_none());
        assert!(parser.feed("\"text\": \"hello\"").is_none());
        let event = parser.feed("}\n\n");
        assert!(event.is_some());
    }

    #[test]
    fn test_event_type() {
        let mut parser = SseParser::new();
        let event = parser.feed("event: message\ndata: hello\n\n").unwrap();
        assert_eq!(event.event_type.as_deref(), Some("message"));
    }
}
