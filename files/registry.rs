//! Windows registry read helpers for Altair Solidthinking.

pub const APP_KEY: &str = "Software\\Altair\\Altair Solidthinking";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Altair Solidthinking";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Altair Solidthinking") },
        RegValue { name: "Version",     value: String::from("v1.5.3") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Altair Solidthinking") },
        RegValue { name: "Publisher",       value: String::from("Altair") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Altair Solidthinking\\uninstall.exe\"") },
    ]
}
