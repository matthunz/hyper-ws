use super::UpgradeHandle;
use hyper::{header, Body, Request, Response, StatusCode};
use std::future::Future;
use std::task::{Context, Poll};
use tokio::sync::mpsc::Sender;
use tokio::task;
use tower_service::Service;
use ws_async::{handshake, Error, Result, WebSocket};

pub struct WsService {
    tx: Sender<UpgradeHandle>,
}

impl WsService {
    pub fn new(tx: Sender<UpgradeHandle>) -> Self {
        Self { tx }
    }
}

impl Service<Request<Body>> for WsService {
    type Response = Response<Body>;
    type Error = Error;
    type Future = impl Future<Output = Result<Self::Response>> + Send + Sync + 'static;

    fn poll_ready(&mut self, _cx: &mut Context) -> Poll<Result<()>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut tx = self.tx.clone();

        async move {
            if let Some(key) = req.headers().get(handshake::SEC_WEBSOCKET_KEY) {
                // TODO don't unwrap
                let accept = handshake::accept(key);
                let res = Response::builder()
                    .status(StatusCode::SWITCHING_PROTOCOLS)
                    .header(header::CONNECTION, header::UPGRADE)
                    .header(header::UPGRADE, "websocket")
                    .header(handshake::SEC_WEBSOCKET_ACCEPT, accept)
                    .body(Body::empty())
                    .unwrap();

                let handle = task::spawn(WebSocket::upgrade(req.into_body()));
                if let Err(_) = tx.send(handle).await {
                    todo!()
                }

                Ok(res)
            } else {
                unimplemented!()
            }
        }
    }
}
