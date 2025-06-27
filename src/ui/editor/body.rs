use std::f32;

use super::preview::MarkdownPreview;
use crate::data::Note;


pub(super) struct EditorBody {
    split_ratio: f32,
    preview: MarkdownPreview,
    cursor_pos: Option<usize>,
}

impl Default for EditorBody {
    fn default() -> Self {
        Self { 
            split_ratio: 0.5,
            preview: MarkdownPreview::default(),
            cursor_pos: None,
        }
    }
}

impl EditorBody {
    pub fn show(&mut self, ui: &mut egui::Ui, note: &mut Note, show_preview: bool, split_view: bool) -> bool {
        let mut text = note.content().to_string();
        if split_view {
            // 分屏布局
            let total_height = ui.available_height();
            // println!("{}", total_height);
            let total_width = ui.available_width();
            let editor_width = total_width * self.split_ratio;
            ui.horizontal_top(|ui| {
                ui.set_height(total_height);
                // 编辑区
                ui.push_id("editor", |ui| {
                    egui::ScrollArea::both().show(ui, |ui| {
                        ui.set_max_height(total_height);
                        ui.add(egui::TextEdit::multiline(&mut text)
                            .desired_width(editor_width) // 减去滚动条宽度
                            .font(egui::TextStyle::Monospace)
                        );
                    });
                });

                // 预览区
                if show_preview {
                    ui.push_id("preview", |ui| {
                        egui::ScrollArea::both().show(ui, |ui| {
                            self.preview.show(ui, note.content(), self.cursor_pos);
                        });
                    });
                }
            });
        } else {
            // 单一视图
            if show_preview {
                egui::ScrollArea::both().show(ui, |ui| {
                    self.preview.show(ui, &note.content(), None);
                });
            } else {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut text)
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace)
                    );
                });
            }
        }
        
        note.update_content(text)
    }
}