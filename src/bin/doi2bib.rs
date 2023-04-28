use std::process;

use clap::App;
use clap::AppSettings;
use clap::Arg;

fn print_version() {
    println!("doi2bib 1.1");
}

fn main() {
    let matches = App::new("doi2bib")
        .version("1.1")
        .setting(AppSettings::ArgRequiredElseHelp)
        .about("Retrieves BibTeX data for a given DOI")
        .arg(
            Arg::with_name("doi")
                .multiple(true)
                .required(true)
                .help("The DOI to retrieve BibTeX data for"),
        )
        .get_matches();

    if matches.is_present("version") {
        print_version();
        return;
    }

    for doi in matches.values_of("doi").unwrap() {
        // We do some encoding as needed.
        let doi = doi.replace('+', "%2B");

        // We retrieve the data from https://doi.org/
        match reqwest::blocking::Client::new()
            .get(&format!("https://doi.org/{}", doi))
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
                println!("{}", bibtex_data);
            }
            Err(e) => {
                eprintln!("Error retrieving DOI {}: {}", doi, e);
                process::exit(1);
            }
        }
    }
}
