use async_tungstenite::{
    tokio::{connect_async, ConnectStream},
    tungstenite::Message,
    WebSocketStream,
};
use futures::prelude::*;

pub struct Client {
    ws_stream: WebSocketStream<ConnectStream>,
}

impl Client {
    pub async fn new(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        //let url = "wss://towerverse.herokuapp.com";
        let (ws_stream, _) = connect_async(url.as_str()).await?;

        Ok(Client {
            ws_stream: ws_stream,
        })
    }

    pub async fn hello(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //let traveller = Traveller {
        //id: "".into(),
        //name: "".into(),
        //email: "".into(),
        //};
        let payload_json = r#"
        {
            "event": "createTraveller",
            "travellerName": "MichalUSER",
            "travellerEmail": "email",
            "travellerPassword": "asdttgh"
        }
        "#;

        self.ws_stream.send(Message::text(payload_json)).await?;

        let msg = self
            .ws_stream
            .next()
            .await
            .ok_or("didn't receive anything")??;
        println!("Received: {}", msg);

        Ok(())
    }
}
