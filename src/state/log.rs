use crate::{logger::memory::MemoryLog, AppState};

pub trait LogState {
    fn get_logs(&self) -> Vec<MemoryLog>;
}

impl LogState for AppState {
    fn get_logs(&self) -> Vec<MemoryLog> {
        self.memory_logger.get_logs()
    }
}