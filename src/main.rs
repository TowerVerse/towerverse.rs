mod client;

use client::Client;
use std::time::Instant;

//use serde::{Deserialize, Serialize};
//#[derive(Serialize, Deserialize)]

// TODO
// Work on traveller struct, use code above
// Add concurrency, https://tokio.rs/tokio/tutorial/spawning

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new("wss://towerverse.herokuapp.com".into()).await?;
    //let mut client = Client::new("ws://localhost:5000".into()).await?;


    let handle = tokio::spawn(async move {
        let now = Instant::now();
        client.hello().await.unwrap();
        client.hello2().await.unwrap();
        client.listen().await.unwrap();
        println!("elapsed: {} ms", now.elapsed().as_millis());
    });

    handle.await.unwrap();

    //client.hello().await.unwrap();
    //client.hello2().await.unwrap();
    //client.listen().await.unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run())
}
