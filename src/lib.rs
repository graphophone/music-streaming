use anyhow::Result;

pub mod config;

pub async fn run(conf: &config::Config) -> Result<()> {
    println!("conf: {:?}", conf);
    Ok(())
}