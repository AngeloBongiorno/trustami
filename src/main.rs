use clap::{Parser, Subcommand};
use std::error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use trustami::path_resolver;
use trustami::utils::{self, Index, TfIdf};
use trustami::view;

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Search term in the specified directory
    Query {
        //#[arg(help = "One or more terms to search")]
        query_string: String,
        #[arg(help="Directory to search in", default_value=utils::get_current_directory())]
        dir_path: PathBuf,
    },
    /// Index the documents in the specified directory
    Index {
        #[arg(help="Directory to index in", default_value=utils::get_current_directory())]
        dir_path: PathBuf,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("ERROR: {err}.");
    };
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Query {
            query_string,
            dir_path,
        } => {
            let index: Index;

            if let Ok(mut index_file_handle) = File::open("index.json") {
                // index was found, load it
                dbg!("Index was found!");
                let mut buf = String::new();
                index_file_handle.read_to_string(&mut buf)?;
                index = serde_json::from_str(&buf)?;
            } else {
                // build index
                dbg!("Index was not found!");
                let file_paths = path_resolver::collect_valid_paths(dir_path);
                index = utils::index_docs(&file_paths);
            }

            let Index {
                term_frequencies,
                inverse_document_frequency,
            } = index;
            let mut results: Vec<TfIdf> = Vec::new();

            // COMPUTE TF IDF
            for tf_doc in &term_frequencies {
                let tfidf = TfIdf::new(
                    query_string,
                    tf_doc,
                    &inverse_document_frequency,
                    term_frequencies.len(),
                );
                results.push(tfidf);
            }
            dbg!("presenting results...");
            view::present_results_cli(results);
            Ok(())
        }
        Command::Index { dir_path } => {
            let mut file_handle = File::create_new("index.json")?;
            let file_paths = path_resolver::collect_valid_paths(dir_path);
            let new_index = utils::index_docs(&file_paths);
            let serialized = serde_json::to_string(&new_index)?;
            file_handle.write_all(serialized.as_bytes())?;
            Ok(())
        }
    }
}
