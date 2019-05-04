extern crate json;

use json::JsonValue;
use std::collections::HashMap;

use error::I18nError;
use std::fs;

type Translations = HashMap<String, JsonValue>;

pub struct Config<'a> {
    locales: Vec<&'a str>,
    directory: &'a str,
}

impl<'a> Config<'a> {
    fn new<T: Into<&'a str>>(locales: Vec<&'a str>, directory: T) -> Self {
        Config {
            locales,
            directory: directory.into(),
        }
    }
}

pub struct I18n<'a> {
    config: Config<'a>,
    current_locale: &'a str,
    translations: Translations,
}

impl<'b> I18n<'b> {
    pub fn new<T: Into<&'b str>>(config: Config<'b>, default_locale: T) -> Result<Self, I18nError> {
        let current_locale = default_locale.into();
        let translations = read_files(&config)?;
        Ok(I18n {
            config,
            current_locale,
            translations: translations,
        })
    }
}

fn read_files<'a>(config: &'a Config<'a>) -> Result<Translations, I18nError> {
    config
        .locales
        .iter()
        .map(|locale| {
            let path = &format!("{}/{}.json", config.directory, locale);
            let buffer = fs::read_to_string(path)?;
            let contents = json::parse(&buffer)?;
            Ok((locale.to_string(), contents))
        })
        .collect::<Result<Translations, I18nError>>()
}
