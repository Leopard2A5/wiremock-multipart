use lazy_regex::{regex};

#[derive(Debug, PartialEq, Eq)]
pub struct Part<'a> {
    pub content: &'a [u8],
}

impl<'a> Part<'a> {
    pub fn name(&self) -> Option<&'a str> {
        let header = self.header();
        match header {
            None => None,
            Some(header) => {
                let regex = regex!(r#"name="([^"].*)""#i);
                regex.captures(header)
                    .and_then(|cap| cap.get(1))
                    .map(|mtch| mtch.as_str())
            },
        }
    }

    pub fn filename(&self) -> Option<&'a str> {
        let header = self.header();
        match header {
            None => None,
            Some(header) => {
                let regex = regex!(r#"filename="([^"].*)""#i);
                regex.captures(header)
                    .and_then(|cap| cap.get(1))
                    .map(|mtch| mtch.as_str())
            },
        }
    }

    pub fn content_type(&self) -> Option<&'a str> {
        let header = self.header();
        match header {
            None => None,
            Some(header) => {
                let regex = regex!(r#"content-type:\s*([^\n].*)"#i);
                regex.captures(header)
                    .and_then(|cap| cap.get(1))
                    .map(|mtch| mtch.as_str())
            },
        }
    }

    pub fn header(&self) -> Option<&'a str> {
        match self.header_body_boundary() {
            None => None,
            Some(end_of_header_index) => {
                Some(std::str::from_utf8(&self.content[0..end_of_header_index]).unwrap())
            },
        }
    }

    pub fn body(&self) -> Option<&'a [u8]> {
        match self.header_body_boundary() {
            None => None,
            Some(end_of_header_index) => {
                Some(&self.content[(end_of_header_index + 2)..])
            },
        }
    }

    fn header_body_boundary(&self) -> Option<usize> {
        let mut end_of_header_index = None;
        for index in 0..(self.content.len() - 1) {
            if self.content[index] == '\n' as u8 && self.content[index + 1] == '\n' as u8 {
                end_of_header_index = Some(index);
                break;
            }
        }

        end_of_header_index
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_part_name() {
        assert_eq!(
            Part::from("Content-Disposition: form-data; name=\"text\"\nContent-Type: plain/text\n\ncontent").name(),
            Some("text"),
        );
    }

    #[test]
    fn should_extract_file_name() {
        assert_eq!(
            Part::from("Content-Disposition: form-data; name=\"text\"\n; filename=\"my-file.txt\"\nContent-Type: plain/text\n\ncontent").filename(),
            Some("my-file.txt"),
        );
    }

    #[test]
    fn should_extract_content_type() {
        assert_eq!(
            Part::from("Content-Disposition: form-data; name=\"text\"\n; filename=\"my-file.txt\"\nContent-Type: plain/text\n\ncontent").content_type(),
            Some("plain/text"),
        );
    }

    #[test]
    fn should_extract_part_body() {
        assert_eq!(
            Part::from("Content-Disposition: form-data; name=\"text\"\nContent-Type: plain/text\n\ncontent").body(),
            Some("content".as_bytes()),
        );
    }
}
