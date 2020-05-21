use tokio::stream::StreamExt;
use hyper_ws::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let mut server = Server::bind(&addr)?;

    while let Some(ref mut ws) = server.next_socket().await? {
        while let Some(Ok(frame)) = ws.next().await {
            let mut payload = frame.into_payload();

            while let Some(res) = payload.next().await {
                let bytes = res?;
                let s = std::str::from_utf8(&bytes)?;
                dbg!(s);
            }
        }
    }

    Ok(())
}