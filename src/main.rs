mod env_manager;
mod fetch_api;
mod overview;
mod page_builder;

use std::fs;
use env_manager::Env;
use fetch_api::fetch;

use crate::{overview::{Overview}, page_builder::Page};

fn write_to_file (file_name: &str, content: &str) {
    let file_path = file_name;
    fs::write(file_path, content).expect("Unable to write file")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let envs = Env::get();
    println!("Using api url: {}", envs.api_url);

    let resp = fetch(&envs.api_url).await?;
    let overview = Overview::new(&resp);
    let page = Page::new(overview);
    write_to_file("report.md", &page.as_markdown());

    Ok(())
}