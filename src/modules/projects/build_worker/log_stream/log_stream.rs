use super::log_item::LogItem;
use super::traits::LogStreamTransporter;

pub struct LogStream {
    transporters: Vec<Box<dyn LogStreamTransporter>>,
}

impl LogStream {
    pub fn new(transporters: Vec<Box<dyn LogStreamTransporter>>) -> Self {
        Self { transporters }
    }

    pub fn add_transporter(&mut self, transporter: Box<dyn LogStreamTransporter>) {
        self.transporters.push(transporter);
    }

    pub async fn stream(&self, item: LogItem) {
        for transporter in &self.transporters {
            transporter.store(item.clone()).await;
            transporter.stream(item.clone()).await;
        }
    }
}

impl std::fmt::Debug for LogStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogStream").finish()
    }
}

// impl Clone for LogStream {
//     fn clone(&self) -> Self {
//         let new_transporters: Vec<Box<dyn LogStreamTransporter>> = vec![];
//         // for transporter in &self.transporters {
//         //     let t = transporter.into();
//         //     new_transporters.push(t);
//         // }
//         Self {
//             transporters: new_transporters,
//         }
//     }
// }
