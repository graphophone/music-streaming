use anyhow::Result;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, config::Credentials};

use crate::config;

pub async fn connect_to_rustfs(conf: &config::RustfsConfig) -> Result<Client> {
    let credentials = Credentials::new(
        &conf.access_key,
        &conf.secret_key,
        None,
        None,
        "rustfs",
    );

    let shared_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(conf.region.clone()))
        .credentials_provider(credentials)
        .endpoint_url(&conf.endpoint_url)
        .load()
        .await;

    let s3_config = aws_sdk_s3::config::Builder::from(&shared_config)
        .force_path_style(true)
        .build();

    let rustfs_client = Client::from_conf(s3_config);
    let buckets = rustfs_client.list_buckets().send().await?;
    let music_bucket = buckets.buckets().iter()
        .find(|&b| b.name == Some(conf.bucket.clone()));
    if music_bucket.is_none() {
        return Err(anyhow::Error::msg("music bucket not found"));
    }

    Ok(rustfs_client)
}