use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleText {
    pub ui: HashMap<String, String>,
}

pub fn load_locales() -> anyhow::Result<HashMap<&'static str, LocaleText>> {
    let mut locales = HashMap::new();

    let en_data = include_str!("../../locales/en.json");
    locales.insert("en", serde_json::from_str(en_data)?);

    let zh_data = include_str!("../../locales/zh.json");
    locales.insert("zh", serde_json::from_str(zh_data)?);

    let jp_data = include_str!("../../locales/jp.json");
    locales.insert("jp", serde_json::from_str(jp_data)?);

    Ok(locales)
}

static LOCALES: Lazy<HashMap<&'static str, LocaleText>> = Lazy::new(|| load_locales().unwrap());

#[derive(Debug, Clone, Copy)]
pub enum Language {
    English,
    Chinese,
    Japanese,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
            Language::Japanese => "jp",
        }
    }
}

/// 翻译特征，用于UI组件
pub trait Translate {
    fn t(&self, key: &str) -> String;
}

pub(crate) fn t(key: &str, lang: Language) -> String {
    LOCALES
        .get(lang.code())
        .and_then(|loc| loc.ui.get(key))
        .cloned()
        .unwrap_or_else(|| key.to_string())
}