use std::fs::File;
use std::io::Read;
use trustami::parsers;
use trustami::tokenizer::Tokenizer;
use trustami::term_frequency::TermFrequency;
use trustami::path_resolver;
use trustami::inverse_doc_frequency::InverseDocumentFrequency;
use trustami::utils::TfIdf;
use trustami::view;

fn main() {

    let data_dir_path = "./data";
    let file_paths = path_resolver::collect_valid_paths(data_dir_path);

    let mut tf_docs = vec![];

    for file_path in file_paths {
        let mut file_handle = File::open(&file_path).unwrap();

        let mut input_data = String::new();
        let _ = file_handle.read_to_string(&mut input_data).unwrap();

        let txt = parsers::parse_xml_string(input_data);

        let chars: Vec<char> = txt.chars().collect();
        let tokenizer = Tokenizer::from_chars(&chars);

        let mut tf = TermFrequency::new(file_path);

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

    let mut results: Vec<TfIdf> = Vec::new();

    // COMPUTE TF IDF
    for tf_doc in tf_docs {
        let tfidf = TfIdf::new("rome", &tf_doc, &idf).expect("term caused idf error");
        results.push(tfidf);
    }

    view::present_results_cli(results);

}
