use std::{collections::HashSet, path::Path};
use crate::data::ExportNote;
use super::{ImportConfig, ImportResult};

#[derive(Default)]
pub(in crate::io) struct MarkdownHandler;

impl MarkdownHandler {
    // 导出单个笔记到Markdown文件
    pub fn export_note(&self, note: &ExportNote, path: &Path, include_metadata: bool) -> anyhow::Result<()> {
        let mut content = String::new();

        // 添加元数据作为Front Matter(如果配置需要)
        if include_metadata {
            content.push_str(&format!("---\n"));
            content.push_str(&format!("title: {}\n", note.title()));
            if let Some(created_at) = note.created() {
                content.push_str(&format!("created: {}\n", created_at));
            }
            if !note.tags().is_empty() {
                content.push_str(&format!("tags: [{}]\n", 
                    note.tags().iter().cloned().collect::<Vec<_>>().join(" ")
                ));
            }
            content.push_str(&format!("---\n\n"));
        }

        // 添加内容
        content.push_str(note.content());

        //写入文件
        std::fs::write(path, content)?;
        Ok(())
    }

    // 从Markdown文件中导入笔记
    pub fn import_note(&self, path: &Path, config: &ImportConfig) -> anyhow::Result<ImportResult> {
        let content = std::fs::read_to_string(path)?;
        let (front_matter, content) = self.parse_front_matter(&content);
        
        let mut warnings = Vec::new();

        let title = front_matter
            .as_ref()
            .and_then(|fm| Some(fm.title.clone()))
            .take_if(|s| !s.is_empty())
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Untitled")
                    .to_string()
            });
        
        let tags = front_matter
            .as_ref()
            .and_then(|fm| Some(fm.tags.clone()))
            .unwrap_or(HashSet::new());

        let created_at = front_matter
            .and_then(|fm| fm.created)
            .and_then(|s| match chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M") {
                Ok(dt) => Some(dt.and_local_timezone(chrono::Local).unwrap().to_utc()),
                Err(e) => { warnings.push(format!("parse created time failed: {}", e)); None }
            });
        
        let updated_at = if config.preserve_timestamps {
            super::get_modified(path, &mut warnings)
        } else {
            None
        };

        Ok(ImportResult {
            note: ExportNote::new(None, title, tags, content, created_at, updated_at),
            warnings,
        })
    }
}

impl MarkdownHandler {
    // 解析Front Matter(YAML格式)
    fn parse_front_matter(&self, content: &str) -> (Option<FrontMatter>, String) {
        // 查找 front matter 块
        let mut lines = content.lines();
        let mut front_lines = Vec::new();
        let mut in_front = false;
        let mut is_valid = false;

        for line in lines.by_ref() {
            if line.trim() == "---" {
                if !in_front {
                    in_front = true;
                } else {
                    // 结束 front matter
                    break;
                }
            } else if in_front {
                if line.trim().starts_with("title:") { is_valid = true; }
                front_lines.push(line);
            }
        }

        if is_valid {
            let content_rest = lines.collect::<Vec<_>>().join("\n");

            let mut title = String::new();
            let mut created = None;
            let mut tags = HashSet::new();

            for line in front_lines {
                let line = line.trim();
                if let Some(rest) = line.strip_prefix("title:") {
                    title = rest.trim().to_string();
                } else if let Some(rest) = line.strip_prefix("created:") {
                    created = Some(rest.trim().to_string());
                } else if let Some(rest) = line.strip_prefix("tags:") {
                    // 解析 tags: [tag1 tag2 ...]
                    let rest = rest.trim();
                    if rest.starts_with('[') && rest.ends_with(']') {
                        let tag_str = &rest[1..rest.len()-1];
                        for tag in tag_str.split_whitespace() {
                            if !tag.is_empty() {
                                tags.insert(tag.to_string());
                            }
                        }
                    }
                }
            }
            
            let fm = FrontMatter {
                title,
                created,
                tags,
            };
            (Some(fm), content_rest)
        } else {
            (None, content.to_string())
        }
    }
}

#[derive(Debug)]
struct FrontMatter {
    title: String,
    created: Option<String>,
    tags: HashSet<String>,
}