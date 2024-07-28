use std::collections::HashMap;
use unity::prelude::*;
use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use crate::interface::{get_current_voice, set_voice, reflect_voice_setting, reload_messages};

static mut PREVIEW_VOICE: i32 = 0; // ID de la voix par défaut, 0 pour Japonais
static mut CURRENT_VOICE: i32 = 0; // ID de la voix par défaut, 0 pour Japonais

// Récupérer les traductions pour chaque ID de voix
fn get_voice_translations() -> HashMap<i32, &'static str> {
    let mut translations = HashMap::new();
    translations.insert(0, "Japanese");
    translations.insert(1, "English");
    translations
}

// Récupérer le nom de la voix basé sur l'ID de voix actuel
fn get_voice_name(voice_id: i32) -> &'static str {
    get_voice_translations().get(&voice_id).unwrap_or(&"Unknown")
}

// Structure pour gérer les paramètres de voix
pub struct VoiceSettings;

// Implémentation du trait pour les méthodes de menu de changement de voix
impl ConfigBasicMenuItemSwitchMethods for VoiceSettings {
    fn init_content(this: &mut ConfigBasicMenuItem) {
        unsafe {
            CURRENT_VOICE = get_current_voice();
            PREVIEW_VOICE = CURRENT_VOICE;
            update_preview_text(this);
        }
    }

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

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) {
        unsafe {
            let help_text = get_voice_name(CURRENT_VOICE);
            this.help_text = format!("Current Voice: {}", help_text).into();
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: Option<&'static MethodInfo>) {
        unsafe {
            let command_text = get_voice_name(CURRENT_VOICE);
            this.command_text = command_text.to_string().into();
        }
    }
}

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

fn update_preview_text(this: &mut ConfigBasicMenuItem) {
    unsafe {
        let voice_name = get_voice_name(PREVIEW_VOICE);
        this.command_text = voice_name.to_string().into();
        this.update_text();
    }
}

fn update_texts(this: &mut ConfigBasicMenuItem) {
        this.title_text = "Change Voice".into();
        VoiceSettings::set_help_text(this, None);
        VoiceSettings::set_command_text(this, None);
        this.update_text();
}

#[no_mangle]
extern "C" fn voice_callback() -> &'static mut ConfigBasicMenuItem {
    let switch = ConfigBasicMenuItem::new_switch::<VoiceSettings>("Change Voice");
    if let Some(method) = switch.get_class_mut().get_virtual_method_mut("ACall") {
        method.method_ptr = a_button_confirm as _;
    }
    update_texts(switch);
    switch
}

pub fn voice_install() {
    cobapi::install_game_setting(voice_callback);
}