use std::{error::Error};

use hyper::{Client, client::HttpConnector};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Making GET request.");
    let https = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get("http://www.google.com/".parse()?).await?;
    
    println!("Received Response! Code {}", res.status());
    Ok(())
}