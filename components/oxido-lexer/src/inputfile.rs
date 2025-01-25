use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct InputFile<'a> {
    pub filename: &'static str,
    pub content: Peekable<Chars<'a>>,
}

impl<'a> InputFile<'a> {
    pub fn new(filename: &'static str, content: Chars<'a>) -> InputFile<'a> {
        InputFile {
            filename,
            content: content.peekable(),
        }
    }
}
