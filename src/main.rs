use eventstore::{
    Client, Credentials, EventData,  StreamPosition, ReadStreamOptions
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::runtime::Runtime;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct TestEvent{
    id: String,
    important_data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let settings = "esdb+discover://127.0.0.1:2113?tls=false&keepAliveTimeout=10000&keepAliveInterval=10000".parse()?;
    let client = Client::new(settings)?;

    let event = TestEvent {
        id: Uuid::new_v4().to_string(),
        important_data: "I wrote my first event!".to_string(),
    };

    let event_data = EventData::json("TestEvent", event)?.id(Uuid::new_v4());

    client
        .append_to_stream("some-stream", &Default::default(), event_data)
        .await?;

    let options = ReadStreamOptions::default().max_count(10);
    let mut stream = client.read_stream("some-stream", &options).await?;

    while let Some(event) = stream.next().await? {
        let c = event.event.unwrap().data.
        println!("{}", c)
    }

    Ok(())



    // let user = User {
    //     username: "test_user".to_string(),
    //     password: "test_password".to_string(),
    // };
    //
    // let payload = serde_json::to_vec(&user)?;
    //
    // let event = EventData::json("user-logged-in", payload)?;
    //
    // client
    //     .append_to_stream("login-stream", ExpectedVersion::Any, event)
    //     .await?;
    //
    // let result: ReadResult<_> = client
    //     .read_stream("login-stream", &StreamPosition::Start, 1, true)
    //     .await?;
    //
    // match result {
    //     ReadResult::Ok(mut events) => {
    //         let resolved_event = events.next().unwrap().unwrap();
    //         let event = resolved_event.get_original_event();
    //         let data = event.data.as_json::<User>()?;
    //
    //         println!("User logged in: {}", data.username);
    //     }
    //     ReadResult::StreamNotFound { stream_name } => {
    //         println!("Stream not found: {}", stream_name);
    //     }
    //     ReadResult::Error { source } => {
    //         println!("Error: {}", source);
    //     }
    // }
    //
    // Ok(())
}

