use std::collections::HashMap;
use std::thread::sleep;
use reqwest::header::{HeaderMap, COOKIE, HeaderValue, CONTENT_TYPE, AUTHORIZATION};

static TOKEN: &'static str = "Bearer ...";

fn main() {
    let client = reqwest::blocking::Client::builder()
        .build()
        .unwrap();

    let mut count = 0;

    loop {
        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, HeaderValue::from_str("MoodleSession=niqs8aauftvf7tmbfufffk9vgp")
            .unwrap());

        let response = client.get("https://onlearn.it.kmitl.ac.th/mod/forum/view.php?id=12867")
            .headers(headers)
            .send()
            .unwrap()
            .text()
            .unwrap();

        let document = scraper::Html::parse_document(&response);

        let title_selector = scraper::Selector::parse("div[data-name='Flash Quiz (Total 5 Pts.)'] > table > tbody > tr")
            .unwrap();

        let i = document.select(&title_selector).count();

        if count == 0 {
            count = i;
        } else {
            if count != i {
                println!("New post!");
                count = i;
                notification();
            }
        }

        sleep(std::time::Duration::from_secs(30));
    }
}

fn notification() {
    let client = reqwest::blocking::Client::builder()
        .build()
        .unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("	application/x-www-form-urlencoded")
        .unwrap());
    headers.insert(AUTHORIZATION, HeaderValue::from_str(TOKEN)
        .unwrap());

    let mut map = HashMap::new();
    map.insert("message", "IT LAW NEW Flash Quiz!!!\n Let do it now!");
    map.insert("body", "json");

    client.post("https://notify-api.line.me/api/notify")
        .form(&map)
        .headers(headers)
        .send().unwrap();

    println!("Notification sent! {}", TOKEN);
}
