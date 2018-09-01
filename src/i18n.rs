
use std::fs::File;
use std::io::Read;
use std::io::BufReader;

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
            let i18n: I18n = I18n{config: config, current_lang: current_lang};
            let buffers = i18n.read_files();
            i18n
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

    fn read_files(&self) -> Vec<String> {
        let mut buffers = Vec::new();
        &self.config.locales
                .iter()
                .map(|file| {
                    let path = &format!("{}/{}.json", self.config.directory, file);
                    let file = File::open(path).expect("Failed to open the file");
                    let mut reader = BufReader::new(file);
                    let mut contents = String::new();
                    reader.read_to_string(&mut contents).expect("Failed to read the file");
                    buffers.push(contents);
                });
        buffers
    }

}