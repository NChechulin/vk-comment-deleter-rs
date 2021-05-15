pub mod cli;
pub mod comment_reading;
pub mod vk_api;
use std::thread;
use std::time::Duration;
use tokio;

// Время в мс между запросами. Не рекомендуется уменьшать из-за ограничений VK API.
const REQUEST_INTERVAL: u32 = 1400;
const SLEEP_DURATION: Duration = Duration::from_millis(REQUEST_INTERVAL as u64);

fn calculate_total_time(comments_num: u32) -> u32 {
    let total_ms: f32 = (REQUEST_INTERVAL as f32) * (comments_num as f32);
    (total_ms / 60_000.0) as u32
}

#[tokio::main]
async fn main() {
    let token = cli::read_token();
    let path = cli::read_comments_dir();

    if !cli::read_agreement() {
        println!("Комментарии не будут удалены");
        return;
    }

    println!("Начинаю загружать все комментарии...");
    let all_comments = comment_reading::get_all_comments(&path);
    println!("Комментарии загружены: всего {} шт.", all_comments.len());

    println!(
        "Начинаю удаление. Займет примерно {} мин.",
        calculate_total_time(all_comments.len() as u32)
    );

    let mut client = reqwest::Client::new();

    for (number, comment) in all_comments.iter().enumerate() {
        match comment
            .send_delete_comment_request(&token, &mut client)
            .await
        {
            Ok(_) => {}
            Err(e) => println!("Ошибка в комментарии #{} {}", number, e),
        }
        thread::sleep(SLEEP_DURATION);
    }
}
