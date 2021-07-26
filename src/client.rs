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
    packets: Vec<Value>,
}

impl Client {
    pub async fn new(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async(url.as_str()).await?;

        Ok(Client {
            ws_stream: ws_stream,
            packets: Vec::new(),
        })
    }

    pub fn _quit(&mut self) {
        println!("Quit!");
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
        Self::send(self, payload_json).await?;

        Ok(())
    }

    pub async fn hello2(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let payload_json = json!({
            "event": "logoutTraveller",
        });
        Self::send(self, payload_json).await?;

        Ok(())
    }

    pub async fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(msg) = self.ws_stream.next().await {
            let msg_json: Value = serde_json::from_str(msg?.into_text()?.as_str())?;
            let current_ref = msg_json["ref"].as_str();
            self.packets.retain(|packet| {
                if packet["ref"].as_str() == current_ref {
                    return false;
                }
                true
            });
            println!("received: {}", msg_json["ref"]);
            if self.packets.len() == 0 {
                break;
            }
        }

        Ok(())
    }

    pub async fn send(&mut self, mut data: Value) -> Result<(), Box<dyn std::error::Error>> {
        data["ref"] = json!(Self::get_uuid());
        self.ws_stream.send(Message::text(data.to_string())).await?;
        self.packets.push(data);

        Ok(())
    }
}
