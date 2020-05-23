use hyper::upgrade::Upgraded;

pub mod client;
pub use client::Client;

pub mod server;
pub use server::Server;

pub use ws_async::frame::Opcode;

pub type Payload<T = Upgraded> = ws_async::frame::Payload<T>;

pub type Frame<P = Payload> = ws_async::frame::Frame<P>;

pub type Socket<T = Upgraded> = ws_async::Socket<T>;

pub async fn upgrade(body: hyper::Body) -> hyper::Result<Socket> {
    body.on_upgrade()
        .await
        .map(Socket::from_upgraded)
        .map_err(Into::into)
}
