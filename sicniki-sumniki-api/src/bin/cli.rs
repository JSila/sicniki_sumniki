use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path;

use clap::Clap;

use sicniki_sumniki::Repository;

#[derive(Clap)]
struct Opts {
    src_file: path::PathBuf
}

fn main() {
    dotenv::dotenv().unwrap();
    let db_url = std::env::var("DATABASE_URL").unwrap();

    let opts: Opts = Opts::parse();
    let file = File::open(opts.src_file).unwrap();

    let repo = Repository::new(&db_url);

    for (n, line) in BufReader::new(file).lines().enumerate() {
        if line.is_err() {
            continue
        }

        let word = line.unwrap();

        if word.split_whitespace().count() > 1 {
            continue
        }

        if !word.contains('s') && !word.contains('c') && !word.contains('z') && !word.contains('š') && !word.contains('č') && !word.contains('ž') {
            continue
        }

        if let Err(e) = repo.insert_new_word(&word) {
            eprintln!("Cannot insert a word: {}", e.to_string());
        };
        if n%30_000 == 0 {
            println!("{} words processed.", n)
        }
    }
}