use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrawlError {
    source_id: &'static str,
    message: String,
}

impl CrawlError {
    pub fn new(source_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            source_id,
            message: message.into(),
        }
    }

    pub fn source_id(&self) -> &'static str {
        self.source_id
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for CrawlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "crawler '{}' failed: {}", self.source_id, self.message)
    }
}

impl Error for CrawlError {}
