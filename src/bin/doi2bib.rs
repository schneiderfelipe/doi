use std::process;

use clap::Parser;

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
        // We do some encoding as needed.
        let doi = doi.replace('+', "%2B");

        // We retrieve the data from https://doi.org/
        match reqwest::blocking::Client::new()
            .get(&format!("https://doi.org/{doi}"))
            .header(reqwest::header::ACCEPT, "text/bibliography; style=bibtex")
            .send()
        {
            Ok(resp) => {
                let bibtex_data = resp
                    .text()
                    .unwrap()
                    .replace(", ", ",\n  ")
                    .replace("}, ", "},\n  ")
                    .replace(",\n  ", ", ")
                    .replace("}}", "}\n}\n");
                println!("{bibtex_data}");
            }
            Err(e) => {
                eprintln!("Error retrieving DOI {doi}: {e}");
                process::exit(1);
            }
        }
    }
}
