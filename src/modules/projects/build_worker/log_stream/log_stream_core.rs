use super::log_item::{LogItem, LogLevel};
use super::traits::LogStreamTransporter;
use std::sync::Arc;

#[derive(Clone)]
pub struct LogStream {
    transporters: Vec<Arc<dyn LogStreamTransporter>>,
}

impl LogStream {
    pub fn new(transporters: Vec<Arc<dyn LogStreamTransporter>>) -> Self {
        Self { transporters }
    }

    pub fn empty() -> Self {
        Self {
            transporters: Vec::new(),
        }
    }

    pub fn add_transporter(&mut self, transporter: Arc<dyn LogStreamTransporter>) {
        self.transporters.push(transporter);
    }

    pub fn transporter_count(&self) -> usize {
        self.transporters.len()
    }

    pub async fn stream(&self, item: LogItem) {
        for transporter in &self.transporters {
            transporter.store(item.clone()).await;
            transporter.stream(item.clone()).await;
        }
    }

    pub async fn log_info(&self, deployment_id: &str, step: &str, message: &str) {
        self.stream(LogItem::new(
            step,
            LogLevel::Info,
            message.to_string(),
            deployment_id.to_string(),
            false,
        ))
        .await;
    }

    pub async fn log_warn(&self, deployment_id: &str, step: &str, message: &str) {
        self.stream(LogItem::new(
            step,
            LogLevel::Warn,
            message.to_string(),
            deployment_id.to_string(),
            false,
        ))
        .await;
    }

    pub async fn log_error(&self, deployment_id: &str, step: &str, message: &str, is_end: bool) {
        self.stream(LogItem::new(
            step,
            LogLevel::Error,
            message.to_string(),
            deployment_id.to_string(),
            is_end,
        ))
        .await;
    }

    pub async fn log_end(&self, deployment_id: &str, step: &str, message: &str) {
        self.stream(LogItem::new(
            step,
            LogLevel::Info,
            message.to_string(),
            deployment_id.to_string(),
            true,
        ))
        .await;
    }
}

impl std::fmt::Debug for LogStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogStream")
            .field("transporters_count", &self.transporters.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestTransporter {
        stored_count: AtomicUsize,
        streamed_count: AtomicUsize,
    }

    #[async_trait]
    impl LogStreamTransporter for TestTransporter {
        async fn store(&self, _item: LogItem) {
            self.stored_count.fetch_add(1, Ordering::SeqCst);
        }
        async fn stream(&self, _item: LogItem) {
            self.streamed_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[tokio::test]
    async fn test_log_stream_clone_preserves_transporters() {
        let transporter = Arc::new(TestTransporter {
            stored_count: AtomicUsize::new(0),
            streamed_count: AtomicUsize::new(0),
        });

        let original = LogStream::new(vec![transporter.clone()]);
        let cloned = original.clone();

        assert_eq!(cloned.transporter_count(), 1);

        cloned.log_info("dep-123", "Build", "Starting build").await;

        assert_eq!(transporter.stored_count.load(Ordering::SeqCst), 1);
        assert_eq!(transporter.streamed_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_log_stream_helpers() {
        let transporter = Arc::new(TestTransporter {
            stored_count: AtomicUsize::new(0),
            streamed_count: AtomicUsize::new(0),
        });

        let stream = LogStream::new(vec![transporter.clone()]);
        stream.log_warn("dep-1", "WarnStep", "warning").await;
        stream.log_error("dep-1", "ErrorStep", "error", false).await;
        stream.log_end("dep-1", "DoneStep", "all done").await;

        assert_eq!(transporter.stored_count.load(Ordering::SeqCst), 3);
        assert_eq!(transporter.streamed_count.load(Ordering::SeqCst), 3);
    }
}
