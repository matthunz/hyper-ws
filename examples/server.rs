use hyper_ws::Server;
use tokio::stream::StreamExt;
use ws_async::frame::Frame;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let mut server = Server::bind(&addr)?;

    while let Some(ref mut ws) = server.next_socket().await? {
        while let Some(Ok(frame)) = ws.next().await {
            let mut payload = frame.payload;

            while let Some(res) = payload.next().await {
                let bytes = res?;
                let s = std::str::from_utf8(&bytes)?;
                dbg!(s);
            }

            ws.send_frame(Frame::text("Hello World!".as_bytes()))
                .await?;
        }
    }

    Ok(())
}
