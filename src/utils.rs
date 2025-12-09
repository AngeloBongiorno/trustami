use std::env::current_dir;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::inverse_doc_frequency::InverseDocumentFrequency;
use crate::term_frequency::TermFrequency;

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
