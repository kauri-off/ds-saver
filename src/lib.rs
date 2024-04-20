use std::io;

use curl::easy::{Easy, List};
use serde_json::{to_string, Value};

pub struct Chat {
    pub chat_id: String,
    pub name: String
}

pub fn get_messages(chat_id: &str, token: &str, full: bool) -> String {
    if full {
        return get_messages_full(chat_id, token);
    } else {
        return get_messages_count(chat_id, token, 50);
    }
}

pub fn get_chats(token: &str) -> Vec<Chat> {
    let res = discord_api("https://discord.com/api/v9/users/@me/channels", token);
    let parsed: Vec<Value> = serde_json::from_str(&res).unwrap();
    let mut result: Vec<Chat> = Vec::new();

    for user in parsed {
        let id: String = user["id"].to_string();
        let mut user_name = String::from("");
        let array_user: &Vec<Value> = user["recipients"].as_array().unwrap();

        for users in array_user {
            user_name = users["username"].to_string();
        }
        result.push(Chat { chat_id: id, name: user_name });
    }

    result
}

fn discord_api(url: &str, token: &str) -> String {
    let mut easy = Easy::new();

    let header = format!("Authorization: {}", token);
    let mut list = List::new();
    list.append(&header).unwrap();

    easy.url(&url).unwrap();
    easy.http_headers(list).unwrap();

    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();

        transfer.perform().unwrap();
    }
    std::str::from_utf8(&data).expect("Error parse UTF-8").to_string()
}

fn get_messages_full(chat_id: &str, token: &str) -> String {
    let mut result = Vec::new();
    let mut last_message = get_messages_count(chat_id, token, 50);
    let mut i = 0;

    loop {
        if parse_json(&last_message).unwrap_or(vec![]).len() == 0 {
            break;
        }
        let parsed_msg = parse_json(&last_message).unwrap();
        let last_id = parsed_msg[parsed_msg.len()-1]["id"].to_string();
        result.extend(parsed_msg);
        let last_id = last_id[1..last_id.len()-1].to_string();

        let url = format!("https://discord.com/api/v9/channels/{}/messages?before={}&limit={}", chat_id, last_id, 50);

        last_message = discord_api(&url, token);
        i += 1;
        eprint!("Stage: {}\r", i);
    }

    to_string(&result).unwrap()
}

fn get_messages_count(chat_id: &str, token: &str, messages: i32) -> String {
    let url = format!("https://discord.com/api/v9/channels/{}/messages?limit={}", chat_id, messages);
    discord_api(&url, token)
}

pub fn parse_json(messages: &str) -> Result<Vec<Value>, io::Error> {
    let res: Vec<Value> = serde_json::from_str(&messages)?;
    Ok(res)
}