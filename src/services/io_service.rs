use std::path::Path;
use anyhow::Result;
use crate::data::{Note, ExportNote};
use crate::io::{ExportConfig, Exporter, ImportConfig, ImportResult, Importer, MergeStrategy};
use super::AppState;
use super::NoteService;

pub trait IoService {
    fn export_config(&self) -> &ExportConfig;
    fn import_config(&self) -> &ImportConfig;
    // 导出笔记
    fn export_note(&self, note_id: &str, output_path: &Path) -> Result<()>;
    // 导出所有笔记
    fn export_all_notes(&self, output_dir: &Path) -> Result<()>;
    // 导入笔记
    fn import(&mut self, input_path: &Path) -> Result<usize>;
}

impl IoService for AppState {
    fn export_config(&self) -> &ExportConfig {
        &self.export_config
    }

    fn import_config(&self) -> &ImportConfig {
        &self.import_config
    }

    fn export_note(&self, note_id: &str, output_path: &Path) -> Result<()> {
        if let Some(note) = self.get_note(note_id) {
            let export_note = ExportNote::from_note(&note);
            Exporter::export_note(&export_note, output_path, &self.export_config)
        } else {
            Err(anyhow::anyhow!("The note id is not existed: {}", note_id))
        }
    }

    fn export_all_notes(&self, output_dir: &Path) -> Result<()> {
        let notes = self.get_all_notes_for_export()?;
        Exporter::export_all(notes, output_dir, &self.export_config)
    }

    fn import(&mut self, input_path: &Path) -> Result<usize> {
        let results = Importer::import(input_path, &self.import_config)?;
        self.save_imported_notes(results)
    }
}

impl AppState {
    fn get_all_notes_for_export(&self) -> Result<Vec<ExportNote>> {
        Ok(
            self.filter_notes("")?.iter()
            .map(|note| ExportNote::from_note(note))
            .collect()
        )
    }

    fn save_imported_notes(&mut self, results: Vec<ImportResult>) -> Result<usize> {
        let mut count = 0;
        for result in results.iter() {
            let note = result.note().to_note();
            if let Err(e) = self.save_imported_note(&note) {
                eprintln!("Failed to import note: {}", e);
            } else {
                count += 1;
            }
        }

        Ok(count)
    }

    // 导入笔记
    fn save_imported_note(&mut self, note: &Note) -> Result<()> {
        let mut note = note.clone();
        if let Some(_) = self.get_note(note.id()) {
            match self.import_config.merge_strategy {
                MergeStrategy::Skip => return Ok(()),
                MergeStrategy::Rename => {
                    *note.id_mut() = uuid::Uuid::new_v4().to_string();
                }
                _ => (),
            };
        }

        // 先保存笔记到数据库中
        self.save_note(&note)?;
        // 再更新内存中笔记信息
        let mut notebook = self.notebook.lock().unwrap();
        notebook.insert_or_replace_note(note);
        Ok(())
    }
}