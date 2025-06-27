use std::path::PathBuf;
use rfd::FileDialog;

pub fn pick_available_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Markdown,JSON", &["md", "markdown", "json"])
        .pick_file()
}

pub fn pick_directroy() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}

pub fn save_markdown_file(default_title: &str) -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Markdown", &["md"])
        .set_file_name(&format!("{}.md", default_title))
        .save_file()
}

pub fn save_json_file(default_title: &str) -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_file_name(&format!("{}.json", default_title))
        .save_file()
}