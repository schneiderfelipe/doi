use std::fmt;
use std::str::FromStr;

use itertools::Itertools;
use nutype::nutype;
use thiserror::Error;
use url::Url;

#[derive(Clone, Debug)]
pub struct Doi {
    prefix: DoiPrefix,
    suffix: DoiSuffix,
}

#[derive(Clone, Debug, Error)]
pub enum DoiParseError {
    #[error("slash not found")]
    SlashNotFound,

    #[error("not in DOI namespace")]
    NotInNamespace,

    #[error("failed to parse prefix subdivision: {0}")]
    PrefixSubdivision(#[from] DoiPrefixSubdivisionError),

    #[error("failed to parse suffix: {0}")]
    Suffix(#[from] DoiSuffixError),
}

impl FromStr for Doi {
    type Err = DoiParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix(DOI_ORG).unwrap_or(s);
        let s = s.strip_prefix("doi:").unwrap_or(s);

        let (prefix, suffix) = s.split_once('/').ok_or(DoiParseError::SlashNotFound)?;
        Ok(Self {
            prefix: prefix.parse()?,
            suffix: suffix.parse()?,
        })
    }
}

#[derive(Clone, Debug)]
struct DoiPrefix {
    subdivisions: Vec<DoiPrefixSubdivision>,
}

#[nutype(
    sanitize(trim, lowercase)
    validate(not_empty)
)]
#[derive(AsRef, Clone, Debug, Display, FromStr)]
struct DoiPrefixSubdivision(String);

impl FromStr for DoiPrefix {
    type Err = DoiParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix(DOI_NAMESPACE)
            .ok_or(DoiParseError::NotInNamespace)?;
        let subdivisions: Result<_, _> = s.split('.').map(str::parse).collect();
        let subdivisions = subdivisions?;
        Ok(Self { subdivisions })
    }
}

#[nutype(
    sanitize(trim, lowercase)
    validate(not_empty)
)]
#[derive(AsRef, Clone, Debug, Display, FromStr)]
struct DoiSuffix(String);

impl fmt::Display for Doi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.prefix, self.suffix)
    }
}

impl fmt::Display for DoiPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{DOI_NAMESPACE}")?;
        self.subdivisions
            .iter()
            .map(AsRef::as_ref)
            .intersperse(".")
            .try_for_each(|s| write!(f, "{s}"))
    }
}

const DOI_NAMESPACE: &str = "10.";
const DOI_ORG: &str = "https://doi.org/";

impl From<Doi> for Url {
    fn from(doi: Doi) -> Self {
        Self::parse(&format!("{DOI_ORG}{doi}")).expect("DOI should always be translatable to URL")
    }
}

impl From<&Doi> for Url {
    fn from(doi: &Doi) -> Self {
        Self::parse(&format!("https://doi.org/{doi}"))
            .expect("DOI should always be translatable to URL")
    }
}
