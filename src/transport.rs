use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use futures::future::join_all;
use strum::{EnumIter, IntoEnumIterator};

use crate::agent::session::VizierSession;

#[derive(Debug, Clone, Hash, PartialEq, Eq, EnumIter)]
pub enum VizierTransportChannel {
    Discord,
    API,
}

#[derive(Debug, Clone)]
pub struct VizierRequest {
    pub user: String,
    pub content: String,
}

impl VizierRequest {
    pub fn to_prompt(&self) -> String {
        format!("{}: {}", self.user, self.content)
    }
}

#[derive(Debug, Clone)]
pub enum VizierResponse {
    StartThinking,
    StopThinking,
    Message(String),
}

#[derive(Debug, Clone)]
pub struct VizierTransport {
    request_writer: Arc<flume::Sender<(VizierSession, VizierRequest)>>,
    request_reader: Arc<flume::Receiver<(VizierSession, VizierRequest)>>,

    response_writer: Arc<flume::Sender<(VizierSession, VizierResponse)>>,
    response_reader: Arc<flume::Receiver<(VizierSession, VizierResponse)>>,

    agent_transport: Arc<(
        flume::Sender<(VizierSession, VizierRequest)>,
        flume::Receiver<(VizierSession, VizierRequest)>,
    )>,
    channel_transport: HashMap<
        VizierTransportChannel,
        Arc<(
            flume::Sender<(VizierSession, VizierResponse)>,
            flume::Receiver<(VizierSession, VizierResponse)>,
        )>,
    >,
}

impl VizierTransport {
    pub fn new() -> Self {
        let (request_writer, request_reader) = flume::unbounded::<(VizierSession, VizierRequest)>();
        let (response_writer, response_reader) =
            flume::unbounded::<(VizierSession, VizierResponse)>();

        let agent_transport = Arc::new(flume::unbounded());

        let mut channel_transport = HashMap::new();
        for channel in VizierTransportChannel::iter() {
            channel_transport.insert(channel, Arc::new(flume::unbounded()));
        }

        Self {
            request_writer: Arc::new(request_writer),
            request_reader: Arc::new(request_reader),

            response_writer: Arc::new(response_writer),
            response_reader: Arc::new(response_reader),

            agent_transport,
            channel_transport,
        }
    }

    pub async fn send_request(&self, session: VizierSession, request: VizierRequest) -> Result<()> {
        self.request_writer.send((session, request))?;
        Ok(())
    }

    pub async fn send_response(
        &self,
        session: VizierSession,
        response: VizierResponse,
    ) -> Result<()> {
        self.response_writer.send((session, response))?;
        Ok(())
    }

    pub async fn read_request(&self) -> Result<(VizierSession, VizierRequest)> {
        let res = self.agent_transport.1.recv()?;

        Ok(res)
    }

    pub async fn read_response(
        &self,
        channel: VizierTransportChannel,
    ) -> Result<(VizierSession, VizierResponse)> {
        let res = self.channel_transport.get(&channel).unwrap().1.recv()?;

        Ok(res)
    }

    pub async fn run(&self) -> Result<()> {
        // transport request
        let request_reader = self.request_reader.clone();
        let agent_transport = self.agent_transport.clone();
        let agent_handle = tokio::spawn(async move {
            loop {
                if let Ok((session, request)) = request_reader.recv() {
                    // TODO: middleware here
                    log::info!("request {:?} -> {:?}", session, request);

                    let _ = agent_transport.0.send((session, request));
                }
            }
        });

        // transport per channels
        let channel_transport = self.channel_transport.clone();
        let response_reader = self.response_reader.clone();
        let channel_handle = tokio::spawn(async move {
            loop {
                if let Ok((session, response)) = response_reader.recv() {
                    // TODO middleware here
                    log::info!("response {:?} -> {:?}", session, response);

                    let channel = match session {
                        VizierSession::DiscordChanel(_) => VizierTransportChannel::Discord,
                        VizierSession::API(_) => VizierTransportChannel::API,
                    };

                    let _ = channel_transport
                        .get(&channel)
                        .unwrap()
                        .0
                        .send((session, response));
                }
            }
        });

        join_all(vec![agent_handle, channel_handle]).await;
        Ok(())
    }
}
