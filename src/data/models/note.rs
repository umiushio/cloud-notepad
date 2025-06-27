use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use super::note_version::NoteVersion;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub(in crate::data) id: String,             // UUID
    pub(in crate::data) title: String,
    pub(in crate::data) content: String,
    pub(in crate::data) tags: HashSet<String>,
    pub(in crate::data) created_at: DateTime<Utc>,
    pub(in crate::data) updated_at: DateTime<Utc>,
    pub(in crate::data) is_pinned: bool,
}

impl Note {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            content: String::new(),
            tags: HashSet::new(),
            created_at: now,
            updated_at: now,
            is_pinned: false,
        }
    }

    pub fn updated_by_note_version(&mut self, note_version: &NoteVersion) {
        self.title = note_version.title.clone();
        self.content = note_version.content.clone();
        self.tags = note_version.tags.clone();
        self.updated_at = note_version.saved_at.clone();
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn id_mut(&mut self) -> &mut String{
        &mut self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn tags(&self) -> &HashSet<String> {
        &self.tags
    }

    pub fn updated_at(&self) -> String {
        format!("{}", self.updated_at.with_timezone(&Local).format("%Y-%m-%d %H:%M"))
    }

    /// 更新标题 (变化时会刷新更新时间)
    pub fn update_title(&mut self, title: String) -> bool {
        if self.title != title {
            self.title = title;
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// 更新内容 (变化时会刷新更新时间)
    pub fn update_content(&mut self, content: String) -> bool {
        if self.content != content {
            self.content = content;
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// 判断笔记中(标题和内容)是否含有关键字符串
    pub fn contains(&self, key: &str, case_sensitive: Option<bool>) -> bool {
        let case_sensitive = case_sensitive.unwrap_or(false);
        if case_sensitive {
            self.title.contains(key) || self.content.contains(key)
        } else {
            self.title.to_lowercase().contains(&key.to_lowercase())
            || self.content.to_lowercase().contains(&key.to_lowercase())
        }
    }

    /// 新增标签 (标签变化不会影响修改时间)
    pub fn add_tag(&mut self, tag: String) -> bool {
        self.tags.insert(tag)
    }

    /// 删除标签 (标签变化不会影响修改时间)
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        self.tags.remove(tag)
    }

}