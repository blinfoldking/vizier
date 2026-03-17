use std::sync::Arc;

use anyhow::Result;
use async_broadcast::{Receiver, Sender, broadcast};
use tokio::task::JoinSet;

use crate::schema::{VizierRequest, VizierResponse, VizierSession};

#[derive(Debug, Clone)]
pub struct VizierTransport {
    request_channel: Arc<(
        Sender<(VizierSession, VizierRequest)>,
        Receiver<(VizierSession, VizierRequest)>,
    )>,

    response_channel: Arc<(
        Sender<(VizierSession, VizierResponse)>,
        Receiver<(VizierSession, VizierResponse)>,
    )>,
}

impl VizierTransport {
    pub fn new() -> Self {
        let request_channel = Arc::new(broadcast(100));
        let response_channel = Arc::new(broadcast(100));

        Self {
            request_channel,
            response_channel,
        }
    }

    pub async fn send_request(&self, session: VizierSession, req: VizierRequest) -> Result<()> {
        self.request_channel.0.broadcast((session, req)).await?;
        Ok(())
    }

    pub async fn subscribe_request(&self) -> Result<Receiver<(VizierSession, VizierRequest)>> {
        Ok(self.request_channel.1.clone())
    }

    pub async fn send_response(&self, session: VizierSession, res: VizierResponse) -> Result<()> {
        self.response_channel.0.broadcast((session, res)).await?;
        Ok(())
    }

    pub async fn subscribe_response(&self) -> Result<Receiver<(VizierSession, VizierResponse)>> {
        Ok(self.response_channel.1.clone())
    }

    pub async fn run(&self) -> Result<()> {
        let mut set = JoinSet::new();

        // log all request
        let mut req_rx = self.request_channel.1.clone();
        set.spawn(async move {
            while let Ok((session, req)) = req_rx.recv().await {
                log::info!("[Request]: {:?} {:?}", session, req);
            }
        });

        let mut res_rx = self.response_channel.1.clone();
        set.spawn(async move {
            while let Ok((session, res)) = res_rx.recv().await {
                log::info!("[Response]: {:?} {:?}", session, res);
            }
        });

        set.join_all().await;
        Ok(())
    }
}
