use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response= reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response Body: {}", body);
    } else {
        println!(
            "Faild to get a valid response. Status: {}",
            response.status()
        );
    }
    Ok(())
}
