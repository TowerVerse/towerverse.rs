mod client;

use client::Client;
use std::time::Instant;

//use serde::{Deserialize, Serialize};
//#[derive(Serialize, Deserialize)]

// TODO
// Work on traveller struct, use code above
// Add concurrency, https://tokio.rs/tokio/tutorial/spawning

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    let mut client = Client::new("wss://towerverse.herokuapp.com".into()).await?;

    client.hello().await?;
    client.hello2().await?;

    client.listen().await?;

    println!("elapsed: {}ms", now.elapsed().as_millis());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run())
}
