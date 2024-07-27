#[unity::from_offset("App", "Language", "GetLang")]
pub fn get_current_language() -> i32;

#[unity::from_offset("App", "Language", "SetLang")]
pub fn set_language(lang: i32);

#[unity::from_offset("App", "Language", "ReflectSetting")]
pub fn reflect_language_setting();

#[unity::from_offset("App", "Mess", "Reload")]
pub fn reload_messages();