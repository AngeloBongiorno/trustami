use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use trustami::tokenizer::Tokenizer;
use trustami::parser;


type TermFrequency = HashMap<String, u16>;

fn main() {
    let file_path = "./data/italy.xml";
    let mut file_handle = File::open(file_path).unwrap();

    let mut input_data = String::new();
    let _ = file_handle.read_to_string(&mut input_data).unwrap();

    let txt = parser::parse_xml_string(input_data);

    let chars: Vec<char> = txt.chars().collect();
    let tokenizer = Tokenizer::from_chars(&chars);

    let mut tf = TermFrequency::new();

    for token in tokenizer {
        if let Some(count) = tf.get_mut(&token) {
            *count += 1;
        } else {
            tf.insert(token, 1);
        }
    }

    println!("There are {} terms in the doc", tf.len());
}


