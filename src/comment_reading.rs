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


pub fn get_comments_from_file(path: &String) -> Vec<Comment> {
    let re = Regex::new(r"https://vk.com/wall(-?\d+)_\d+\?reply=(\d+)").unwrap();
    let mut result = Vec::new();

    let html = read_file_contents(path);

    for capture in re.captures_iter(&html) {
        result.push(Comment {
            owner_id: capture[1].to_string(),
            comment_id: capture[2].to_string(),
        });
    }

    result
}
