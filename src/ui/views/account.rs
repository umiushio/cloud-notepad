use crate::{i18n::Translate, state::AuthState, ui::dialogs::account_dialog::{LoginDialog, RegisterDialog}};

pub struct AccountView {
    is_open: bool,
    pos: egui::Pos2,
    register_dialog: RegisterDialog,
    login_dialog: LoginDialog,
}

impl AccountView {
    pub fn new() -> Self {
        Self {
            is_open: false,
            pos: egui::Pos2::default(),
            register_dialog: RegisterDialog::new(),
            login_dialog: LoginDialog::new(),
        }
    }

    pub fn show<T: AuthState + Translate>(&mut self, ui: &mut egui::Ui, state: &mut T) {
        if let Some((name, email, password)) = self.register_dialog.show(ui.ctx(), state) {
            state.register(&name, &email, &password);
        }

        if let Some((email, password)) = self.login_dialog.show(ui.ctx(), state) {
            state.login(&email, &password);
        }
        
        if !self.is_open { return; }

        let width = 150.0;

        let menu_response = egui::Window::new("account_menu")
            .title_bar(false)
            .resizable(false)
            .fixed_pos(self.pos)
            .fixed_size(egui::vec2(width, 0.0))
            .show(ui.ctx(), |ui| {
                if let Some(user_name) = state.user_name() {
                    ui.label(user_name);
                    ui.separator();
                    if ui.button("logout").clicked() {
                        state.logout();
                        self.is_open = false;
                    }
                } else {
                    if ui.button("login").clicked() {
                        self.login_dialog.open();
                        self.is_open = false;
                    }

                    if ui.button("register").clicked() {
                        self.register_dialog.open();
                        self.is_open = false;
                    }
                }
            });

        if ui.input(|i| i.key_pressed(egui::Key::Escape)) 
            || menu_response
            .map(|r| r.response.clicked_elsewhere())
            .unwrap_or(true) {
                self.is_open = false;
            }
    }

    pub fn open(&mut self, pos: egui::Pos2) {
        self.is_open = true;
        self.pos = pos;
    }
}