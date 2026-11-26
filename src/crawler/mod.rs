use std::future::Future;

mod error;
mod model;
mod runner;

pub use error::CrawlError;
pub use model::CrawledNotice;
pub use runner::{crawl, run};

/// A crawler fetches and normalizes notices from one source.
///
/// Site-specific fetching and parsing belong in implementations under `sources/`.
pub trait Crawler: Send + Sync {
    fn source_id(&self) -> &'static str;

    fn crawl(&self) -> impl Future<Output = Result<Vec<CrawledNotice>, CrawlError>> + Send + '_;
}
