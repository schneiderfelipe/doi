//! A strongly-typed library for working
//! with
//! [Digital object identifiers](https://en.wikipedia.org/wiki/Digital_object_identifier).
//! (DOIs).

use nutype::nutype;
use once_cell::sync::Lazy;
use url::Url;

/// Prefix URL for a
/// [Digital object identifier](https://en.wikipedia.org/wiki/Digital_object_identifier).
static URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://doi.org/").expect("URL should be parseable"));

/// Namespace of a
/// [Digital object identifier](https://en.wikipedia.org/wiki/Digital_object_identifier).
const NAMESPACE: &str = "10.";

/// A
/// [Digital object identifier](https://en.wikipedia.org/wiki/Digital_object_identifier).
#[nutype(
    sanitize(trim, lowercase, with = sanitize_doi)
    validate(not_empty, with = validate_doi)
)]
#[derive(*)]
#[derive(Display)]
pub struct Doi(String);

#[allow(clippy::map_unwrap_or)]
fn sanitize_doi(doi: String) -> String {
    [URL.as_str(), "doi:"].into_iter().fold(doi, |doi, prefix| {
        doi.strip_prefix(prefix).map(Into::into).unwrap_or(doi)
    })
}

fn validate_doi(doi: &str) -> bool {
    doi.starts_with(NAMESPACE) && doi.contains('/')
}

impl From<Doi> for Url {
    fn from(doi: Doi) -> Self {
        Self::from(&doi)
    }
}

impl From<&Doi> for Url {
    fn from(doi: &Doi) -> Self {
        URL.join(doi.as_ref())
            .expect("DOI should always be translatable to URL")
    }
}
