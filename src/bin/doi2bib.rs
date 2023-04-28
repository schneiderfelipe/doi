use std::process;

use clap::Parser;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::header;

/// Retrieve BibTeX data for a given DOI.
#[derive(Parser)]
#[command(author, version, about, arg_required_else_help(true))]
struct Cli {
    /// DOI to retrieve BibTeX data for.
    doi: Vec<Doi>,
}

type Doi = String;

fn main() {
    let cli = Cli::parse();

    for doi in cli.doi {
        match Client::new()
            .get(&format!("https://doi.org/{doi}"))
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
