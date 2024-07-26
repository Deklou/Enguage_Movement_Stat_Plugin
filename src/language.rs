use unity::prelude::*;
use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use crate::interface::{get_current_language, set_language, reflect_language_setting, reload_messages};

fn get_language_name(lang_id: i32) -> &'static str {
    match lang_id {
        0 => "Japanese",
        1 => "US English",
        4 => "EU English",
        5 => "EU French",
        6 => "EU Spanish",
        7 => "EU German",
        8 => "EU Italian",
        9 => "Traditional Chinese",
        10 => "Simplified Chinese",
        11 => "Korean",
        _ => "Unknown",
    }
}

fn get_localized_string(key: &str, lang_id: i32) -> &'static str {
    match (key, lang_id) {
        ("change_language", 0) => "言語を変更",
        ("change_language", 1) => "Change Language",
        ("change_language", 4) => "Change Language",
        ("change_language", 5) => "Changer de langue",
        ("change_language", 6) => "Cambiar idioma",
        ("change_language", 7) => "Sprache ändern",
        ("change_language", 8) => "Cambia lingua",
        ("change_language", 9) => "更改语言",
        ("change_language", 10) => "更改语言",
        ("change_language", 11) => "언어 변경",
        ("current_language", 0) => "現在の言語",
        ("current_language", 1) => "Current Language",
        ("current_language", 4) => "Current Language",
        ("current_language", 5) => "Langue Actuelle",
        ("current_language", 6) => "Idioma Actual",
        ("current_language", 7) => "Aktuelle Sprache",
        ("current_language", 8) => "Lingua Corrente",
        ("current_language", 9) => "当前语言",
        ("current_language", 10) => "当前语言",
        ("current_language", 11) => "현재 언어",
        _ => "Unspecified",
    }
}


pub struct LanguageSettings;

impl ConfigBasicMenuItemSwitchMethods for LanguageSettings {
    fn init_content(this: &mut ConfigBasicMenuItem) {
        update_texts(this);
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) -> BasicMenuResult {
        unsafe {
            let current_lang = get_current_language();
            let new_lang = ConfigBasicMenuItem::change_key_value_i(current_lang, 0, 11, 1);
            if current_lang != new_lang {
                set_language(new_lang);
                reflect_language_setting();
                reload_messages();
                update_texts(this);
                BasicMenuResult::se_cursor()
            } else {
                BasicMenuResult::new()
            }
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) {
        unsafe {
            let current_language = get_current_language();
            let help_text = get_localized_string("current_language", current_language);
            let lang_name = get_language_name(current_language);
            this.help_text = format!("{}: {}", help_text, lang_name).into();
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) {
        unsafe {
            let current_language = get_current_language();
            let command_text = get_language_name(current_language);
            this.command_text = format!("{}", command_text).into();
        }
    }
}

fn update_texts(this: &mut ConfigBasicMenuItem) {
    unsafe {
        let lang_id = get_current_language();
        this.title_text = get_localized_string("change_language", lang_id).into();
        LanguageSettings::set_help_text(this, None);
        LanguageSettings::set_command_text(this, None);
        this.update_text();
    }
}

#[no_mangle]
extern "C" fn language_callback() -> &'static mut ConfigBasicMenuItem {
    let switch = ConfigBasicMenuItem::new_switch::<LanguageSettings>("Change Language");
    update_texts(switch); // Make sure text is updated here as well
    switch
}

pub fn language_install() {
    cobapi::install_game_setting(language_callback);
}