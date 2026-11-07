//! Windows registry read helpers for Notion Ai Desktop.

pub const APP_KEY: &str = "Software\\Notion\\Notion Ai Desktop";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Notion Ai Desktop";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Notion Ai Desktop") },
        RegValue { name: "Version",     value: String::from("v1.7.11") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Notion Ai Desktop") },
        RegValue { name: "Publisher",       value: String::from("Notion") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Notion Ai Desktop\\uninstall.exe\"") },
    ]
}
