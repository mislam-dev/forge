use super::super::log_item::LogItem;
use super::super::traits::LogStreamTransporter;
use async_trait::async_trait;
pub struct LokiStreamTransporter;

impl LokiStreamTransporter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LogStreamTransporter for LokiStreamTransporter {
    async fn store(&self, _item: LogItem) {
        println!("Storing log item in Loki: {:#?}", _item);
    }
    async fn stream(&self, _item: LogItem) {
        println!("Streaming log item to Loki: {:#?}", _item);
    }
}
