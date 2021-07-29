mod client;

use client::Client;
use std::time::Instant;

// TODO
// Work on traveller struct, use code above

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // wss://towerverse.herokuapp.com
    // ws://localhost:5000
    let mut client = Client::new("ws://localhost:5000".into()).await?;

    let now = Instant::now();
    client
        .login_traveller(
            "visual.studio.growtopia@gmail.com".into(),
            "123467890".into(),
        )
        .await
        .unwrap();
    client.hello().await.unwrap();
    println!("elapsed: {} ms", now.elapsed().as_millis());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run())
}
