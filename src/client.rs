use async_tungstenite::{
    tokio::{connect_async, ConnectStream},
    tungstenite::Message,
    WebSocketStream,
};
use futures::prelude::*;
use serde_json::{json, Value};
use uuid::Uuid;

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

    pub async fn login_traveller(
        &mut self,
        email: String,
        password: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload_json = json!({
            "event": "loginTraveller",
            "travellerEmail": email,
            "travellerPassword": password
        });
        let msg = Self::send(self, payload_json).await?;
        println!("{}", msg["event"].as_str().unwrap());

        Ok(())
    }

    pub async fn hello(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //let traveller = Traveller {
        //id: "".into(),
        //name: "".into(),
        //email: "".into(),
        //};
        let payload_json = json!({
            "event": "logoutTraveller"
        });
        let msg = Self::send(self, payload_json).await?;
        println!("{}", msg["event"].as_str().unwrap());

        Ok(())
    }

    pub async fn send(&mut self, mut data: Value) -> Result<Value, Box<dyn std::error::Error>> {
        data["ref"] = json!(Self::get_uuid());
        self.ws_stream.send(Message::text(data.to_string())).await?;
        let msg = self
            .ws_stream
            .next()
            .await
            .ok_or("didn't receive anything")??;

        Ok(serde_json::from_str(msg.to_text()?)?)
    }
}
