use super::log_item::LogItem;
use async_trait::async_trait;

#[async_trait]
pub trait LogStreamTransporter: Send + Sync {
    async fn store(&self, item: LogItem);
    async fn stream(&self, item: LogItem);
}
