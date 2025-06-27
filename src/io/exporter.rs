use std::path::Path;
use anyhow::Result;
use crate::data::{ExportNote, NoteVersion};
use super::{ExportConfig, ExportFormat, formats::{MarkdownHandler, JsonHandler}};

pub struct Exporter;

impl Exporter {
    // 导出单个笔记
    pub fn export_note(note: &ExportNote, output_path: &Path, config: &ExportConfig) -> Result<()> {
        match config.format {
            ExportFormat::Markdown(include_metadata) => {
                let handler = MarkdownHandler;
                handler.export_note(note, output_path, include_metadata)
            }
            ExportFormat::Json => {
                let handler = JsonHandler;
                handler.export_note(note, output_path)
            }
            _ => unimplemented!(),
        }
    }

    // 导出单个笔记所有版本(JSON格式)
    pub fn export_note_versions(versions: Vec<NoteVersion>, output_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(&versions)?;
        std::fs::write(output_path, json)?;
        Ok(())
    }

    // 导出所有笔记到目录
    pub fn export_all(notes: Vec<ExportNote>, output_dir: &Path, config: &ExportConfig) -> Result<()> {
        match config.format {
            ExportFormat::Markdown(include_metadata) => {
                let handler = MarkdownHandler;
                for note in notes.iter() {
                    let mut filename = sanitize_filename(&note.title) + ".md";
                    if std::fs::exists(output_dir.join(&filename))? {
                        let id = note.id.clone().unwrap_or(uuid::Uuid::new_v4().to_string());
                        filename = format!("{}-{{{}}}.md", sanitize_filename(&note.title), id);
                    }
                    let path = output_dir.join(filename);
                    handler.export_note(note, &path, include_metadata)?;
                }
                Ok(())
            }
            ExportFormat::Json => {
                let path = output_dir.join("notes_backup.json");
                let json = serde_json::to_string_pretty(&notes)?;
                std::fs::write(path, json)?;
                Ok(())
            }
            _ => unimplemented!(),
        }
    }
}

fn sanitize_filename(name: &str) -> String {
    // 实现文件名安全处理
    name.replace(|c: char| !c.is_ascii_alphanumeric(), "_")
}