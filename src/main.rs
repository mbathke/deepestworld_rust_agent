mod utils;
mod player;

use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tungstenite::{connect, Message};
use utils::parse_csrf_token;
use player::Player;

#[derive(Serialize, Deserialize, Debug)]
struct WsToken {
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthMessage(String, WsToken);

#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse(String, Player);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // load .env config
    dotenv().ok();

    let login_url = env::var("LOGIN_URL").expect("LOGIN_URL not found.");
    let game_url = env::var("GAME_URL").expect("GAME_URL not found.");
    let ws_url = env::var("WS_URL").expect("WS_URL not found.");
    let ws_token_url = env::var("WS_AUTH_TOKEN_URL").expect("WS_AUTH_TOKEN_URL not found.");
    let username = env::var("USERNAME").expect("USERNAME not found.");
    let password = env::var("PASSWORD").expect("PASSWORD not found.");
    // let character = env::var("CHARACTER").expect("CHARACTER not found.");

    let client = Client::builder()
        .cookie_store(true)
        .build()?;

    // first, call the login root to get a valid csrf token and initial session ID
    let response = client.get(&login_url).send().await?;
    // let response_headers = response.headers().clone();

    if response.status() != 200 {
        println!("Invalid Response");
        return Ok(());
    }

    // get the csrf token
    let response_body = response.text().await?;
    let csrf_token = parse_csrf_token(&response_body).expect("Couldn't parse csrf_token");
    // println!("csrf token: {}", csrf_token);

    // request the login page to get a valid session
    let params = [
        ("_username", username),
        ("_password", password),
        ("_target_path", "/default-login-redirect?return=0".to_string()),
        ("_csrf_token", csrf_token.to_string()),
    ];
    let _login_response = client
        .post(&login_url)
        .form(&params)
        .send().await?;

    println!("Login response status: {}", _login_response.status());

    // first request the ws_auth_token here
    let ws_token_response = client.get(ws_token_url)
        .header("Referer", game_url)
        .send().await?;
    let ws_token_string = ws_token_response.text().await?;
    let ws_token: WsToken =
        serde_json::from_str(&ws_token_string).expect("Could not get Websocket token.");

    println!("ws_token {:?}", ws_token);

    let (mut socket, ws_response) = connect(ws_url).expect("Can't connect to websocket.");

    // first send the auth token, then we will get an auth object

    println!("Connected to the websocket");
    println!("Response HTTP code: {}", ws_response.status());
    for (header, _value) in ws_response.headers() {
        println!("* {header}");
    }

    println!("Say hello");

    // create the auth message with serde
    let auth_message = serde_json::to_string(&AuthMessage("auth".to_string(), ws_token)).unwrap();
    println!("Auth Message {:?}", auth_message);

    socket.send(Message::Text(auth_message.into())).unwrap();

    for _i in 0..1 { 
        let msg = socket.read().expect("Error reading message.");
        println!("Received: {}", msg);

        // TODO: find out how to deserialize the message
        let player = serde_json::from_str::<AuthResponse>(&msg.to_string());
        println!("Player: {:#?}", player);
    }

    let _ = socket.close(None);

    Ok(())
}
