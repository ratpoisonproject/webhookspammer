use std::io::{Read, stdout, Write};
use dialoguer::{Input, Password, Confirm, theme::ColorfulTheme};
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
                println!("{}", "spam - spam a webhook".italic().grey());
                println!("{}", "──────────────────".grey());
            },
            "exit" => {
                println!("{}", "exiting...".italic().grey());
                break;
            },
            "spam" => {
                // ask for webhook url
                let webhook_url = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your Discord webhook URL")
                    .validate_with({
                        // check if the url is valid
                        // send a GET request to the url and check if the response is 200
                        |input: &String| -> Result<(), &str> {
                            let client = reqwest::blocking::Client::new();
                            let res = client.get(input).send();
                            match res {
                                Ok(res) => {
                                    if res.status() == 200 {
                                        Ok(())
                                    } else {
                                        Err("Invalid webhook URL")
                                    }
                                }
                                Err(_) => Err("Invalid webhook URL"),
                            }
                        }
                    })
                    .interact_text()
                    .unwrap();

                // ask for message
                let message: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the message you want to spam")
                    .interact_text()
                    .unwrap();

                // ask for amount
                let amount: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the amount of times you want to spam the webhook")
                    .validate_with({
                        // check if the amount is valid
                        // make sure it's a number
                        |input: &String| -> Result<(), &str> {
                            match input.parse::<u32>() {
                                Ok(_) => Ok(()),
                                Err(_) => Err("Invalid amount"),
                            }
                        }
                    })
                    .interact_text()
                    .unwrap();

                // convert amount to u32
                let amount: u32 = amount.parse().unwrap();

                // ask for username
                let username: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the username you want to use")
                    .interact_text()
                    .unwrap();

                // convert all the strings to &str
                let webhook_url: &str = &webhook_url;
                let message: &str = &message;
                let username: &str = &username;
    

                println!("{}", "──────────────────".grey());
                println!("{}", message.italic().grey());
                println!("{}", webhook_url.italic().grey());
                println!("{}", amount.to_string().italic().grey());
                println!("{}", username.italic().grey());
                println!("{}", "──────────────────".grey());

                // ask for confirmation
                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Are you sure you want to spam this webhook?")
                    .interact()
                    .unwrap();
                
                if confirm {
                    // send the requests
                    let client = reqwest::blocking::Client::new();
                    for i in 0..amount {
                        let res = client.post(&*webhook_url)
                            .json(&json!({
                                "content": message,
                                "username": username
                            }))
                            .send();
                        match res {
                            Ok(_) => {
                                println!("{} {} {} {}", "sent message".italic().grey(), i.to_string().bold().green(), "out of".italic().grey(), amount.to_string().bold().green());
                            }
                            Err(_) => {
                                println!("{}", "failed to send message".italic().grey());
                            }
                        }
                    }
                } else {
                    println!("{}", "cancelled".italic().grey());
                }
            },
            _ => {
                println!("{}", "invalid command".italic().grey());
            }
        }
    }
}
