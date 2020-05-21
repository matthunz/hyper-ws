use tokio::task::JoinHandle;
use ws_async::WebSocket;

mod factory;
pub use factory::WsFactory;

mod service;
pub use service::WsService;

type UpgradeHandle = JoinHandle<hyper::Result<WebSocket>>;
