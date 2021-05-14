use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct Comment {
    pub owner_id: String,
    pub comment_id: String,
}

fn read_file_contents(path: &String) -> String {
    let file = File::open(path).expect("Unable to open the file");
    let mut reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1251))
            .build(file),
    );

    let mut result = String::new();
    reader.read_to_string(&mut result).unwrap();

    result
}
