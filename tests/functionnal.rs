
extern crate translator;

use translator::*;

fn create_config<'s>() -> I18nConfig<'s> {
    I18nConfig{locales: &["en", "fr"], directory: "xx"}
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
#[should_panic]
fn should_panic_because_lang_not_registered() {
    let config: I18nConfig = create_config();
    let mut i18n: I18n = I18n::configure(&config);
    i18n.set_current_lang("es");
}