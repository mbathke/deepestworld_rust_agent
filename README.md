# Deepestworld Rust Agent

This is a work in progress and it's not yet clear what it can be used for.

## Installation

First create a `.env` file in your root directory and enter the following content:

```ini
USERNAME=<your_user_name>
PASSWORD=<your_password>
CHARACTER=<your_character_name>
LOGIN_URL=https://deepestworld.com/login
GAME_URL=https://deepestworld.com/game
WS_URL=wss://ca1.deepestworld.com/
WS_AUTH_TOKEN_URL=https://deepestworld.com/ws-auth-token
```

Install all modules and/or run the program with cargo.

```bash
$ cargo build
$ cargo run
```

## Status and ToDo's

It logs into the deepestworld game and requests the character information via the websocket.

### ToDo's 

* create a websocket connection with tungstenite
* build the api locally
* display the information in some useful way with bevy game engine
