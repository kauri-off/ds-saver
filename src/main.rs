use ds_saver::*;
use std::fs;
use structopt::StructOpt;

mod console;

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Chats { token } => {
            console::print_chats(get_chats(&token).await);
        }
        Opt::Messages {
            token,
            chat_id,
            output,
            print,
            full,
        } => {
            let messages = get_messages(&chat_id, &token, full).await;
            if print {
                console::pretty_print(&messages);
            }
            match output {
                Some(t) => {
                    fs::write(t, messages).unwrap();
                }
                None => {
                    if !print {
                        fs::write(format!("{}.json", chat_id), messages).unwrap();
                        println!(
                            "Messages saved to {}, check 'messages --help'",
                            format!("{}.json", chat_id)
                        )
                    }
                }
            }
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "ds-saver", about = "Discord saver")]
enum Opt {
    #[structopt(name = "chats", about = "Get chats id")]
    Chats {
        #[structopt(short, long)]
        token: String,
    },
    #[structopt(name = "messages", about = "Get messages from chat")]
    Messages {
        #[structopt(short, long)]
        token: String,

        #[structopt(short, long)]
        chat_id: String,

        #[structopt(short, long)]
        output: Option<String>,

        #[structopt(
            short,
            long,
            about = "Print messages to screen, if not save to default path"
        )]
        print: bool,

        #[structopt(short, long, about = "Save full chat")]
        full: bool,
    },
}
