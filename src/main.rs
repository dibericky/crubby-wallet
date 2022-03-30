mod env_manager;
mod fetch_api;
mod analyzer;

use env_manager::Env;
use fetch_api::fetch;

use crate::analyzer::{Overview};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let envs = Env::get();
    println!("Using api url: {}", envs.api_url);

    let resp = fetch(&envs.api_url).await?;
   let overview = Overview::calculate(&resp);
    println!("{:?}", overview);
    Ok(())
}