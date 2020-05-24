use hyper_ws::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new();

    let uri = "ws://127.0.0.1:8080".parse()?;
    let mut ws = client.connect(uri).await?;

    while let Some(Ok(frame)) = ws.next_frame().await {
        let mut payload = frame.payload;

        dbg!(payload.bytes().await?);
    }

    Ok(())
}
