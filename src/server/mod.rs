use crate::Socket;
use futures::future;
use std::net::SocketAddr;
use std::task::{Context, Poll};
use tokio::sync::mpsc::Receiver;
use tokio::task::{self, JoinHandle};

mod factory;
pub use factory::WsFactory;

mod service;
pub use service::WsService;

type UpgradeHandle = JoinHandle<hyper::Result<Socket>>;

/// A WebSocket socket server, listening for connections.
/// ```
/// use hyper_ws::Server;
/// use tokio::stream::StreamExt;
///
/// # async {
/// let addr = "127.0.0.1:8080".parse()?;
/// let mut server = Server::bind(&addr)?;
///
/// while let Some(ref mut ws) = server.next_socket().await? {
///     while let Some(Ok(frame)) = ws.next().await {
///         dbg!(frame);
///     }    
/// }
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// # };
/// ```
pub struct Server {
    rx: Receiver<UpgradeHandle>,
}

impl Server {
    /// Creates a new Server which will be bound to the specified address.
    pub fn bind(addr: &SocketAddr) -> hyper::Result<Self> {
        let http = hyper::Server::try_bind(addr)?;
        let (make_svc, rx) = WsFactory::new();

        task::spawn(http.serve(make_svc));
        Ok(Self { rx })
    }

    pub async fn next_socket(&mut self) -> hyper::Result<Option<Socket>> {
        if let Some(handle) = self.next_upgrade().await {
            // TODO don't unwrap
            let ws = handle.await.unwrap()?;
            Ok(Some(ws))
        } else {
            Ok(None)
        }
    }

    pub async fn next_upgrade(&mut self) -> Option<UpgradeHandle> {
        future::poll_fn(|cx| self.poll_upgrade(cx)).await
    }

    pub fn poll_upgrade(&mut self, cx: &mut Context) -> Poll<Option<UpgradeHandle>> {
        self.rx.poll_recv(cx)
    }
}
