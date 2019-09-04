use serde_json::Value;

const QOTD_URL: &str = "https://quotes.rest/qod";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(QOTD_URL)?;

    if let Some(content_type) = resp.headers().get("Content-Type") {
        let ct = content_type.to_str()?;
        match ct.find("application/json") {
            Some(_) => {
                let mut resp = resp;
                let text = resp.text()?;
                let json: Value = serde_json::from_str(&text)?;
                let quote = &json["contents"]["quotes"][0];
                
                println!("\n{}\n\n\t-- {}, {}",
                    quote["quote"],  
                    &quote["author"].to_string()[..].trim_matches('"'), 
                    quote["date"].to_string()[..].trim_matches('"'));
            },
            None => panic!("Expected a Content-Type of application/json. Found {}", ct),
        }
    }
    Ok(())
}
