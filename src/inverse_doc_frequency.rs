use crate::term_frequency::TermFrequency;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InverseDocumentFrequency(HashMap<String, f32>);

impl InverseDocumentFrequency {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get_inner_map(&self) -> &HashMap<String, f32> {
        &self.0
    }

    pub fn update(&mut self, term: &str, docs: &Vec<TermFrequency>) {
        if !self.0.contains_key(term) {
            // one is added for smoothing
            let docs_count = docs.len() + 1;

            let mut matched_docs_count = 0;
            for doc in docs {
                if doc.term_freq.contains_key(term) {
                    matched_docs_count += 1;
                }
            }
            // one is added for smoothing
            matched_docs_count += 1;

            let idf_value = (docs_count / matched_docs_count) as f32;
            let _ = self.0.insert(term.to_string(), idf_value);
        }
    }
}
