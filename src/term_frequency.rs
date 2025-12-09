use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Default)]
pub struct TermFrequency {
    pub document_path: PathBuf,
    pub term_freq: HashMap<String, u32>,
}

impl TermFrequency {
    pub fn new(document_path: PathBuf) -> Self {
        Self {
            document_path,
            term_freq: HashMap::new(),
        }
    }

    pub fn update(&mut self, term: &str) {
        if let Some(count) = self.term_freq.get_mut(term) {
            *count += 1;
        } else {
            self.term_freq.insert(term.to_string(), 1);
        }
    }
}
