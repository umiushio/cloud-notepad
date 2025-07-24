use std::sync::Arc;

use tracing::{info_span, Instrument, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_appender::non_blocking::WorkerGuard;

use super::{AsyncLogType, MemoryLogger};

/// 客户端日志配置
pub struct ClientLogger {
    console_enabled: bool,
    file_enabled: bool,
    log_dir: Option<String>,
    max_level: Level,
    _guard: Option<WorkerGuard>,    // 保持文件写入器存活
}

impl ClientLogger {
    /// 创建新的日志配置
    pub fn new() -> Self {
        Self {
            console_enabled: true,
            file_enabled: false,
            log_dir: None,
            max_level: Level::INFO,
            _guard: None,
        }
    }

    /// 启用控制台日志
    pub fn enable_console(mut self, level: Level) -> Self {
        self.console_enabled = true;
        self.max_level = std::cmp::max(self.max_level, level);
        self
    }

    /// 启用文件日志
    pub fn enable_file(mut self, dir: &str, level: Level) -> Self {
        self.file_enabled = true;
        self.log_dir = Some(dir.to_string());
        self.max_level = std::cmp::max(self.max_level, level);
        self
    }

    /// 初始化日志系统
    pub fn init(self, memory_logger: Option<MemoryLogger>) -> Arc<Self> {
        let filter = EnvFilter::from_default_env()
            .add_directive(self.max_level.into())
            .add_directive("async_io=info".parse().unwrap());

        // 控制台输出(开发环境)
        let console_layer = if self.console_enabled {
            Some(fmt::Layer::default()
                .with_writer(std::io::stdout)
                .with_ansi(true)
                .with_level(true)
                .with_target(true)
                .with_thread_names(true))
        } else {
            None
        };

        // 文件输出(生产环境)
        let mut guard = None;
        let file_layer = if self.file_enabled {
            let dir = self.log_dir.as_ref().unwrap();
            let file_appender = tracing_appender::rolling::daily(dir, "notes_client.log");
            let (non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);

            guard = Some(file_guard);
            Some(fmt::Layer::default()
                .json()
                .with_writer(non_blocking)
                .with_ansi(false))
        } else {
            None
        };

        // 初始化
        Registry::default()
            .with(filter)
            .with(console_layer)
            .with(file_layer)
            .with(memory_logger)
            .init();
        Arc::new(Self { _guard: guard, ..self })
    }
}

/// 异步任务包装器
pub async fn with_logging<F, T, E>(
    log_type: AsyncLogType, 
    action: &str, 
    field: Option<&str>,
    value: Option<&str>,
    task: F) -> Result<T, E> 
where
    F: std::future::Future<Output = Result<T, E>> + Send,
    E: std::error::Error
{
    let span = info_span!("async_task", action);
    
    async move {
        let field = field.unwrap_or("");
        let value = value.unwrap_or("");
        // 启动日志
        super::log_async(&log_type, action, field, value, "started");

        let result = task.await;

        // 完成/失败日志
        match &result {
            Ok(_) => super::log_async(&log_type, action, field, value, "completed"),
            Err(e) => {
                super::log_async(&log_type, action, field, value, &format!("failed: {}", e.to_string()));
                super::log_error(&format!("{:?}", e), action);
            }
        }
        
        result
    }
    .instrument(span)
    .await
    
}
