pub mod importer;
pub mod exporter;
pub mod formats;

pub use importer::Importer;
pub use exporter::Exporter;

use crate::data::ExportNote;

// 导入结果
pub struct ImportResult {
    note: ExportNote,
    warnings: Vec<String>,
}

impl ImportResult {
    pub fn note(&self) -> &ExportNote {
        &self.note
    }

    pub fn warnings(&self) -> &Vec<String> {
        &self.warnings
    }
}

// 导入配置
#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub merge_strategy: MergeStrategy,  // 冲突处理策略
    pub preserve_timestamps: bool,      //是否保留原时间戳
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            merge_strategy: MergeStrategy::Rename,
            preserve_timestamps: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeStrategy {
    Skip,       // 跳过已有笔记
    Overwrite,  // 覆盖已有笔记
    Rename,     // 重命名新笔记(添加后缀)
}

// 导出配置
#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub format: ExportFormat,   
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::Markdown(true),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportFormat {
    Markdown(bool),   // 单个或多个.md 文件, 是否包含元数据
    Json,             // 单一JSON文件(完整备份)
    Html,             // HTML格式(可选)
}