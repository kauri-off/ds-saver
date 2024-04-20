# Discord Message Saver
Save messages from discord channel via terminal

## How to get Token
Check this [guide](https://gist.github.com/MarvNC/e601f3603df22f36ebd3102c501116c6)

# Help menu

```
Discord saver

USAGE:
    ds-saver <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    chats       Get chats id
    help        Prints this message or the help of the given subcommand(s)
    messages    Get messages from chat
```
```
Get messages from chat

USAGE:
    ds-saver messages [FLAGS] [OPTIONS] --chat-id <chat-id> --token <token>

FLAGS:
    -f, --full       
    -h, --help       Prints help information
    -p, --print      
    -V, --version    Prints version information

OPTIONS:
    -c, --chat-id <chat-id>    
    -o, --output <output>      
    -t, --token <token> 
```
```
Get chats id

USAGE:
    ds-saver chats --token <token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --token <token> 
```