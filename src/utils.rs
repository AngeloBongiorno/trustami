use crate::inverse_doc_frequency::InverseDocumentFrequency;
use crate::term_frequency::TermFrequency;

pub fn compute_tf_idf(term: &str, doc: &TermFrequency, idf: &InverseDocumentFrequency) -> Option<f32> {
    let lowercase_term = term.to_lowercase();
    let term_freq = doc.0.get(&lowercase_term).copied().unwrap_or(0);
    let inverse_doc_freq = idf.0.get(&lowercase_term)?;

    Some(term_freq as f32 * inverse_doc_freq.log10())
}
