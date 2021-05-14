use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use std::collections::HashSet;
use std::fs::{read_dir, File};
use std::io::{BufReader, Read};

#[derive(PartialEq, Eq, Hash)]
pub struct Comment {
    pub owner_id: String,
    pub comment_id: String,
}

pub fn get_list_of_files(folder_path: &String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let paths = read_dir(folder_path).unwrap();
    let re = Regex::new(r"comments(\d{0,10}).html").unwrap(); // Someone teach me regex...

    for path in paths {
        let path_str = path.unwrap().path().display().to_string();
        // if current file has form of `comments.html` or `comments<any_number_here>.html`
        if re.is_match(&path_str) {
            result.push(path_str);
        }
    }

    result
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

fn get_comments_from_file(path: &String) -> Vec<Comment> {
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

pub fn get_all_comments(folder_path: &String) -> Vec<Comment> {
    let mut result: HashSet<Comment> = HashSet::new();

    for file in get_list_of_files(folder_path) {
        for comment in get_comments_from_file(&file) {
            result.insert(comment);
        }
    }

    result.into_iter().collect()
}
