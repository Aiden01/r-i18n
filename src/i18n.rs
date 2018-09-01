pub struct I18nConfig<'a> {
    pub locales: &'a [&'a str],
    pub directory: &'a str
} 

#[derive(Clone)]
pub struct I18n<'b> {
    pub config: &'b I18nConfig<'b>,
    pub current_lang: &'b str
}

impl<'b> I18n<'b> {
    pub fn configure(config: &'b I18nConfig<'b>) -> I18n<'b> {
        if let Some(current_lang) = config.locales.get(0) {
            I18n{config: config, current_lang: current_lang}
        } else {
            panic!("You must add one language");
        }
    }  

    pub fn set_current_lang(&mut self, lang: &'b str) {
        match self.config.locales.contains(&lang) {
            true => self.current_lang = lang,
            false => panic!("Please add {} to the list.", lang)
        }
    }

}