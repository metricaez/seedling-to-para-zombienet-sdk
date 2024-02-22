use zombienet_sdk::{NetworkConfig, NetworkConfigExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let network = NetworkConfig::load_from_toml("./zombienet-configs/0001-simple.toml")
        .expect("errored?")
        .spawn_native()
        .await?;

    println!("🚀🚀🚀🚀 network deployed");

    Ok(())
}
