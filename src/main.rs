//use tungstenite::{connect, Message};

mod client;
// mod traveller;

//use traveller::Traveller;
use client::Client;

//use serde::{Deserialize, Serialize};
//#[derive(Serialize, Deserialize)]
//struct Person {
//name: String,
//age: usize,
//verified: bool,
//}

// TODO
// Add id's to packets

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new("wss://towerverse.herokuapp.com".into()).await?;
    client.hello().await?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run())
}
