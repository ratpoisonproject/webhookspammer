use std::io::{Read, stdout, Write};

use crossterm::style::Stylize;
use serde_json::json;

fn print_cli_prompt() -> String {
    println!("{} {} {} {}", "╭─".grey(), "webhookspammer".bold().green(), "─".grey(), env!("CARGO_PKG_VERSION").bold().green());
    print!("{} {} ", "╰".grey(), "$".grey());
    stdout().flush().unwrap();

    // read input
    // make sure not to overwrite the prompt
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input
}

fn main() {
    println!("{}", r#"╋╋╋╋╋┏┓
╋╋╋╋┏┛┗┓
┏━┳━┻┓┏╋━━┳━━┳┳━━┳━━┳━┓
┃┏┫┏┓┃┃┃┏┓┃┏┓┣┫━━┫┏┓┃┏┓┓
┃┃┃┏┓┃┗┫┗┛┃┗┛┃┣━━┃┗┛┃┃┃┃
┗┛┗┛┗┻━┫┏━┻━━┻┻━━┻━━┻┛┗┛
╋╋╋╋╋╋╋┃┃
╋╋╋╋╋╋╋┗┛"#.blue());
    println!("{}{}", "webhookspammer v".bold().green(), env!("CARGO_PKG_VERSION").bold().green());
    println!("{}{}", "by ".bold().green(), "the ratpoison project".italic().yellow());
    println!("{}", "run help for a list of commands".italic().grey());
    println!("{}", "──────────────────".grey());
    
    loop {
        let mut input = print_cli_prompt();
        input.pop(); // remove newline

        let args: Vec<&str> = input.split(" ").collect();
        let command = args[0];
        match command {
            "help" => {
                println!("{}", "──────────────────".grey());
                println!("{}", "help".bold().green());
                println!("{}", "──────────────────".grey());
                println!("{}", "help - show this message".italic().grey());
                println!("{}", "exit - exit the program".italic().grey());
                println!("{}", "spam <url> - spam a webhook".italic().grey());
                println!("{}", "──────────────────".grey());
            },
            "exit" => {
                println!("{}", "exiting...".italic().grey());
                break;
            },
            "spam" => {
                // if not enough arguments
                if args.len() < 2 {
                    println!("{}", "not enough arguments".italic().grey());
                    continue;
                }

                // send "test" 5 times
                let url = args[1];
                let client = reqwest::blocking::Client::new();
                // check if url is valid
                if !url.contains("https://discord.com/api/webhooks") {
                    println!("{}", "invalid url".italic().grey());
                    continue;
                }

                // check if the webhook is valid
                let res = client.get(url).send();
                if res.is_err() {
                    println!("{}", "invalid webhook".italic().grey());
                    continue;
                }

                // send the message
                for _ in 0..5 {
                    let res = client.post(url).json(&json!({
                        "content": "test"
                    })).send();
                    if res.is_err() {
                        println!("{}", "invalid webhook".italic().grey());
                        continue;
                    }
                }

                println!("{}", "sent 5 messages".italic().grey());

                // ask if they want to delete the webhook
                print!("{} ", "delete webhook? (y/n)".yellow());
                stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                input.pop(); // remove newline

                if input == "y" {
                    let res = client.delete(url).send();
                    if res.is_err() {
                        println!("{}", "invalid webhook".italic().grey());
                        continue;
                    }
                    println!("{}", "deleted webhook".italic().grey());
                }
            },
            _ => {
                println!("{}", "invalid command".italic().grey());
            }
        }
    }
}
