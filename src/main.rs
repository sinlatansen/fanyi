use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "fy")]
struct Args {
    #[arg(trailing_var_arg = true)]
    input: Vec<String>,
}

#[derive(Deserialize)]
struct MyMemoryResponse {
    responseData: ResponseData,
}

#[derive(Deserialize)]
struct ResponseData {
    translatedText: String,
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

async fn translate(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (source, target) = if is_chinese(text) {
        ("zh", "en")
    } else {
        ("en", "zh")
    };

    let url = format!(
        "https://api.mymemory.translated.net/get?q={}&langpair={}|{}",
        text, source, target
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    let data: MyMemoryResponse = response.json().await?;

    Ok(data.responseData.translatedText)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let text = args.input.join(" ");
    if text.is_empty() {
        eprintln!("Usage: fy <text>");
        return Ok(());
    }

    let result = translate(&text).await?;
    println!("{}", result);

    Ok(())
}
