
//! # i18n
//! An i18n implementation in Rust. 

//! [![Build Status](https://travis-ci.com/WebD-EG/i18n.svg?branch=master)](https://travis-ci.com/WebD-EG/i18n)

//! > API documentation [https://crates.io/crates/i18n](https://crates.io/crates/i18n)

//! **Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

//! - [i18n](#i18n)
//!   - [Installation](#installation)
//!   - [Usage](#usage)
//!     - [Configuration](#configuration)
//!     - [Example](#example)
//! - [Contribution guide](#contribution-guide)



//! ## Installation
//! To install the library, you have to put this line into your **Cargo.toml** file.
//! ```toml
//! [dependencies]
//! i18n = "version number"
//! ```

//! ## Usage

//! ### Configuration
//! First, create the configuration with the directory that contains your translations files and your languages.
//! ```rust
//! extern crate i18n;
//! use i18n::I18nConfig;

//! fn main() {
//!     let config: I18nConfig =  I18nConfig{locales: &["en", "fr", "es"], directory: "translations"};
//! }
//! ```
//! Then, load the configuration:
//! ```rust
//! extern crate i18n;
//! use i18n::I18n;

//! fn main() {
//!     let config: I18nConfig =  I18nConfig{locales: &["en", "fr", "es"], directory: "translations"};
//!     let i18n: I18n = I18n::configure(&config);
//! }
//! ```
//! With this example, you will need to have a **en.json**, **fr.json** and **es.json** inside the /translations directory. Each file should looks like that:
//! ```json
//! {
//!     "keyword": "value"
//! }
//! ```
//! ### Example
//! I have a en.json file that looks like that:
//! ```json
//! {
//!     "introduction": "Hello, my name is WebD"
//! }
//! ```

//! Then, in my main.rs

//! ```rust
//! extern crate i18n;
//! use i18n::I18n;

//! fn main() {
//!     let config: I18nConfig =  I18nConfig{locales: &["en", "fr", "es"], directory: "translations"};
//!     let i18n: I18n = I18n::configure(&config);
//!     // by default, the current language will be the first element of the locales array. You can do like that if you want to set the language:
//!     // i18n.set_current_lang("fr");
//!     i18n.t("introduction"); // output should be "Hello, my name is WebD"
//! }
//! ```

//! Now, I have a fr.json file that looks like that:
//! ```json
//! {
//!     "introduction": "Bonjour, mon nom est WebD"
//! }
//! ```

//! If I set the current language to french:
//! ```rust
//! extern crate i18n;
//! use i18n::I18n;

//! fn main() {
//!     let config: I18nConfig =  I18nConfig{locales: &["en", "fr", "es"], directory: "translations"};
//!     let i18n: I18n = I18n::configure(&config);
//!     i18n.set_current_lang("fr");
//!     i18n.t("introduction"); // output should be "Bonjour, mon nom est WebD
//! }
//! ```

//! # Contribution guide
//! 1. Fork and Clone the repository
//! 2. Create your own branch
//! 3. Start Coding!
//! 4. Make a pull request when you're done :)

extern crate json;

pub mod i18n;
pub use i18n::I18n;
pub use i18n::I18nConfig;