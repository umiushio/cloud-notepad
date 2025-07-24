pub mod client;
pub mod memory;

pub use client::ClientLogger;
pub use memory::MemoryLogger;

pub enum AsyncLogType {
    Sync,
    Auth,
}

/// 记录异步任务操作
pub fn log_async(log_type: &AsyncLogType, action: &str, field: &str, value: &str, status: &str) {
    match log_type {
        AsyncLogType::Sync => {
            tracing::info!(
                log_type = "sync",
                action = %action,
                field = %field,
                value = %value,
                status = %status,
                "Sync operation"
            );
        }
        AsyncLogType::Auth => {
            tracing::info!(
                log_type = "auth",
                action = %action,
                field = %field,
                value = %value,
                status = %status,
                "Auth operation"
            );
        }
    }
}

/// 记录错误(带上下文)
pub fn log_error(error: &str, context: &str) {
    tracing::error!(
        error = %error,
        context = %context,
        "Operation failed"
    );
}
