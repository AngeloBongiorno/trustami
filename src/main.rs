use std::fs::File;
use std::io::Read;
use trustami::parsers;
use trustami::tokenizer::Tokenizer;
use trustami::term_frequency::TermFrequency;
use trustami::path_resolver;
use trustami::inverse_doc_frequency::InverseDocumentFrequency;
use trustami::utils;

fn main() {

    let data_dir_path = "./data";
    let file_paths = path_resolver::collect_valid_paths(data_dir_path);

    println!("{:?}", file_paths);

    let mut tf_docs = vec![];

    for file_path in file_paths {
        let mut file_handle = File::open(file_path).unwrap();

        let mut input_data = String::new();
        let _ = file_handle.read_to_string(&mut input_data).unwrap();

        let txt = parsers::parse_xml_string(input_data);

        let chars: Vec<char> = txt.chars().collect();
        let tokenizer = Tokenizer::from_chars(&chars);

        let mut tf = TermFrequency::default();

        // compute TF for doc
        for token in tokenizer {
            tf.update(&token);
        }

        tf_docs.push(tf);
    }

    // update IDF
    let mut idf = InverseDocumentFrequency::default();
    for tf_doc in &tf_docs {
        for key in tf_doc.0.keys() {
            idf.update(key, &tf_docs); 
        }
    }

    // COMPUTE TF IDF
    for tf_doc in tf_docs {
        let tfidf = utils::compute_tf_idf("rome", &tf_doc, &idf);
        println!("{:?}", tfidf);
    }
}
