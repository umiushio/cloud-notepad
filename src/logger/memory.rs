use tracing_subscriber::layer::{Context, Layer};
use tracing::{Subscriber, Event};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct MemoryLog {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub target: String,
    pub fields: String,
}

#[derive(Clone)]
pub struct MemoryLogger {
    logs: Arc<Mutex<Vec<MemoryLog>>>,
    max_entries: usize,
}

impl MemoryLogger {
    pub fn new(max_entries: usize) -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::with_capacity(max_entries))),
            max_entries,
        }
    }

    pub fn get_logs(&self) -> Vec<MemoryLog> {
        self.logs.lock().unwrap().clone()
    }
}

impl<S: Subscriber> Layer<S> for MemoryLogger {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut log_entry = MemoryLog {
            timestamp: chrono::Local::now().to_rfc3339(),
            level: event.metadata().level().as_str().to_string(),
            message: "".to_string(),
            target: event.metadata().target().to_string(),
            fields: "".to_string(),
        };

        let mut visitor = LogVisitor::new(&mut log_entry);
        event.record(&mut visitor);

        let mut logs = self.logs.lock().unwrap();
        if logs.len() >= self.max_entries {
            logs.remove(0);
        }
        logs.push(log_entry);
    }
}

struct LogVisitor<'a> {
    entry: &'a mut MemoryLog,
}

impl<'a> LogVisitor<'a> {
    fn new(entry: &'a mut MemoryLog) -> Self {
        Self { entry }
    }
}

impl<'a> tracing::field::Visit for LogVisitor<'a> {    
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.record_str(field, &value.to_string())
    }
    
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.record_str(field, &value.to_string())
    }
    
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.record_str(field, &value.to_string())
    }
    
    fn record_i128(&mut self, field: &tracing::field::Field, value: i128) {
        self.record_str(field, &value.to_string())
    }
    
    fn record_u128(&mut self, field: &tracing::field::Field, value: u128) {
        self.record_str(field, &value.to_string())
    }
    
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.record_str(field, &value.to_string())
    }
    
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "message" => self.entry.message = value.to_string(),
            "level" => self.entry.level = value.to_string(),
            "target" => self.entry.target = value.to_string(),
            _ => {
                if !self.entry.fields.is_empty() {
                    self.entry.fields.push_str(", ");
                }
                self.entry.fields.push_str(&format!("{}={}", field.name(), value));
            }
        }
    }
    
    fn record_bytes(&mut self, field: &tracing::field::Field, value: &[u8]) {
        self.record_str(field, &String::from_utf8(value.to_vec()).unwrap())
    }
    
    fn record_error(&mut self, field: &tracing::field::Field, value: &(dyn std::error::Error + 'static)) {
        self.record_str(field, &format!("{}", value));
    }
    
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.record_str(field, &format!("{:?}", value));
    }
}