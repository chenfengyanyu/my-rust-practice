// https://crates.io/crates/async-openai
// 命令行导入环境变量：export OPENAI_API_KEY='sk-xxxx'
use async_openai::{
    types::{CreateImageRequestArgs, ImageSize, ResponseFormat},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create client, reads OPENAI_API_KEY environment variable for API key.
    let client = Client::new();
    println!("{:?}", client);

    let request = CreateImageRequestArgs::default()
        .prompt("cats on sofa and carpet in living room")
        .n(2)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai")
        .build()?;

    let response = client.images().create(request).await?;

    // Download and save images to ./data directory.
    // Each url is downloaded and saved in dedicated Tokio task.
    // Directory is created if it doesn't exist.
    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));

    Ok(())
}
