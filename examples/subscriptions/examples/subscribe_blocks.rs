use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/REDACTED_INFURA_PROJECT_ID")
            .await?;
    let mut stream = provider.subscribe_blocks().await?.take(1);
    while let Some(block) = stream.next().await {
        println!(
            "Ts: {:?}, block number: {} -> {:?}",
            block.timestamp,
            block.number.unwrap(),
            block.hash.unwrap()
        );
    }

    Ok(())
}
