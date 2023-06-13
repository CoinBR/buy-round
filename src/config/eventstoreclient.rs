use eventstore::{Client, Credentials, EventData, ReadStreamOptions, StreamPosition};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use tokio::runtime::Runtime;

struct ConnectionInfo {
    ip: String,
    port: u16,
    settings: String,
}

impl ConnectionInfo {
    fn from_env() -> Result<Self, Box<dyn Error>> {
        let ip = env::var("EVENTSTORE_IP")?;
        let port_str = env::var("EVENTSTORE_PORT")?;
        let port = port_str.parse::<u16>()?;
        let settings = env::var("EVENTSTORE_SETTINGS")?;

        Ok(ConnectionInfo { ip, port, settings })
    }

    fn get_connection_string(&self) -> String {
        format!(
            "esdb+discover://{}:{}?{}",
            self.ip, self.port, self.settings
        )
    }
}

lazy_static! {
    // ES == EventStore Client
    pub static ref CLIENT: Client = {

        let connection_info = ConnectionInfo::from_env().unwrap_or_else(|_| {
            panic!("Unable to get EventStore connection info from environment");
        });

        Client::new(connection_info.get_connection_string()).unwrap_or_else(|_| {
            panic!("Unable to connect to EventStore")
        })
    };
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//
//     let settings = "esdb+discover://127.0.0.1:2113?tls=false&keepAliveTimeout=10000&keepAliveInterval=10000".parse()?;
//     let client = Client::new(settings)?;
//
//     let event = TestEvent {
//         id: Uuid::new_v4().to_string(),
//         important_data: "I wrote my first event! aaa".to_string(),
//     };
//
//     let event_data = EventData::json("TestEvent", event)?.id(Uuid::new_v4());
//
//     client
//         .append_to_stream("some-stream", &Default::default(), event_data)
//         .await?;
//
//     let options = ReadStreamOptions::default().max_count(10);
//     let mut stream = client.read_stream("some-stream", &options).await?;
//
//
//     while let Some(event) = stream.next().await? {
//         let c = event.event.unwrap().data;
//         let c_string = String::from_utf8_lossy(&c);
//         println!("{}", c_string);
//     }
//
//
//     Ok(())
//
// }
