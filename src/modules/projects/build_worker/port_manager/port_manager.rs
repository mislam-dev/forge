use crate::shared::error::AppError;
use std::collections::HashSet;
use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct PortManager {
    allocated: Arc<Mutex<HashSet<u16>>>,
    range_start: u16,
    range_end: u16,
}

impl PortManager {
    pub fn new(range_start: u16, range_end: u16) -> Self {
        Self {
            allocated: Arc::new(Mutex::new(HashSet::new())),
            range_start,
            range_end,
        }
    }

    /// Acquire a free port — atomically checked + tracked in-process.
    pub async fn acquire(&self) -> Result<u16, AppError> {
        let mut allocated = self.allocated.lock().await;

        for port in self.range_start..=self.range_end {
            if allocated.contains(&port) {
                continue; // already handed out this session
            }
            if Self::is_os_available(port) {
                allocated.insert(port);
                return Ok(port);
            }
        }

        Err(AppError::InternalServerError(format!(
            "Port pool exhausted (range {}–{}, {} allocated)",
            self.range_start,
            self.range_end,
            allocated.len(),
        )))
    }

    /// Release a port back to the pool when an app shuts down.
    pub async fn release(&self, port: u16) {
        let mut allocated = self.allocated.lock().await;
        allocated.remove(&port);
        tracing::info!(port, "Port released back to pool");
    }

    /// How many ports are still available.
    pub async fn available_count(&self) -> usize {
        let allocated = self.allocated.lock().await;
        let total = (self.range_end - self.range_start + 1) as usize;
        total - allocated.len()
    }

    /// Snapshot of all currently allocated ports (for health/debug endpoints).
    pub async fn allocated_ports(&self) -> HashSet<u16> {
        self.allocated.lock().await.clone()
    }

    fn is_os_available(port: u16) -> bool {
        TcpListener::bind(("0.0.0.0", port)).is_ok()
    }
}
