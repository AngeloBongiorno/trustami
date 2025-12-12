use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::inverse_doc_frequency::InverseDocumentFrequency;
use crate::parsers;
use crate::term_frequency::TermFrequency;
use crate::tokenizer::Tokenizer;

#[derive(Debug)]
pub struct TfIdf {
    pub document_path: PathBuf,
    pub score: f32,
}

impl TfIdf {
    pub fn new(
        term: &str,
        doc: &TermFrequency,
        idf: &InverseDocumentFrequency,
        docs_count: usize,
    ) -> Self {
        let lowercase_term = term.to_lowercase();
        let term_freq = doc.term_freq.get(&lowercase_term).copied().unwrap_or(0);

        //let inverse_doc_freq = idf.0.get(&lowercase_term).ok_or("Term is not in IDF index.")?;

        // defaults to 1 if the term does not exist in the corups
        let smoothing_default = (1 + docs_count) as f32;
        let inverse_doc_freq = idf
            .get_inner_map()
            .get(&lowercase_term)
            .unwrap_or(&smoothing_default);
        //let inverse_doc_freq = idf.0.get(&lowercase_term).unwrap_or(&smoothing_default);

        let score = term_freq as f32 * inverse_doc_freq.log10();
        Self {
            document_path: doc.document_path.clone(),
            score,
        }
    }
}

impl std::fmt::Display for TfIdf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\tScore: {:.2}",
            self.document_path.display(),
            self.score
        )
    }
}

pub fn get_current_directory() -> OsString {
    current_dir().unwrap().into_os_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub term_frequencies: Vec<TermFrequency>,
    pub inverse_document_frequency: InverseDocumentFrequency,
}

pub fn index_docs(file_paths: &Vec<PathBuf>) -> Index {
    let mut tf_docs = vec![];
    for file_path in file_paths {
        let mut file_handle = File::open(file_path).unwrap();
        let mut input_data = String::new();
        let _ = file_handle.read_to_string(&mut input_data).unwrap();

        let txt = parsers::parse_xml_string(input_data);

        let chars: Vec<char> = txt.chars().collect();
        let tokenizer = Tokenizer::from_chars(&chars);

        let mut tf = TermFrequency::new(file_path.to_path_buf());

        // compute TF for doc
        for token in tokenizer {
            tf.update(&token);
        }

        tf_docs.push(tf);
    }
    // update IDF
    let mut idf = InverseDocumentFrequency::default();
    for tf_doc in &tf_docs {
        for key in tf_doc.term_freq.keys() {
            idf.update(key, &tf_docs);
        }
    }

    Index {
        term_frequencies: tf_docs,
        inverse_document_frequency: idf,
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::utils::TfIdf;

    #[test]
    fn formatting_works() {
        let tf_idf = TfIdf {
            document_path: PathBuf::from_str("a/path/buf").unwrap(),
            score: 12.36163,
        };

        let result = format!("{}", tf_idf);
        let expected = String::from_str("a/path/buf\n\tScore: 12.36").unwrap();

        assert_eq!(result, expected)
    }
}
