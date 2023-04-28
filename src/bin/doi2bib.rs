use std::env;
use std::process;

fn print_usage(program: &str) {
    println!("Usage: {} doi [doi...]", program);
    println!("\t-h  show this message");
    println!("\t-v  show version");
}

fn print_version() {
    println!("doi2bib 1.1");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut do_help = false;
    let mut do_version = false;

    let mut doi_list = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" => do_help = true,
            "-v" => do_version = true,
            _ => doi_list.push(args[i].clone()),
        }
        i += 1;
    }

    if do_help {
        print_usage(&program);
        return;
    }

    if do_version {
        print_version();
        return;
    }

    if doi_list.is_empty() {
        eprintln!("No DOI provided");
        print_usage(&program);
        process::exit(1);
    }

    let http_headers = "Accept: text/bibliography; style=bibtex";

    for doi in doi_list {
        // We do some encoding as needed.
        let doi = doi.replace("+", "%2B");
        println!("{}", http_headers);
        println!();

        // We retrieve the data from https://doi.org/
        match reqwest::blocking::Client::new()
            .get(&format!("https://doi.org/{}", doi))
            .header("Accept", http_headers)
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
