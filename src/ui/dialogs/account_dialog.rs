use crate::{i18n::Translate};

pub struct RegisterDialog {
    is_open: bool,
    name: String,
    email: String,
    password: String,
    confirm_password: String,
    show_password: bool,
    error: Option<ValidationError>,
}

impl RegisterDialog {
    pub fn new() -> Self {
        Self {
            is_open: false,
            name: String::new(),
            email: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            show_password: false,
            error: None,
        }
    }

    pub fn show<T: Translate>(&mut self, ctx: &egui::Context, t: &T) -> Option<(String, String, String)> {
        if !self.is_open {
            return None;
        }

        let mut result = None;
        let mut is_open = true;

        egui::Window::new(t.t("sign up"))
            .open(&mut is_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(t.t("name"));
                let name_response = ui.text_edit_singleline(&mut self.name);

                ui.label(t.t("email"));
                let email_response = ui.text_edit_singleline(&mut self.email);

                ui.label(t.t("password"));
                let mut password = self.password.clone();
                let password_response = ui.add(
                    egui::TextEdit::singleline(&mut password)
                        .password(!self.show_password)
                        .hint_text(t.t("password"))
                );
                if password_response.changed() {
                    self.password = password;
                }

                ui.label(t.t("confirm password"));
                let mut confirm_password = self.confirm_password.clone();
                let confirm_response = ui.add(
                    egui::TextEdit::singleline(&mut confirm_password)
                        .password(!self.show_password)
                        .hint_text(t.t("confirm password"))
                );
                if confirm_response.changed() {
                    self.confirm_password = confirm_password;
                }

                // 显示验证错误（如果有）
                if let Some(err) = &self.error {
                    ui.colored_label(egui::Color32::RED, t.t(err.display()));
                }

                ui.horizontal(|ui| {
                    // 取消按钮
                    if ui.button(t.t("cancel")).clicked() {
                        self.is_open = false;
                    }

                    // 确认按钮
                    if ui.button(t.t("register")).clicked() 
                        || ( name_response.lost_focus()
                        && email_response.lost_focus()
                        && password_response.lost_focus()
                        && confirm_response.lost_focus()  
                        && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        if self.password != self.confirm_password {
                            self.error = Some(ValidationError::PasswordMatchError)
                        } else if !validate_email(&self.email) {
                            self.error = Some(ValidationError::InvalidEmail);
                        } else if !validate_password(&self.password) {
                            self.error = Some(ValidationError::InvalidPassword);
                        } else {
                            result = Some((self.name.clone(), self.email.clone(), self.password.clone()));
                            self.is_open = false;
                        }
                    } 
                })
            });

        self.is_open &= is_open;
        result
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.name.clear();
        self.email.clear();
        self.password.clear();
        self.confirm_password.clear();
        self.error = None;
        self.show_password = false;
    }
}

pub struct LoginDialog {
    is_open: bool,
    email: String,
    password: String,
    show_password: bool,
    error: Option<ValidationError>,
}

impl LoginDialog {
    pub fn new() -> Self {
        Self {
            is_open: false,
            email: String::new(),
            password: String::new(),
            show_password: false,
            error: None,
        }
    }

    pub fn show<T: Translate>(&mut self, ctx: &egui::Context, t: &T) -> Option<(String, String)> {
        if !self.is_open {
            return None;
        }

        let mut result = None;
        let mut is_open = true;

        egui::Window::new(t.t("sign in"))
            .open(&mut is_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(t.t("email"));
                let email_response = ui.text_edit_singleline(&mut self.email);

                ui.label(t.t("password"));
                let mut password = self.password.clone();
                let password_response = ui.add(
                    egui::TextEdit::singleline(&mut password)
                        .password(!self.show_password)
                        .hint_text(t.t("password"))
                );
                if password_response.changed() {
                    self.password = password;
                }

                // 显示验证错误（如果有）
                if let Some(err) = &self.error {
                    ui.colored_label(egui::Color32::RED, t.t(err.display()));
                }

                ui.horizontal(|ui| {
                    // 取消按钮
                    if ui.button(t.t("cancel")).clicked() {
                        self.is_open = false;
                    }

                    // 确认按钮
                    if ui.button(t.t("login")).clicked() 
                        || (email_response.lost_focus()
                        && password_response.lost_focus() 
                        && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        if !validate_email(&self.email) {
                            self.error = Some(ValidationError::InvalidEmail);
                        } else if !validate_password(&self.password) {
                            self.error = Some(ValidationError::InvalidPassword);
                        } else {
                            result = Some((self.email.clone(), self.password.clone()));
                            self.is_open = false;
                        }
                    } 
                })
            });

        self.is_open &= is_open;
        result
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.email.clear();
        self.password.clear();
        self.error = None;
        self.show_password = false;
    }
}
pub struct LogoutDialog {

}

#[derive(Debug)]
enum ValidationError {
    InvalidEmail,
    InvalidPassword,
    PasswordMatchError,
}

impl ValidationError {
    pub fn display(&self) -> &str {
        match self {
            ValidationError::InvalidEmail => "invalid email format",
            ValidationError::InvalidPassword => "invalid password format (6-18)",
            ValidationError::PasswordMatchError => "passwords do not match",
        }
    }
}

fn validate_email(email: &str) -> bool {
    let email_regex = regex::Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    email_regex.is_match(email)
}

fn validate_password(password: &str) -> bool {
    let n = password.len();
    n >= 6 && n <= 16
}