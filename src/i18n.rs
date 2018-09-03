
extern crate json;

use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::collections::HashMap;

use json::JsonValue;

/// I18n configuration
/// # Example
/// ```no-run
/// extern crate i18n;
/// use i18n::I18nConfig;
///
/// fn main() {
///     let config: I18nConfig =  I18nConfig{locales: &["en", "fr", "es"], directory: "translations"};
/// }
/// ```
pub struct I18nConfig<'a> {
    pub locales: &'a [&'a str],
    pub directory: &'a str
}


pub struct I18n<'b> {
    pub config: &'b I18nConfig<'b>,
    pub current_lang: &'b str,
    pub translations: HashMap<String, JsonValue>
}

impl<'b> I18n<'b> {
    /// Configures the library
    ///
    /// # Example
    /// ```no-run
    /// extern crate i18n;
    /// use i18n::I18n;
    /// use i18n::I18nConfig;
    /// 
    /// extern crate r_i18n;
    /// use r_i18n::I18n;
    /// use r_i18n::I18nConfig;
    ///
    /// fn main() {
    ///     let config: I18nConfig =  I18nConfig{locales: &["en", "fr", "es"], directory: "translations"};
    ///     let i18n: I18n = I18n::configure(&config);
    /// }
    /// ```
    pub fn configure(config: &'b I18nConfig<'b>) -> I18n<'b> {
        if let Some(current_lang) = config.locales.get(0) {
            let mut translations = HashMap::new();
            let mut i18n: I18n = I18n{config: config, current_lang: current_lang, translations: HashMap::new()};
            let buffers: HashMap<String, String> = i18n.read_files();
            for (lang, json) in buffers {
                let parsed_json = json::parse(json.as_str()).unwrap();
                translations.insert(lang, parsed_json);
            }
            i18n.translations = translations;
            i18n
        } else {
            panic!("You must add one language");
        }
    }


    /// Sets the current language
    ///
    /// # Example
    /// ```no-run
    /// extern crate r_i18n;
    /// use r_i18n::I18n;
    /// use r_i18n::I18nConfig;
    ///
    /// fn main() {
    ///     let config: I18nConfig =  I18nConfig{locales: &["en", "fr", "es"], directory: "translations"};
    ///     let mut r_i18n: I18n = I18n::configure(&config);
    ///     r_i18n.set_current_lang("fr");
    /// }
    /// ```
    pub fn set_current_lang(&mut self, lang: &'b str) {
        match self.config.locales.contains(&lang) {
            true => self.current_lang = lang,
            false => panic!("Please add {} to the list.", lang)
        }
    }

    fn read_files(&self) -> HashMap<String, String> {
        let mut buffers = HashMap::new();
        for filename in self.config.locales {
            let path = &format!("{}/{}.json", self.config.directory, filename);
            let file = File::open(path).expect("Failed to open the file");
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents).expect("Failed to read the file");
            buffers.insert(filename.to_string(), contents);
        }
        buffers
    }

    /// Translates by the keyword
    ///
    /// # Example
    /// ```no-run
    /// extern crate i18n;
    /// use i18n::I18n;
    /// use i18n::I18nConfig;
    /// 
    /// extern crate r_i18n;
    /// use r_i18n::I18n;
    /// use r_i18n::I18nConfig;
    ///
    /// fn main() {
    ///     let config: I18nConfig =  I18nConfig{locales: &["en", "fr", "es"], directory: "translations"};
    ///     let mut i18n: I18n = I18n::configure(&config);
    ///     i18n.set_current_lang("fr");
    ///     i18n.t("introduction"); // output should be "Bonjour, mon nom est WebD
    /// }
    /// ```
    pub fn t(&self, key: &'b str) -> &JsonValue {
        match self.translations.get(self.current_lang) {
            Some(language_json) => {
                if language_json.has_key(key) {
                    &language_json[key]
                } else {
                    panic!("Unable to find the key {}", key);
                }
            },
            None => panic!("Unable to get the language")
        }
    }

}
