use quick_xml::events::Event;
use quick_xml::reader::Reader;

pub fn parse_xml_string(input: String) -> String {
    let mut reader = Reader::from_str(&input);
    reader.config_mut().trim_text(true);
    let mut buffer = Vec::new();
    let mut txt = String::new();
    loop {
        let event = reader
            .read_event_into(&mut buffer)
            .expect("Failed to read XML");
        match event {
            Event::Eof => break,
            Event::Text(e) => txt.push_str(&e.decode().unwrap()),
            _ => (),
        }
    }
    txt
}
