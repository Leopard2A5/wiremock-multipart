//! wiremock-multipart adds matchers for use with [wiremock](https://crates.io/crates/wiremock)
//! to check multipart characteristics of requests.
//!
//! ## How to install
//! Add `wiremock-multipart` to your dev-dependencies:
//! ```toml
//! [dev-dependencies]
//! # ...
//! wiremock-multipart = "0.1"
//! ```
//!
//! ## Getting started
//!
//! ```rust
//! use wiremock::{MockServer, Mock, ResponseTemplate};
//! use wiremock::matchers::method;
//! use wiremock_multipart::prelude::*;
//!
//! #[async_std::main]
//! async fn main() {
//!     // Start a background HTTP server on a random local port
//!     let mock_server = MockServer::start().await;
//!
//!     // Arrange the behaviour of the MockServer adding a Mock
//!     Mock::given(method("POST"))
//!         .and(NumberOfParts(2))
//!         .respond_with(ResponseTemplate::new(200))
//!         // Mounting the mock on the mock server - it's now effective!
//!         .mount(&mock_server)
//!         .await;
//!
//!     // if we now send a multipart/form-data request with two parts to it, the request
//!     // will match and return 200.
//! }
//! ```

#[cfg(test)] extern crate indoc;
#[cfg(test)] extern crate maplit;
extern crate wiremock;

pub mod number_of_parts;
mod header_utils;
mod part;

pub mod prelude {
    pub use crate::number_of_parts::NumberOfParts;
}

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

