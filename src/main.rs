pub mod cli;
pub mod comment_reading;

fn main() {
    let path = String::from("/home/nikolay/Downloads/comments/comments.html");
    let result = comment_reading::get_comments_from_file(&path);
    println!("{}", result.len());
    // cli::read_token();
    // println!("{}", cli::read_agreement());
}
