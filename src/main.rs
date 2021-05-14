extern crate regex;
use dialoguer::Input;
use regex::Regex;

/// Проверяет, может ли введенная строка быть токеном (но не дает 100%-й гарантии)
fn token_first_validation(token: &String) -> Result<(), &'static str> {
    let re = Regex::new("^[a-z0-9]{85}$").unwrap();
    if re.is_match(token) {
        return Ok(());
    }

    Err("Введенная строка не может быть валидным токеном.")
}

/// Читает строку с токеном и валидирует
fn read_token() -> String {
    Input::new()
        .with_prompt("Введите VK API access_token")
        .validate_with(|token: &String| -> Result<(), &str> {
            match token_first_validation(token) {
                Ok(()) => Ok(()),
                Err(message) => Err(message),
            }
        })
        .interact()
        .unwrap()
}

/// Читает Y/N для согласия на удаление комментариев
fn read_agreement() -> bool {
    let agreed: String = Input::new()
        .with_prompt("Начать удаление? [Y/N]")
        .interact()
        .unwrap();

    agreed.trim().to_ascii_lowercase() == "y"
}

fn main() {
    // read_token();
    println!("{}", read_agreement());
}
