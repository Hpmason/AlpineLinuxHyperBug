use std::{error::Error};

use hyper::Client;
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Making GET request.");
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get("https://www.google.com/".parse()?).await?;
    
    println!("Received Response! Code {}", res.status());
    Ok(())
}