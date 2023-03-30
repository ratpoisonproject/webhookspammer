use crossterm::style::Stylize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde_json::json;




fn main() {
    // prompt for discord webhook url
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

    // prompt for message text
    let message_text = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(r"Enter the message text, use \ to go to a new line")
        .validate_with({
            // check if the message text is empty
            |input: &String| -> Result<(), &str> {
                if input.is_empty() || input.len() > 2000 {
                    Err("Message text cannot be empty or more than 2000 characters")
                } else {
                    Ok(())
                }
            }
        })
        .interact_text()
        .unwrap();

    // replace \ with a new line character
    let message_text = message_text.replace("\\", "\n");
    
    // ask for the delay between messages
    let delay = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the delay between messages in milliseconds")
        .default("1000".to_string())
        .validate_with({
            // check if the delay is a valid number
            |input: &String| -> Result<(), &str> {
                match input.parse::<u64>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Invalid number"),
                }
            }
        })
        .interact_text()
        .unwrap();

    // convert the delay to u64
    let delay = delay.parse::<u64>().unwrap();

    // prompt for the number of messages to send
    let number_of_messages = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the number of messages to send")
        .default("100".to_string())
        .validate_with({
            // check if the number of messages is a valid number
            |input: &String| -> Result<(), &str> {
                match input.parse::<u64>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Invalid number"),
                }
            }
        })
        .interact_text()
        .unwrap();

    // convert the number of messages to u64
    let number_of_messages = number_of_messages.parse::<u64>().unwrap();

    // confirm the settings
    let confirm = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Confirm the settings")
        .default(0)
        .item("Yes")
        .item("No")
        .interact()
        .unwrap();

    // if the user selected no, exit the program
    if confirm == 1 {
        println!("{}", "Goodbye!".red());
        std::process::exit(0);
    }

    // construct the json body
    let body = json!({
        "content": message_text,
        "username": "Spammer",
    });

    // send the messages
    for _ in 0..number_of_messages {
        let client = reqwest::blocking::Client::new();
        let res = client.post(&webhook_url).json(&body).send();
        match res {
            Ok(res) => {
                if res.status() == 204 {
                    println!("{}", "Message sent!".green());
                } else {
                    // if the error is that the webhook does not exist, prompt the user
                    if res.status() == 404 {
                        println!("{}", "The webhook does not exist! Sounds like they deleted it!".red());
                        std::process::exit(0);
                    }
                    println!("{}", "Message failed to send!".red());
                }
            }
            Err(_) => println!("{}", "Message failed to send!".red()),
        }
        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
}
