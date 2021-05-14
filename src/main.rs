pub mod cli;

fn main() {
    cli::read_token();
    println!("{}", cli::read_agreement());
}
