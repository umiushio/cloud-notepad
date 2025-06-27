use std::collections::HashMap;
use super::note::Note;

#[derive(Debug, Default)]
pub struct Notebook {
    pub(in crate::data) notes: HashMap<String, Note>,
    pub(in crate::data) tags: HashMap<String, usize>,
}

impl Notebook {
    /// 添加笔记
    pub fn insert_or_replace_note(&mut self, note: Note) {
        for tag in note.tags() {
            *self.tags.entry(tag.clone()).or_default() += 1;
        }
        if let Some(old_note) = self.notes.insert(note.id().to_string(), note) {
            for tag in old_note.tags() {
                if let Some(count) = self.tags.get_mut(tag) {
                    *count -= 1;
                    if *count == 0 {
                        self.tags.remove(tag);
                    }
                }
            }
        }
    }

    /// 删除笔记
    pub fn delete_note(&mut self, note_id: &str) {
        if let Some(note) = self.notes.get(note_id) {
            for tag in note.tags() {
                if let Some(count) = self.tags.get_mut(tag) {
                    *count -= 1;
                    if *count == 0 {
                        self.tags.remove(tag);
                    }
                } else {
                    unreachable!()
                }
            }
            self.notes.remove(note_id);
        }
    }

    pub fn find_note(&self, note_id: &str) -> Option<Note> {
        self.notes.get(note_id).cloned()
    }

    /// 获取所有标签以及使用计数
    pub fn get_tags_with_count(&self) -> HashMap<String, usize> {
        self.tags.clone()
    }

    /// 根据标题和内容是否包含关键字来筛选笔记
    pub fn filter_notes(&self, key: &str) -> Vec<&Note> {
        if !key.is_empty() {
            self.notes.values()
                .filter(|note| note.contains(&key, None))
                .collect()
        } else {
            self.notes.values().collect()
        }
    }

    /// 根据标签筛选笔记
    pub fn filter_notes_by_tag(&self, tag: Option<String>) -> Vec<&Note> {
        if let Some(tag) = tag {
            self.notes.values()
            .filter(|note| note.tags().contains(&tag))
            .collect()
        } else {
            self.notes.values().collect()
        }
    }
}
