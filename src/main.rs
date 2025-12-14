use anyhow::{self, Context};
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use trustami::os_interaction;
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
    CreateIndex {
        #[arg(help = "Name of the new index")]
        index_name: String,
        #[arg(help="Directory to index", default_value=utils::get_current_directory())]
        dir_path: PathBuf,
    },
    ListIndexes,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let user_data_directory =
        dirs::data_local_dir().context("Couldn't find the user's data directory.")?;

    match &cli.command {
        Command::Query {
            query_string,
            dir_path,
        } => {
            // TODO: point to correct index path
            let index: Index;

            if let Ok(mut index_file_handle) = File::open("index.json") {
                // index was found, load it
                let mut buf = String::new();
                index_file_handle.read_to_string(&mut buf)?;
                index = serde_json::from_str(&buf)?;
            } else {
                // TODO: build index similarly to create index command
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
            view::present_results_cli(results);
            Ok(())
        }
        Command::CreateIndex {
            dir_path,
            index_name,
        } => {
            let mut input = std::io::stdin().lock();
            let mut file_handle =
                os_interaction::create_index_file(user_data_directory, index_name, &mut input)?;
            let file_paths = path_resolver::collect_valid_paths(dir_path);
            let new_index = utils::index_docs(&file_paths);
            let serialized = serde_json::to_string(&new_index)
                .context("Failed to serialize newly created index.")?;
            file_handle
                .write_all(serialized.as_bytes())
                .context("Failed to write index data to file.")?;
            Ok(())
        }
        Command::ListIndexes => {
            todo!("Implement indexes name listing");
        }
    }
}
