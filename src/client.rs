use async_tungstenite::{
    tokio::{connect_async, ConnectStream},
    tungstenite::Message,
    WebSocketStream,
};
use futures::prelude::*;
use uuid::Uuid;
use serde_json::{json, Value};

pub struct Client {
    ws_stream: WebSocketStream<ConnectStream>,
}

impl Client {
    pub async fn new(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async(url.as_str()).await?;

        Ok(Client {
            ws_stream: ws_stream,
        })
    }

    fn get_uuid() -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }

    pub async fn hello(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //let traveller = Traveller {
        //id: "".into(),
        //name: "".into(),
        //email: "".into(),
        //};
        let payload_json = json!({
            "event": "loginTraveller",
            "travellerEmail": "visual.studio.growtopia@gmail.com",
            "travellerPassword": "1234567890"
        });
        let res = Self::send(self, payload_json).await?;
        println!("{}", res);

        Ok(())
    }

    pub async fn hello2(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let payload_json = json!({
            "event": "logoutTraveller",
        });
        let res = Self::send(self, payload_json).await?;
        println!("{}", res);

        Ok(())
    }

    pub async fn send(&mut self, mut data: Value) -> Result<String, Box<dyn std::error::Error>> {
        data["ref"] = json!(Self::get_uuid());
        self.ws_stream.send(Message::text(data.to_string())).await?;

        let msg = self
            .ws_stream
            .next()
            .await
            .ok_or("didn't receive anything")??;
        // Maybe there is a better way to do this line under
        Ok(msg.into_text().unwrap())
    }
}
