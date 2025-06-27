use crate::{i18n::Language, io::{ExportConfig, ImportConfig}};
use super::{AppState, Theme};

pub trait SettingsService {
    fn language(&self) -> Language;
    fn language_mut(&mut self) -> &mut Language;
    fn theme(&self) -> Theme;
    fn theme_mut(&mut self) -> &mut Theme;

    fn export_config_mut(&mut self) -> &mut ExportConfig;
    fn import_config_mut(&mut self) -> &mut ImportConfig;
}

impl SettingsService for AppState {
    fn language(&self) -> Language {
        self.language
    }

    fn language_mut(&mut self) -> &mut Language {
        &mut self.language
    }

    fn theme(&self) -> Theme {
        self.theme
    }

    fn theme_mut(&mut self) -> &mut Theme {
        &mut self.theme
    }

    fn export_config_mut(&mut self) -> &mut ExportConfig {
        &mut self.export_config
    }

    fn import_config_mut(&mut self) -> &mut ImportConfig {
        &mut self.import_config
    }
}