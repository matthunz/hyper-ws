use hyper_ws::{Frame, Server};
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let mut server = Server::bind(&addr)?;

    while let Some(ref mut ws) = server.next_socket().await? {
        while let Some(Ok(mut frame)) = ws.next().await {
            let bytes = frame.payload.bytes().await?;

            ws.send_frame(Frame::new(frame.op, frame.rsv, bytes))
                .await?;
        }
    }

    Ok(())
}
