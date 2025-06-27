pub mod markdown;
pub mod json;

pub(super) use markdown::MarkdownHandler;
pub(super) use json::JsonHandler;

use std::path::Path;
use chrono::{DateTime, Utc};
use super::{ImportConfig, ImportResult};

fn get_modified(path: &Path, warnings: &mut Vec<String>) -> Option<DateTime<Utc>> {
    match std::fs::metadata(path)
        .and_then(|md| match md.modified() {
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }) {
            Ok(t) => match t.duration_since(std::time::UNIX_EPOCH) {
                Ok(dur) => DateTime::from_timestamp(dur.as_secs() as i64, dur.subsec_nanos()),
                Err(e) => { warnings.push(format!("SystemTimeError: {}", e)); None }
            },
            Err(e) => { warnings.push(format!("access modified time failed: {}", e)); None }
        }
}