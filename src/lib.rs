use anyhow::Result;

pub mod config;
pub mod storage;

pub async fn run(conf: &config::Config) -> Result<()> {
    println!("conf: {:?}", conf);

    let rustfs_client = storage::connect_to_rustfs(&conf.rustfs)
        .await?;

    println!("connected to rustfs");

    Ok(())
}