use crate::i18n::Translate;

pub struct SinglelineDialog {
    title: String,
    label_text: String,
    hint_text: String,
    error_text: String,
    is_open: bool,
    input: String,
    validation_error: Option<String>,
}

impl SinglelineDialog {
    pub fn new(title: &str, label_text: &str, hint_text: &str, error_text: &str) -> Self {
        Self {
            title: title.to_string(),
            label_text: label_text.to_string(),
            hint_text: hint_text.to_string(),
            error_text: error_text.to_string(),
            is_open: false,
            input: String::new(),
            validation_error: None,
        }
    }

    pub fn set_input(&mut self, text: &str) {
        self.input = text.to_string();
    }

    pub fn show<T: Translate>(&mut self, ctx: &egui::Context, service: &T) -> Option<String> {
        if !self.is_open {
            return None;
        }

        let mut result = None;
        let mut is_open = true;

        egui::Window::new(service.t(&self.title))
            .open(&mut is_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(service.t(&self.label_text));

                // 文本输入框
                let text_edit = egui::TextEdit::singleline(&mut self.input)
                    .hint_text(service.t(&self.hint_text));

                let response = ui.add(text_edit);

                // 显示验证错误（如果有）
                if let Some(err) = &self.validation_error {
                    ui.colored_label(egui::Color32::RED, service.t(err));
                }

                ui.horizontal(|ui| {
                    // 取消按钮
                    if ui.button(service.t("cancel")).clicked() {
                        self.is_open = false;
                        self.input.clear();
                        self.validation_error = None;
                    }

                    // 确认按钮
                    if ui.button(service.t("save")).clicked() 
                        || response.lost_focus() 
                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        if self.validate_input() {
                            result = Some(self.input.clone());
                            self.is_open = false;
                            self.input.clear();
                        }
                    } 
                })
            });

        self.is_open &= is_open;
        result
    }

    fn validate_input(&mut self) -> bool {
        if self.input.trim().is_empty() {
            self.validation_error = Some(self.error_text.clone());
            false
        } else {
            self.validation_error = None;
            true
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.input.clear();
        self.validation_error = None;
    }
}