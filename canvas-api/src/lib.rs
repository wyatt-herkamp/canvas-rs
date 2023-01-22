use std::str::FromStr;
use thiserror::Error;

pub mod courses;
#[cfg(test)]
pub(crate) mod test;

#[derive(Error, Debug)]
#[error("Invalid Pagination Response")]
pub struct InvalidPaginationError;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pagination {
    pub current: String,
    pub first: String,
    pub last: String,
    pub next: Option<String>,
}
impl FromStr for Pagination {
    type Err = InvalidPaginationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut current = String::new();
        let mut first = String::new();
        let mut last = String::new();
        let mut next = None;
        for url in value.split(',') {
            let parts: Vec<&str> = url.split(';').collect();
            let link = parts[0].trim();
            let rel = {
                let mut rel = parts[1].trim();
                if rel.starts_with("rel=") {
                    rel = &rel[4..];
                } else {
                    return Err(InvalidPaginationError);
                }
                &rel[1..rel.len() - 1]
            };

            // Remove < and > from link
            let link = link[1..link.len() - 1].to_string();
            match rel {
                "current" => current = link,
                "first" => first = link,
                "last" => last = link,
                "next" => next = Some(link),
                _ => {}
            }
        }
        if current.is_empty() || first.is_empty() || last.is_empty() {
            return Err(InvalidPaginationError);
        }
        Ok(Pagination {
            current,
            first,
            last,
            next,
        })
    }
}
#[test]
pub fn test_pagination_parse() {
    let test = r#"
    <https://<canvas>/api/v1/courses/:id/discussion_topics.json?opaqueA>; rel="current",
      <https://<canvas>/api/v1/courses/:id/discussion_topics.json?opaqueB>; rel="next",
      <https://<canvas>/api/v1/courses/:id/discussion_topics.json?opaqueC>; rel="first",
      <https://<canvas>/api/v1/courses/:id/discussion_topics.json?opaqueD>; rel="last"
      "#;
    let pagination = Pagination::from_str(test).unwrap();
    assert_eq!(
        pagination.current,
        "https://<canvas>/api/v1/courses/:id/discussion_topics.json?opaqueA"
    );
    assert_eq!(
        pagination.first,
        "https://<canvas>/api/v1/courses/:id/discussion_topics.json?opaqueC"
    );
    assert_eq!(
        pagination.last,
        "https://<canvas>/api/v1/courses/:id/discussion_topics.json?opaqueD"
    );
    assert_eq!(
        pagination.next,
        Some("https://<canvas>/api/v1/courses/:id/discussion_topics.json?opaqueB".to_string())
    );
}
