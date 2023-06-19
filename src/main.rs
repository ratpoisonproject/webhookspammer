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
        println!("{}", print_cli_prompt());
    }
}
