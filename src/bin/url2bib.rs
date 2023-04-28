use std::process;
use std::str::FromStr;

use clap::Parser;
use itertools::Itertools;
use itertools::Position;
use nom_bibtex::Bibliography;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::Url;

/// Retrieve BibTeX data for a given URL.
#[derive(Clone, Debug, Parser)]
#[command(author, version, about, arg_required_else_help(true))]
struct Cli {
    /// URL to retrieve BibTeX data for.
    #[arg(value_parser(Url::from_str))]
    url: Vec<Url>,
}

fn main() {
    let cli = Cli::parse();

    for url in cli.url {
        match Client::new()
            .get(url.as_str())
            .send()
            .and_then(Response::text)
        {
            Ok(html) => {
                let title: String = todo!("Hello world!");
                let slug: String = todo!("hello-world");
                let today: String = todo!("%Y-%m-%d");
                let biblio = Bibliography::new("misc".into(), slug, vec![
                    ("title".into(), title),
                    ("howpublished".into(), format!(r"\url{{{}}}", url)),
                    ("note".into(), format!("Accessed: {}", today)),
                ]);
                print(&biblio);
            }
            Err(e) => {
                eprintln!("Error retrieving URL {url}: {e}");
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
