use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "fyy")]
struct Args {
    input: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

fn is_chinese(text: &str) -> bool {
    let first = text.chars().next();
    match first {
        Some(c) => {
            if c >= '\u{4e00}' && c <= '\u{9fff}' {
                true
            } else {
                false
            }
        }
        None => false,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("input: {}", args.input);
    println!("is_chinese: {}", is_chinese(&args.input));

    Ok(())
}
