use std::io;

use serde_json::{to_string, Value};

pub struct Chat {
    pub chat_id: String,
    pub name: String,
}

pub async fn get_messages(chat_id: &str, token: &str, full: bool) -> String {
    if full {
        return get_messages_full(chat_id, token).await;
    } else {
        return get_messages_count(chat_id, token, 50).await;
    }
}

pub async fn get_chats(token: &str) -> Vec<Chat> {
    let res = discord_api("https://discord.com/api/v9/users/@me/channels", token).await;
    let parsed: Vec<Value> = serde_json::from_str(&res).unwrap();
    let mut result: Vec<Chat> = Vec::new();

    for user in parsed {
        let id: String = user["id"].to_string();
        let mut user_name = String::from("");
        let array_user: &Vec<Value> = user["recipients"].as_array().unwrap();

        for users in array_user {
            user_name = users["username"].to_string();
        }
        result.push(Chat {
            chat_id: id,
            name: user_name,
        });
    }

    result
}

async fn discord_api(url: &str, token: &str) -> String {
    let client = reqwest::Client::new();
    client
        .get(url)
        .header("Authorization", token)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

async fn get_messages_full(chat_id: &str, token: &str) -> String {
    let mut result = Vec::new();
    let mut last_message = get_messages_count(chat_id, token, 50).await;
    let mut i = 0;

    loop {
        if parse_json(&last_message).unwrap_or(vec![]).len() == 0 {
            break;
        }
        let parsed_msg = parse_json(&last_message).unwrap();
        let last_id = parsed_msg[parsed_msg.len() - 1]["id"].to_string();
        result.extend(parsed_msg);
        let last_id = last_id[1..last_id.len() - 1].to_string();

        let url = format!(
            "https://discord.com/api/v9/channels/{}/messages?before={}&limit={}",
            chat_id, last_id, 50
        );

        last_message = discord_api(&url, token).await;
        i += 1;
        eprint!("Stage: {}\r", i);
    }

    to_string(&result).unwrap()
}

async fn get_messages_count(chat_id: &str, token: &str, messages: i32) -> String {
    let url = format!(
        "https://discord.com/api/v9/channels/{}/messages?limit={}",
        chat_id, messages
    );
    discord_api(&url, token).await
}

pub fn parse_json(messages: &str) -> Result<Vec<Value>, io::Error> {
    let res: Vec<Value> = serde_json::from_str(&messages)?;
    Ok(res)
}
