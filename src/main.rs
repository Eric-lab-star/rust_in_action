use std::error::Error;
use reqwest;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://chatgpt.com/c/67eb0d59-c1fc-8001-a2ae-4085b0061cc5";
    let mut response = reqwest::get(url)?;
    let content = response.text()?;
    print!("{}", content);
    Ok(())
}
