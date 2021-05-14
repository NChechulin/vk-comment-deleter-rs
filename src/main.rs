pub mod cli;
pub mod comment_reading;

fn main() {
    let path = String::from("/home/nikolay/Downloads/comments");
    // let result = comment_reading::get_list_of_files(&path);
    // for path in result {
    //     println!("{}", path);
    // }
    let result = comment_reading::get_all_comments(&path);
    println!("{}", result.len());
    // let result = comment_reading::get_comments_from_file(&path);
    // println!("{}", result.len());
    // cli::read_token();
    // println!("{}", cli::read_agreement());
}
