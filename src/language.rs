use engage::menu::{BasicMenuResult};
use engage::dialog::yesno::{BasicDialogItemYes, YesNoDialog, TwoChoiceDialogMethods};
use engage::menu::config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods};
use unity::prelude::{MethodInfo, OptionalMethod};
use crate::interface::{get_current_language, set_language, reflect_language_setting, reload_messages};

static mut PREVIEW_LANG: i32 = 1;

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
        ("change_language_confirm", 0) => "changerlangue ?",
        ("change_language_confirm", 1) => "changerlangue ?",
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
        unsafe {
            PREVIEW_LANG = get_current_language();
            update_texts(this);
        }
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) -> BasicMenuResult {
        unsafe {
            let new_lang = ConfigBasicMenuItem::change_key_value_i(PREVIEW_LANG, 0, 11, 1);
            if PREVIEW_LANG != new_lang {
                PREVIEW_LANG = new_lang;
                update_texts(this);
                BasicMenuResult::se_cursor()
            } else {
                BasicMenuResult::new()
            }
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) {
        unsafe {
            let help_text = get_localized_string("current_language", PREVIEW_LANG);
            let lang_name = get_language_name(PREVIEW_LANG);
            this.help_text = format!("{}: {}", help_text, lang_name).into();
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) {
        unsafe {
            let command_text = get_language_name(PREVIEW_LANG);
            this.command_text = format!("{}", command_text).into();
        }
    }
}

extern "C" fn a_button_confirm(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) -> BasicMenuResult {
    unsafe {
        YesNoDialog::bind::<LanguageConfirmation>(
            this.menu,  
            get_localized_string("change_language_confirm", PREVIEW_LANG),
            "Yes",
            "No" 
        );
    }
    BasicMenuResult::se_cursor()
}

struct LanguageConfirmation;

impl TwoChoiceDialogMethods for LanguageConfirmation {
    extern "C" fn on_first_choice(_this: &mut BasicDialogItemYes, _method_info: OptionalMethod) -> BasicMenuResult {
        unsafe {
            set_language(PREVIEW_LANG);
            reflect_language_setting();
            reload_messages();
        }
        BasicMenuResult::new().with_close_this(true)
    }

}

fn update_texts(this: &mut ConfigBasicMenuItem) {
    unsafe {
        let lang_id = PREVIEW_LANG;
        this.title_text = get_localized_string("change_language", lang_id).into();
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