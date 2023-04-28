//! A strongly-typed library for working
//! with
//! [Digital object identifiers](https://en.wikipedia.org/wiki/Digital_object_identifier)
//! (DOIs).

use nutype::nutype;
use once_cell::sync::Lazy;
use regex::Regex;
use regex::RegexBuilder;

/// Regex for
/// [Digital object identifier](https://en.wikipedia.org/wiki/Digital_object_identifier)
/// (DOI)
/// validation.
///
/// It is taken from
/// <https://stackoverflow.com/a/48524047/4039050>,
/// which has been
/// [recommended by CrossRef](https://www.crossref.org/blog/dois-and-matching-regular-expressions/).
static REGEX: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(r"^10.\d{4,9}/[-._;()/:A-Z0-9]+$")
        .case_insensitive(true)
        .build()
        .expect("should be valid")
});

/// A
/// [Digital object identifier](https://en.wikipedia.org/wiki/Digital_object_identifier)
/// (DOI).
#[nutype(
    sanitize(trim, lowercase, with = sanitize_doi)
    validate(not_empty, regex = REGEX)
)]
#[derive(*)]
#[derive(Display)]
pub struct Doi(String);

use url::Url;

/// Prefix URL for retrieving information about
/// [Digital object identifiers](https://en.wikipedia.org/wiki/Digital_object_identifier)
/// (DOIs).
static URL: Lazy<Url> = Lazy::new(|| Url::parse("https://doi.org/").expect("should be parseable"));

/// Sanitize a
/// [Digital object identifier](https://en.wikipedia.org/wiki/Digital_object_identifier)
/// (DOI).
#[inline]
#[allow(clippy::needless_pass_by_value)]
fn sanitize_doi(doi: String) -> String {
    [URL.as_str(), "doi:"]
        .into_iter()
        .fold(doi.as_str(), |doi, prefix| {
            doi.strip_prefix(prefix).unwrap_or(doi)
        })
        .into()
}

impl From<Doi> for Url {
    #[inline]
    fn from(doi: Doi) -> Self {
        Self::from(&doi)
    }
}

impl From<&Doi> for Url {
    #[inline]
    fn from(doi: &Doi) -> Self {
        URL.join(doi.as_ref()).expect("should be joinable")
    }
}
