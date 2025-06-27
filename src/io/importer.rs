use std::path::Path;
use anyhow::Result;
use super::{ImportResult, ImportConfig, formats::{MarkdownHandler, JsonHandler}};

pub struct Importer;

impl Importer {
    // 从文件或目录导入
    pub fn import(input_path: &Path, config: &ImportConfig) -> Result<Vec<ImportResult>> {
        if input_path.is_dir() {
            Self::import_directory(input_path, config)
        } else {
            let result = Self::import_file(input_path, config)?;
            Ok(vec![result])
        }
    }

    // 导入单个文件
    fn import_file(file_path: &Path, config: &ImportConfig) -> Result<ImportResult> {
        match file_path.extension().and_then(|s| s.to_str()) {
            Some("md") | Some("markdown") => {
                let handler = MarkdownHandler;
                handler.import_note(file_path, config)
            }
            Some("json") => {
                let handler = JsonHandler;
                handler.import_note(file_path, config)
            }
            _ => Err(anyhow::anyhow!("Unsupported file format!")),
        }
    }

    // 导入目录
    fn import_directory(dir_path: &Path, config: &ImportConfig) -> Result<Vec<ImportResult>> {
        let mut results = Vec::new();

        for entry in std::fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                match Self::import_file(&path, config) {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        eprintln!("Failed to import {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(results)
    }
}
