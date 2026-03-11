extern crate apiclient_boilerplate_rust;

use apiclient_boilerplate_rust::client::ApiClient;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let base_url = "https://jsonplaceholder.typicode.com";
    let api_client = ApiClient::new(base_url);

     match api_client.get_post(1).await {
        Ok(post) => {
            println!("Post: {post:?}");
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }

    Ok(())
}
