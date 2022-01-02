use std::str::FromStr;

use wiremock::http::HeaderName;
use wiremock::Request;

use crate::part::Part;

pub trait RequestUtils {
    fn multipart_contenttype(&self) -> Option<MultipartContentType>;
    fn parts(&self) -> Vec<Part>;
}

impl RequestUtils for Request {
    fn multipart_contenttype(&self) -> Option<MultipartContentType> {
        let content_type = self.headers.get(&HeaderName::from_str("content-type").unwrap());

        match content_type {
            None => None,
            Some(content_type) => {
                let multipart_value = content_type.iter().find(|value| {
                    value.as_str().to_lowercase().starts_with("multipart/")
                });
                match multipart_value {
                    None => None,
                    Some(value) => {
                        let parts = dbg!(value.as_str().split(";").collect::<Vec<_>>());

                        let multipart_type = parts[0].split("/")
                            .skip(1)
                            .take(1)
                            .collect::<Vec<_>>()[0]
                            .trim();

                        let boundary = parts.iter()
                            .map(|part| part.trim())
                            .find(|part| {
                                part.starts_with("boundary=")
                            })
                            .map(|whole| {
                                whole.split("=")
                                    .skip(1)
                                    .take(1)
                                    .collect::<Vec<_>>()[0]
                                    .trim()
                            });

                        Some(
                            MultipartContentType {
                                multipart_type,
                                boundary,
                            },
                        )
                    },
                }
            },
        }
    }

    fn parts(&self) -> Vec<Part> {
        if let Some(content_type) = self.multipart_contenttype() {
            if content_type.multipart_type == "form-data" {
                if let Some(boundary) = content_type.boundary {
                    let boundary = {
                        let mut tmp: Vec<u8> = vec!['-' as u8; boundary.as_bytes().len() + 2];
                        tmp[0] = '-' as u8;
                        tmp[1] = '-' as u8;
                        tmp[2..].copy_from_slice(boundary.as_bytes());
                        tmp
                    };

                    let boundary_start_indexes = self.body
                        .windows(boundary.len())
                        .enumerate()
                        .filter(|(_, window)| window == &boundary)
                        .map(|(index, _)| index)
                        .collect::<Vec<_>>();

                    boundary_start_indexes
                        .windows(2)
                        .map(|w| (boundary.len() + 1 + w[0], w[1]))
                        .map(|(start, end)| {
                            &self.body[start..end]
                        })
                        .map(|it| Part::from(it))
                        .collect::<Vec<_>>()
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MultipartContentType<'a> {
    pub multipart_type: &'a str,
    pub boundary: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use maplit::hashmap;

    use crate::test_utils::{name, request, requestb, values};

    use super::*;

    #[test]
    fn multipart_contenttype_should_return_none_if_no_multipart_request() {
        assert_eq!(
            request(
                hashmap!{},
            ).multipart_contenttype(),
            None
        );

        assert_eq!(
            request(
                hashmap!{
                    name("accept") => values("application/json"),
                },
            ).multipart_contenttype(),
            None
        );

        assert_eq!(
            request(
                hashmap!{
                    name("content-type") => values("image/jpeg"),
                },
            ).multipart_contenttype(),
            None
        );
    }

    #[test]
    fn multipart_contenttype_should_return_some_if_multipart_request() {
        assert_eq!(
            request(
                hashmap!{
                    name("content-type") => values("multipart/foo"),
                },
            ).multipart_contenttype(),
            Some(
                MultipartContentType {
                    multipart_type: "foo",
                    boundary: None,
                }
            )
        );

        assert_eq!(
            request(
                hashmap!{
                    name("content-type") => values("multipart/bar; boundary=xyz"),
                },
            ).multipart_contenttype(),
            Some(
                MultipartContentType {
                    multipart_type: "bar",
                    boundary: Some("xyz"),
                }
            )
        );
    }

    #[test]
    fn parts_should_find_single_text_part() {
        assert_eq!(
            requestb(
                hashmap!{
                    name("content-type") => values("multipart/form-data; boundary=xyz"),
                },
                indoc!{"
                    --xyz
                    Content-Disposition: form-data; name=part1

                    content
                    --xyz--
                "}.as_bytes().into(),
            ).parts(),
            vec![
                Part::from("Content-Disposition: form-data; name=part1\n\ncontent\n"),
            ],
        );
    }

    #[test]
    fn parts_should_find_two_text_parts() {
        assert_eq!(
            requestb(
                hashmap!{
                    name("content-type") => values("multipart/form-data; boundary=xyz"),
                },
                indoc!{r#"
                    --xyz
                    Content-Disposition: form-data; name=part1

                    content
                    --xyz
                    Content-Disposition: form-data; name="file"; filename="Cargo.toml"
                    Content-Type: plain/text

                    [workspace]
                    members = [
                        "fhttp",
                        "fhttp-core",
                    ]

                    --xyz--
                "#}.as_bytes().into(),
            ).parts(),
            vec![
                Part::from("Content-Disposition: form-data; name=part1\n\ncontent\n"),
                Part::from(indoc!{r#"
                    Content-Disposition: form-data; name="file"; filename="Cargo.toml"
                    Content-Type: plain/text

                    [workspace]
                    members = [
                        "fhttp",
                        "fhttp-core",
                    ]

                "#}),
            ],
        );
    }
}
