#[cfg(test)] extern crate indoc;
#[cfg(test)] extern crate maplit;
extern crate wiremock;

mod number_of_parts_matcher;
mod header_utils;
mod part;

#[cfg(test)]
mod test_utils {
    use std::collections::HashMap;
    use std::str::FromStr;

    use wiremock::http::{HeaderName, HeaderValue, HeaderValues, Method, Url};
    use wiremock::Request;

    pub fn name(name: &'static str) -> HeaderName {
        HeaderName::from_str(name).unwrap()
    }

    pub fn value(val: &'static str) -> HeaderValue {
        HeaderValue::from_str(val).unwrap()
    }

    pub fn values(val: &'static str) -> HeaderValues {
        value(val).into()
    }

    pub fn request(
        headers: HashMap<HeaderName, HeaderValues>,
    ) -> Request {
        requestb(headers, vec![])
    }

    pub fn requestb(
        headers: HashMap<HeaderName, HeaderValues>,
        body: Vec<u8>,
    ) -> Request {
        Request {
            url: Url::from_str("http://localhost").unwrap(),
            method: Method::Post,
            headers,
            body,
        }
    }
}

