#[derive(Debug, PartialEq, Eq)]
pub struct Part<'a> {
    pub content: &'a [u8],
}

impl<'a> From<&'a [u8]> for Part<'a> {
    fn from(content: &'a [u8]) -> Self {
        Part {
            content,
        }
    }
}

impl<'a> From<&'a str> for Part<'a> {
    fn from(text: &'a str) -> Self {
        Part {
            content: text.as_bytes(),
        }
    }
}
