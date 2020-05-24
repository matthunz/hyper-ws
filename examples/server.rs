use hyper_ws::Server;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let mut server = Server::bind(&addr)?;

    while let Some(ref mut ws) = server.next_socket().await? {
        while let Some(Ok(frame)) = ws.next().await {
            ws.send_stream(frame).await?;
        }
    }

    Ok(())
}
