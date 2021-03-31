#[tokio::main]
async fn main() {
    println!("Making GET request.");
    let res = reqwest::get("https://xkcd.com/info.0.json").await.unwrap();
    println!("Response: {:?}", res);
}