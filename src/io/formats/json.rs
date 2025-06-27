use std::path::Path;
use crate::data::ExportNote;
use super::{ImportConfig, ImportResult};

pub(in crate::io) struct JsonHandler;

impl JsonHandler {
    // 导出JSON文件
    pub fn export_note(&self, note: &ExportNote, path: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&note)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    // 导入JSON文件
    pub fn import_note(&self, path: &Path, config: &ImportConfig) -> anyhow::Result<ImportResult> {
        let content = std::fs::read_to_string(path)?;
        let mut note: ExportNote = serde_json::from_str(&content)?;
        let mut warnings = Vec::new();
        if config.preserve_timestamps {
            if note.updated.is_none() {
                note.updated = super::get_modified(path, &mut warnings);
            }
        } else {
            note.created = None;
            note.updated = None;
        }
        Ok(ImportResult {
            note,
            warnings,
        })
    }
}