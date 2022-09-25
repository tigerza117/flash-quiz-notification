use std::collections::HashMap;
use std::env;
use std::thread::sleep;

use dotenv::dotenv;
use lazy_static::lazy_static;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

lazy_static! {
    static ref TOKENS:  Vec<String> = env::var("LINE_TOKEN").unwrap().as_str().split("||").map(|x| ["Bearer", x].join(" ")).collect::<Vec<String>>();
}

static INTERVAL_TIME: u64 = 30;

fn main() {
    dotenv().ok();

    env_logger::init();

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let mut count = 0;

    loop {
        let response = client.get("https://onlearn.it.kmitl.ac.th/mod/forum/view.php?id=12867")
            .send()
            .unwrap()
            .text()
            .unwrap();

        let document = scraper::Html::parse_document(&response);

        if is_timeout(&document) {
            log::warn!("Timeout!!!");
            login(&client);
            sleep(std::time::Duration::from_secs(INTERVAL_TIME / 3));
            continue;
        }

                let title_selector =
                    scraper::Selector::parse("table[class='table discussion-list'] > tbody > tr")
                        .unwrap();

        let i = document.select(&title_selector).count();

        log::debug!("Count got {}!", i);

        if count == 0 {
            count = i;
        } else {
            if count > i && count != i {
                log::info!("New post!");
                count = i;
                notification();
            }
        }

        sleep(std::time::Duration::from_secs(INTERVAL_TIME));
    }
}

fn is_timeout(document: &scraper::Html) -> bool {
    let login_selector = scraper::Selector::parse("a[href='https://onlearn.it.kmitl.ac.th/login/index.php']").unwrap();

    let i = document.select(&login_selector).count();

    return i > 0;
}

fn login(client: &reqwest::blocking::Client) {
    log::info!("Login!!!");
    let username = env::var("ONLEARN_USERNAME").unwrap();
    let password = env::var("ONLEARN_PASSWORD").unwrap();
    let token = get_login_token(client);

    log::debug!("Login token {}", token);

    let mut map = HashMap::new();
    map.insert("rememberusername", "1");
    map.insert("anchor", "");
    map.insert("logintoken", token.as_str());
    map.insert("username", username.as_str());
    map.insert("password", password.as_str());

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/x-www-form-urlencoded")
        .unwrap(),
    );

    client.post("https://onlearn.it.kmitl.ac.th/login/index.php")
        .form(&map)
        .headers(headers)
        .send()
        .unwrap();
}

fn get_login_token(client: &reqwest::blocking::Client) -> String {
    let response = client.get("https://onlearn.it.kmitl.ac.th/login/index.php")
        .send()
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);

    let login_token_selector = scraper::Selector::parse("#login > input[name='logintoken']").unwrap();

    let mut selects = document.select(&login_token_selector);

    let token = selects.next().unwrap().value().attr("value").unwrap();

    return String::from(token);
}

fn notification() {
    let client = reqwest::blocking::Client::builder()
        .build()
        .unwrap();

    for token in TOKENS.iter() {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/x-www-form-urlencoded")
            .unwrap(),
        );
        headers.insert(AUTHORIZATION, HeaderValue::from_str(token)
            .unwrap(),
        );

        let mut map = HashMap::new();
        map.insert("message", "New Flash Quiz!!!\nLet's do it now!");

        client.post("https://notify-api.line.me/api/notify")
            .form(&map)
            .headers(headers)
            .send()
            .unwrap();
    }
    log::info!("Notification sent!!!");
}

