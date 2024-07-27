use std::collections::HashMap;
use unity::prelude::*;
use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use crate::interface::{get_current_language, set_language, reflect_language_setting, reload_messages};

static mut PREVIEW_LANG: i32 = 1;
static mut CURRENT_LANG: i32 = 1;

fn get_language_translations() -> HashMap<i32, Vec<&'static str>> {
    let mut translations = HashMap::new();
    translations.insert(0, vec!["日本語",  "Japanese", "", "", "Japanese", "Japonais", "Japonés", "Japanisch", "Giapponese", "日本語", "日语", "일본어"]);
    translations.insert(1, vec!["日本語で", "English", "", "", "English", "Anglais", "Inglés", "Englisch", "Inglese", "英语", "英语", "영어"]);
    translations.insert(3, vec!["スペイン語", "Spanish", "", "", "Spanish", "Espagnol", "Español", "Spanisch", "Spagnolo", "西班牙语", "西班牙语", "스페인어"]);
    translations.insert(4, vec!["英語で", "English", "", "", "English", "Anglais", "Inglés", "Englisch", "Inglese", "英文", "英文", "영어"]);
    translations.insert(5, vec!["フランス語", "French", "", "", "French", "Français", "Francés", "Französisch", "Francese", "法语", "法语", "프랑스어"]);
    translations.insert(6, vec!["スペイン語", "Spanish", "", "", "Spanish", "Espagnol", "Español", "Spanisch", "Spagnolo", "西班牙语", "西班牙语", "스페인어"]);
    translations.insert(7, vec!["ドイツ語", "German", "", "", "German", "Allemand", "Alemán", "Deutsch", "Tedesco", "德语", "德语", "독일어"]);
    translations.insert(8, vec!["イタリア語", "Italian", "", "", "Italian", "Italien", "Italiano", "Italienisch", "Italiano", "意大利语", "意大利语", "이탈리아어"]);
    translations.insert(9, vec!["中国語 (繁体)", "Traditional Chinese", "", "", "Traditional Chinese", "Chinois Traditionnel", "Chino Tradicional", "Traditionelles Chinesisch", "Cinese Tradizionale", "传统中文", "传统中文", "중국어 (번체)"]);
    translations.insert(10, vec!["中国語 (簡体)", "Simplified Chinese", "", "", "Simplified Chinese", "Chinois Simplifié", "Chino Simplificado", "Vereinfachtes Chinesisch", "Cinese Semplificato", "简体中文", "简体中文", "중국어 (간체)"]);
    translations.insert(11, vec!["韓国語", "Korean", "", "", "Korean", "Coréen", "Coreano", "Koreanisch", "Coreano", "韩语", "韩语", "한국어"]);
    translations
}

fn get_language_name(lang_id: i32, current_lang_id: i32) -> &'static str {
    let translations = get_language_translations();
    translations.get(&lang_id).and_then(|names| names.get(current_lang_id as usize)).unwrap_or(&"Unknown")
}

fn get_localized_string(key: &str, lang_id: i32) -> &'static str {
    match (key, lang_id) {
        ("change_language_confirm", 0) => "changerlangue ?",
        ("change_language_confirm", 1) => "changerlangue ?",
        ("change_language_confirm", 3) => "changerlangue ?",
        ("change_language_confirm", 4) => "changerlangue ?",
        ("change_language_confirm", 5) => "changerlangue ?",
        ("change_language_confirm", 6) => "changerlangue ?",
        ("change_language_confirm", 7) => "changerlangue ?",
        ("change_language_confirm", 8) => "changerlangue ?",
        ("change_language_confirm", 9) => "changerlangue ?",
        ("change_language_confirm", 10) => "changerlangue ?",
        ("change_language_confirm", 11) => "changerlangue ?",
        ("change_language", 0) => "言語を変更",
        ("change_language", 1) => "Change Language",
        ("change_language", 3) => "Cambiar idioma",
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
        ("current_language", 3) => "Idioma Actual",
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
        unsafe {
            CURRENT_LANG = get_current_language();
            PREVIEW_LANG = CURRENT_LANG;
            update_preview_text(this);
        }
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) -> BasicMenuResult {
        unsafe {
            let new_lang = ConfigBasicMenuItem::change_key_value_i(PREVIEW_LANG, 0, 11, 1);
            if PREVIEW_LANG != new_lang {
                PREVIEW_LANG = new_lang;
                update_preview_text(this);
                BasicMenuResult::se_cursor()
            } else {
                BasicMenuResult::new()
            }
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) {
        unsafe {
            let help_text = get_localized_string("current_language", CURRENT_LANG);
            let lang_name = get_language_name(CURRENT_LANG, CURRENT_LANG);
            this.help_text = format!("{}: {}", help_text, lang_name).into();
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) {
        unsafe {
            let command_text = get_language_name(CURRENT_LANG, CURRENT_LANG);
            this.command_text = format!("{}", command_text).into();
        }
    }
}

extern "C" fn a_button_confirm(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) -> BasicMenuResult {
    unsafe {
        if PREVIEW_LANG != CURRENT_LANG {
            set_language(PREVIEW_LANG);
            reflect_language_setting();
            reload_messages();
            CURRENT_LANG = PREVIEW_LANG;
            update_texts(this);
        }
        BasicMenuResult::se_cursor()
    }
}

fn update_preview_text(this: &mut ConfigBasicMenuItem) {
    unsafe {
        let lang_name = get_language_name(PREVIEW_LANG, CURRENT_LANG);
        this.command_text = format!("{}", lang_name).into();
        this.update_text();
    }
}

fn update_texts(this: &mut ConfigBasicMenuItem) {
    unsafe {
        this.title_text = get_localized_string("change_language", CURRENT_LANG).into();
        LanguageSettings::set_help_text(this, None);
        LanguageSettings::set_command_text(this, None);
        this.update_text();
    }
}


#[no_mangle]
extern "C" fn language_callback() -> &'static mut ConfigBasicMenuItem {
    let switch = ConfigBasicMenuItem::new_switch::<LanguageSettings>("Change Language");
    switch.get_class_mut().get_virtual_method_mut("ACall").map(|method| method.method_ptr = a_button_confirm as _);
    update_texts(switch);
    switch
}

pub fn language_install() {
    cobapi::install_game_setting(language_callback);
}