// https://github.com/mainrs/terminal-link-rs/blob/main/src/lib.rs
use std::fmt;

/// A clickable link component.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Link<'a> {
    pub id: &'a str,
    pub text: &'a str,
    pub url: &'a str,
}

impl<'a> Link<'a> {
    /// Create a new link with a name and target url.
    pub fn new(text: &'a str, url: &'a str) -> Self {
        Self { text, url, id: "" }
    }

    /// Create a new link with a name, a target url and an id.
    pub fn with_id(text: &'a str, url: &'a str, id: &'a str) -> Self {
        Self { text, url, id }
    }
}

impl fmt::Display for Link<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.id != "" {
            write!(
                f,
                "\u{1b}]8;id={};{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
                self.id, self.url, self.text
            )
        } else {
            write!(
                f,
                "\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\",
                self.url, self.text
            )
        }
    }
}