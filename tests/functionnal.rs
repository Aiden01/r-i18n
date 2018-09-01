
extern crate i18n;

use i18n::*;

fn create_config<'s>() -> I18nConfig<'s> {
    I18nConfig{locales: &["en", "fr"], directory: "translations"}
}


#[test]
fn should_set_current_lang() {
    let config: I18nConfig = create_config();
    let mut i18n: I18n = I18n::configure(&config);
    i18n
        .set_current_lang("en");
    assert_eq!(i18n.current_lang, "en");
}
#[test]
fn should_set_current_lang_by_default() {
    let config: I18nConfig = create_config();
    let i18n: I18n = I18n::configure(&config);
    assert_eq!(i18n.current_lang, "en");
}

#[should_panic]
#[test]
fn should_panic_because_lang_not_registered() {
    let config: I18nConfig = create_config();
    let mut i18n: I18n = I18n::configure(&config);
    i18n.set_current_lang("es");
}

#[test]
fn should_translate_fr_to_en() {
    let config: I18nConfig = create_config();
    let mut i18n: I18n = I18n::configure(&config);
    i18n.set_current_lang("fr");
    let text = i18n.t("software");
    assert_eq!(text, "logiciel");
}
#[test]
fn should_translate_en_to_fr() {
    let config: I18nConfig = create_config();
    let mut i18n: I18n = I18n::configure(&config);
    i18n.set_current_lang("en");
    let text = i18n.t("logiciel");
    assert_eq!(text, "software");
}
#[test]
fn should_translate_a_long_text_en() {
    let config: I18nConfig = create_config();
    let mut i18n: I18n = I18n::configure(&config);
    i18n.set_current_lang("en");
    let text = i18n.t("introduction");
    assert_eq!(text, "Hello, my name is WebD");
}
#[test]
fn should_translate_a_long_text_fr() {
    let config: I18nConfig = create_config();
    let mut i18n: I18n = I18n::configure(&config);
    i18n.set_current_lang("fr");
    let text = i18n.t("introduction");
    assert_eq!(text, "Bonjour, mon nom est WebD");
}