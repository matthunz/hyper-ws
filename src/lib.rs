use hyper::upgrade::Upgraded;

pub mod client;
pub use client::Client;

pub mod server;
pub use server::Server;

pub type Payload = ws_async::frame::Payload<Upgraded>;

pub type Frame = ws_async::frame::Frame<Payload>;

pub type WebSocket = ws_async::WebSocket<Upgraded>;

pub async fn upgrade(body: hyper::Body) -> hyper::Result<WebSocket> {
    body.on_upgrade()
        .await
        .map(WebSocket::from_upgraded)
        .map_err(Into::into)
}
