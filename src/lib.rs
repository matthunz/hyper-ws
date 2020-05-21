use hyper::upgrade::Upgraded;

pub mod server;
pub use server::Server;

pub type WebSocket = ws_async::WebSocket<Upgraded>;

pub async fn upgrade(body: hyper::Body) -> hyper::Result<WebSocket> {
    body.on_upgrade()
        .await
        .map(WebSocket::from_upgraded)
        .map_err(Into::into)
}
