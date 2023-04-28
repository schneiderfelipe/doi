use std::fmt;
use std::process;
use std::str::FromStr;

use clap::Parser;
use itertools::Itertools;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::header;
use thiserror::Error;
use url::Url;

/// Retrieve BibTeX data for a given DOI.
#[derive(Parser)]
#[command(author, version, about, arg_required_else_help(true))]
struct Cli {
    /// DOI to retrieve BibTeX data for.
    #[arg(value_parser(Doi::from_str))]
    doi: Vec<Doi>,
}

#[derive(Clone)]
struct Doi {
    prefix: DoiPrefix,
    suffix: DoiSuffix,
}

#[derive(Debug, Error)]
enum DoiParseError {
    #[error("slash not found")]
    SlashNotFound,

    #[error("not in DOI namespace")]
    NotInDoiNamespace,
}

impl FromStr for Doi {
    type Err = DoiParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix(DOI_ORG).unwrap_or(s);
        let s = s.strip_prefix("doi:").unwrap_or(s);

        let (prefix, suffix) = s.split_once('/').ok_or(DoiParseError::SlashNotFound)?;
        Ok(Self {
            prefix: prefix.parse()?,
            suffix: suffix.to_lowercase(),
        })
    }
}

#[derive(Clone)]
struct DoiPrefix {
    subdivisions: Vec<String>,
}

impl FromStr for DoiPrefix {
    type Err = DoiParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("10.")
            .ok_or(DoiParseError::NotInDoiNamespace)?;
        let subdivisions = s.split('.').map(str::to_lowercase).collect();
        Ok(Self { subdivisions })
    }
}

type DoiSuffix = String;

impl fmt::Display for Doi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.prefix, self.suffix)
    }
}

impl fmt::Display for DoiPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.subdivisions
            .iter()
            .map(std::string::String::as_str)
            .intersperse(".")
            .try_for_each(|s| write!(f, "{s}"))
    }
}

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

fn main() {
    let cli = Cli::parse();

    for doi in cli.doi {
        match Client::new()
            .get(Url::from(&doi))
            .header(header::ACCEPT, "text/bibliography; style=bibtex")
            .send()
            .and_then(Response::text)
        {
            Ok(bibtex_data) => {
                println!("{bibtex_data}");
            }
            Err(e) => {
                eprintln!("Error retrieving DOI {doi}: {e}");
                process::exit(1);
            }
        }
    }
}
