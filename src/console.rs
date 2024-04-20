use ds_saver::{parse_json, Chat};

pub fn pretty_format(messages: &str) -> Vec<String> {
    let json_data = parse_json(messages).expect("Error load json");
    let mut formated = Vec::new();

    for msg in json_data {
        formated.push(format!("[{}] {}: {}", &msg["timestamp"].to_string()[12..20], msg["author"]["global_name"], msg["content"]));
    }

    formated
}

pub fn pretty_print(messages: &str) {
    for msg in pretty_format(messages).iter().rev() {
        println!("{}", msg);
    }
}


pub fn print_chats(chats: Vec<Chat>) {
    for (i, chat) in chats.iter().enumerate() {
        println!("[{}] {}: {}", i, chat.name, chat.chat_id);
    }
}