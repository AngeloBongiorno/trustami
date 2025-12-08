use std::collections::HashMap;
use crate::term_frequency::TermFrequency;

#[derive(Debug, Default)]
pub struct InverseDocumentFrequency(pub HashMap<String, f32>);

impl InverseDocumentFrequency {

    pub fn update(&mut self, term: &str, docs: &Vec<TermFrequency>) {
        if !self.0.contains_key(term) {
            let docs_count = docs.len();

            let mut matched_docs_count = 0;

            for doc in docs {
                if doc.0.contains_key(term) {
                    matched_docs_count += 1;
                }
            }
            let idf_value = (docs_count / matched_docs_count) as f32;
            let _ = self.0.insert(term.to_string(), idf_value);
        }
    }
}
