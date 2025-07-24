use crate::logger::memory::MemoryLog;


pub struct LogView {
    is_open: bool,
    logs: Vec<MemoryLog>,
    filter: String,
    auto_scroll: bool,
}

impl LogView {
    pub fn new() -> Self {
        Self {
            is_open: false,
            logs: Vec::new(),
            filter: String::new(),
            auto_scroll: true,
        }
    }

    pub fn update_logs(&mut self, new_logs: Vec<MemoryLog>) {
        self.is_open = true;
        self.logs = new_logs;
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        if !self.is_open { return false; }
        egui::Window::new("Log Viewer")
            .open(&mut self.is_open)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    ui.text_edit_singleline(&mut self.filter);
                    ui.checkbox(&mut self.auto_scroll, "Auto-scroll");
                });

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for log in &self.logs {
                            if  !self.filter.is_empty() && 
                                !log.message.contains(&self.filter) &&
                                !log.fields.contains(&self.filter)
                            {
                                continue;
                            }

                            let color = match log.level.as_str() {
                                "ERROR" => egui::Color32::RED,
                                "WARN" => egui::Color32::YELLOW,
                                "INFO" => egui::Color32::WHITE,
                                "DEBUG" => egui::Color32::GRAY,
                                _ => egui::Color32::LIGHT_GRAY,
                            };

                            ui.colored_label(color, format!(
                                "[{}] [{}] {} - {} | {}",
                                log.timestamp,
                                log.level,
                                log.target,
                                log.message,
                                log.fields,
                            ));

                            if self.auto_scroll {
                                ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                            }
                        }
                    })
            });

        self.is_open
    }
}