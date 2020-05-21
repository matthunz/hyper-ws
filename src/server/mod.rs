use ws_async::WebSocket;
use tokio::task::JoinHandle;

mod service;
pub use service::WsService;

type UpgradeHandle = JoinHandle<hyper::Result<WebSocket>>;
