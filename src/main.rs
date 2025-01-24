use dotenv::dotenv;
use reqwest::Client;
use std::env;
// use tungstenite::{connect, Message};

/// Parses a string slice for a keyword and returns a new slice when a semicolon 
/// is found like its typically formatted in a Cookie.
fn parse_cookie<'a>(keyword: &str, string_slice: &'a str) -> Option<&'a str> {
    let start_index = string_slice.find(keyword);

    match start_index {
        Some(start) => {
            let end_index = string_slice[start..].find(';');
            match end_index {
                Some(end) => Some(&string_slice[start..end]),
                None => None
            }
        },
        None => return None
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load .env config
    dotenv().ok();

    let login_url = env::var("LOGIN_URL").expect("LOGIN_URL not found.");
    // let game_url = env::var("GAME_URL").expect("GAME_URL not found.");
    // let ws_url = env::var("WS_URL").expect("WS_URL not found.");
    let username = env::var("USERNAME").expect("USERNAME not found.");
    let password = env::var("PASSWORD").expect("PASSWORD not found.");
    // let character = env::var("CHARACTER").expect("CHARACTER not found.");

    let client = Client::new();
    let response = client.get(&login_url).send().await?;

    if response.status() != 200 {
        println!("Invalid Response");
        return Ok(());
    }

    // let response_body = response.text().await?;
    // let csrf_token = parse_csrf_token(&response_body);
    let params = [
        ("_username", username),
        ("_password", password),
        // ("_target_path", "/default-login-redirect?return=0"),
        // ("_csrf_token", csrf_token),
    ];
    let login_response = client.post(login_url).form(&params).send().await?;

    let cookie = login_response
        .headers()
        .get("set-cookie")
        .expect("Could not get login session cookie.")
        .to_str()
        .unwrap();

    let cookie_value = parse_cookie("PHPSESSID", &cookie).unwrap();

    println!("Login cookie: {}", cookie_value);

    // TODO: connect with the websocket

    Ok(())
}
