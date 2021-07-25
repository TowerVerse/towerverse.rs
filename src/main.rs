mod client;

use client::Client;

//use serde::{Deserialize, Serialize};
//#[derive(Serialize, Deserialize)]

// TODO
// Work on traveller struct, use code above

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new("wss://towerverse.herokuapp.com".into()).await?;
    client.hello().await?;
    client.hello2().await?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run())
}
