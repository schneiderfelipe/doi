use std::process;
use std::str::FromStr;

use clap::Parser;
use doi::Doi;
use either::Either;
use itertools::Itertools;
use itertools::Position;
use nom_bibtex::Bibliography;
use nom_bibtex::Bibtex;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::header;
use reqwest::Url;

/// Retrieve BibTeX data for a given DOI.
#[derive(Clone, Debug, Parser)]
#[command(author, version, about, arg_required_else_help(true))]
struct Cli {
    /// DOI to retrieve BibTeX data for.
    #[arg(value_parser(Doi::from_str))]
    doi: Vec<Doi>,
}

fn main() {
    let cli = Cli::parse();

    for doi in cli.doi {
        match Client::new()
            .get(Url::from(&doi))
            .header(header::ACCEPT, "text/bibliography; style=bibtex")
            .send()
            .and_then(Response::text)
            .map_err(Either::Left)
            .and_then(|text| Bibtex::parse(&text).map_err(Either::Right))
        {
            Ok(bibtex) => {
                bibtex.bibliographies().iter().for_each(|biblio| {
                    print(biblio);
                });
            }
            Err(e) => {
                eprintln!("Error retrieving DOI {doi}: {e}");
                process::exit(1);
            }
        }
    }
}

fn print(biblio: &Bibliography) {
    println!("@{}{{{},", biblio.entry_type(), biblio.citation_key());
    biblio.tags().iter().with_position().for_each(|item| {
        match item {
            Position::First((key, value)) | Position::Middle((key, value)) => {
                println!("  {key:9} = {{{value}}},");
            }
            Position::Last((key, value)) | Position::Only((key, value)) => {
                println!("  {key:9} = {{{value}}}");
            }
        }
    });
    println!("}}\n");
}
