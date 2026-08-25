use music_streaming::{run, config};

#[tokio::main]
async fn main() {
    let conf = config::Config::build("config/config.local.toml")
        .expect("failed to parse config");
    run(&conf)
        .await
        .expect("unhandled error when running music streaming service");
}
