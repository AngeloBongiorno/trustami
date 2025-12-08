use std::fs::File;
use std::io::Read;
use trustami::parsers;
use trustami::tokenizer::Tokenizer;
use trustami::term_frequency::TermFrequency;
use trustami::inverse_doc_frequency::InverseDocumentFrequency;



fn main() {
    let file_paths = vec![
        "./data/italy.xml",
        "./data/comp_sci.xml",
        "./data/ai.xml",
        "./data/machine_learning.xml",
    ];

    let mut docs = vec![];

    for file_path in file_paths {
        let mut file_handle = File::open(file_path).unwrap();

        let mut input_data = String::new();
        let _ = file_handle.read_to_string(&mut input_data).unwrap();

        let txt = parsers::parse_xml_string(input_data);

        let chars: Vec<char> = txt.chars().collect();
        let tokenizer = Tokenizer::from_chars(&chars);

        let mut tf = TermFrequency::default();

        for token in tokenizer {
            tf.update(&token);
        }

        docs.push(tf);
    }


    let mut idf = InverseDocumentFrequency::default();
    idf.update("is", &mut docs); 
    idf.update("Rome", &mut docs);
    println!("{:?}", idf);
}
