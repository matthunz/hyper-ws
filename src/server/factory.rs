use super::{UpgradeHandle, WsService};
use std::future::Future;
use std::task::{Context, Poll};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tower_service::Service;

pub struct WsFactory {
    tx: Sender<UpgradeHandle>,
}

impl WsFactory {
    pub fn new() -> (Self, Receiver<UpgradeHandle>) {
        let (tx, rx) = mpsc::channel(1);
        (Self { tx }, rx)
    }
}

impl<T> Service<T> for WsFactory {
    type Response = WsService;
    type Error = hyper::Error;
    type Future = impl Future<Output = hyper::Result<Self::Response>>;

    fn poll_ready(&mut self, _cx: &mut Context) -> Poll<hyper::Result<()>> {
        Ok(()).into()
    }

    fn call(&mut self, _req: T) -> Self::Future {
        let tx = self.tx.clone();
        async move { Ok(WsService::new(tx)) }
    }
}
