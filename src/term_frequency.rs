use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TermFrequency(pub HashMap<String, u32>);

impl TermFrequency {

    pub fn update(&mut self, term: &str) {
        if let Some(count) = self.0.get_mut(term) {
            *count += 1;
        } else {
            self.0.insert(term.to_string(), 1);
        }
    }
}
