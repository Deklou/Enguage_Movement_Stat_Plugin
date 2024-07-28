use std::collections::HashMap;
use unity::prelude::*;
use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use crate::interface::{reload_messages, get_current_voice, set_voice, reflect_voice_setting};
use crate::language::CURRENT_LANG;

static mut PREVIEW_VOICE: i32 = 1;
static mut CURRENT_VOICE: i32 = 1;

//get the voice translations for each voice ID
fn get_voice_translations() -> HashMap<i32, Vec<&'static str>> {
    let mut translations = HashMap::new();
    translations.insert(0, vec!["日本語", "Japanese", "", "Japonés", "Japanese", "Japonais", "Japonés", "Japanisch", "Giapponese", "日本語", "日语", "일본어"]);
    translations.insert(1, vec!["英語", "English", "", "Inglés", "English", "Anglais", "Inglés", "Englisch", "Inglese", "韩语", "韩语", "한국어"]);
    translations
}

//get the name of a voice based on the selected voice ID and current language ID
fn get_voice_name(voice_id: i32, current_lang_id: i32) -> &'static str {
    let translations = get_voice_translations();
    translations.get(&voice_id).and_then(|names| names.get(current_lang_id as usize)).unwrap_or(&"Unknown")
}

//get the localized string for different keys
fn get_localized_string(key: &str, lang_id: i32) -> &'static str {
    match (key, lang_id) {
        ("change_voice_confirm", 0) => "音声を変えますか？",
        ("change_voice_confirm", 1) => "Change voice?",
        ("change_voice_confirm", 3) => "¿Cambiar voz?",
        ("change_voice_confirm", 4) => "Changer de voix?",
        ("change_voice_confirm", 5) => "Change voice?",
        ("change_voice_confirm", 6) => "¿Cambiar voz?",
        ("change_voice_confirm", 7) => "Allemand?",
        ("change_voice_confirm", 8) => "Italien?",
        ("change_voice_confirm", 9) => "韩语",
        ("change_voice_confirm", 10) => "韩语",
        ("change_voice_confirm", 11) => "한국어",
        ("current_voice", 0) => "現在の声",
        ("current_voice", 1) => "Current voice",
        ("current_voice", 3) => "Voz actual",
        ("current_voice", 4) => "Current voice",
        ("current_voice", 5) => "Voix actuelle",
        ("current_voice", 6) => "Voz actual",
        ("current_voice", 7) => "Aktuelle Stimme",
        ("current_voice", 8) => "Voce attuale",
        ("current_voice", 9) => "目前語音",
        ("current_voice", 10) => "当前语音",
        ("current_voice", 11) => "현재 음성",
        _ => "Unspecified",
    }
}

pub struct VoiceSettings;

//implementing the trait for menu switch methods
impl ConfigBasicMenuItemSwitchMethods for VoiceSettings {
    //initialize the content of the menu
    fn init_content(this: &mut ConfigBasicMenuItem) {
        unsafe {
            CURRENT_VOICE = get_current_voice();
            PREVIEW_VOICE = CURRENT_VOICE;
            update_preview_text(this);
        }
    }

    //changing the voice preview
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) -> BasicMenuResult {
        unsafe {
            let new_voice = ConfigBasicMenuItem::change_key_value_i(PREVIEW_VOICE, 0, 1, 1);
            if PREVIEW_VOICE != new_voice {
                PREVIEW_VOICE = new_voice;
                update_preview_text(this);
                BasicMenuResult::se_cursor()
            } else {
                BasicMenuResult::new()
            }
        }
    }

    //set help text
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) {
        unsafe {
            let help_text = get_localized_string("current_voice", CURRENT_LANG);
            let voice_name = get_voice_name(CURRENT_VOICE, CURRENT_LANG);
            this.help_text = format!("{}: {}", help_text, voice_name).into();
        }
    }

    //set command text
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) {
        unsafe {
            let command_text = get_voice_name(CURRENT_VOICE, CURRENT_LANG);
            this.command_text = command_text.to_string().into();
        }
    }
}

//confirm the voice change when the A button is pressed
extern "C" fn a_button_confirm(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) -> BasicMenuResult {
    unsafe {
        if PREVIEW_VOICE != CURRENT_VOICE {
            set_voice(PREVIEW_VOICE);
            CURRENT_VOICE = PREVIEW_VOICE;
            reflect_voice_setting();
            reload_messages();
            update_texts(this);
        }
        BasicMenuResult::se_cursor()
    }
}

//update the preview text based on the selected voice
fn update_preview_text(this: &mut ConfigBasicMenuItem) {
    unsafe {
        let voice_name = get_voice_name(PREVIEW_VOICE, CURRENT_LANG);
        this.command_text = voice_name.to_string().into();
        this.update_text();
    }
}

//update this plugin texts to reflect the current voice setting
fn update_texts(this: &mut ConfigBasicMenuItem) {
    unsafe {
        this.title_text = get_localized_string("change_voice_confirm", CURRENT_LANG).into();
        VoiceSettings::set_help_text(this, None);
        VoiceSettings::set_command_text(this, None);
        this.update_text();
    }
}

//callback function for voice selection
#[no_mangle]
extern "C" fn voice_callback() -> &'static mut ConfigBasicMenuItem {
    let switch = ConfigBasicMenuItem::new_switch::<VoiceSettings>("Change Voice");
    if let Some(method) = switch.get_class_mut().get_virtual_method_mut("ACall") {
        method.method_ptr = a_button_confirm as _;
    }
    
    update_texts(switch);
    switch
}

//function to install the voice setting in the game
pub fn voice_install() {
    cobapi::install_game_setting(voice_callback);
}
