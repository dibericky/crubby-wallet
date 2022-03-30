use std::env;
use dotenv::dotenv;

fn get_env(name: &str) -> String {
    env::var(name).expect(&format!("{} env should be set", name))
}

#[derive(Debug)]
pub struct Env {
    pub api_url: String
}

impl Env {
    pub fn get() -> Env {
        dotenv().ok();

        Env{
            api_url: get_env("API_URL")
        }
    }
}