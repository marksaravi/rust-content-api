use reqwest::blocking::get;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Example GET request to httpbin.org
    let response = get("https://firestore.googleapis.com/v1/projects/content-api-by-firestore/databases/digital-publishing/documents/schemas")?.text()?;
    println!("Response: {}", response);
    Ok(())
}
