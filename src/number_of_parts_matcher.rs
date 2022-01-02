use wiremock::{Match, Request};

use crate::header_utils::RequestUtils;

pub struct NumberOfPartsMatcher(usize);

impl Match for NumberOfPartsMatcher {
    fn matches(&self, request: &Request) -> bool {
        request.parts().len() == self.0
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use maplit::hashmap;

    use crate::test_utils::*;

    use super::*;

    #[test]
    fn should_compare_number_of_parts_with_expectation() {
        let request = requestb(
            hashmap!{
                name("content-type") => values("multipart/form-data; boundary=xyz"),
            },
        indoc!{"
                --xyz
                Content-Disposition: form-data; name=part1

                content
                --xyz--
            "}.as_bytes().into(),
        );

        assert_eq!(NumberOfPartsMatcher(0).matches(&request), false);
        assert_eq!(NumberOfPartsMatcher(1).matches(&request), true);
        assert_eq!(NumberOfPartsMatcher(2).matches(&request), false);
    }
}
