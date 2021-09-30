use crate::opentelemetry::proto::trace::v1::ResourceSpans;
use async_trait::async_trait;

use slog::{info, Logger};

#[derive(Debug)]
pub enum StorageCommand {
    Insert {
        spans: ResourceSpans,
    }
}


#[async_trait]
pub trait StorageApi {
    async fn insert(&mut self, spans: ResourceSpans) -> Result<ResourceSpans, &'static str>;
}


pub struct MemoryStorage {
    pub spans: Vec<ResourceSpans>,
    // pub log: Logger,
}

#[async_trait]
impl StorageApi for MemoryStorage {

    async fn insert(&mut self, spans: ResourceSpans) -> Result<ResourceSpans, &'static str> {
        // info!(self.log, "Inserted Span");
        self.spans.push(spans.clone());
        return Ok(spans);
    }
}