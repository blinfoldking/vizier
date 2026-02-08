use std::sync::Arc;

use crate::agent::session::VizierSession;

#[derive(Debug, Clone)]
pub struct VizierRequest {
    pub content: String,
}
#[derive(Debug, Clone)]
pub enum VizierResponse {
    StartThinking,
    StopThinking,
    Message(String),
}

#[derive(Debug, Clone)]
pub struct VizierTransport {
    pub request_writer: Arc<flume::Sender<(VizierSession, VizierRequest)>>,
    pub request_reader: Arc<flume::Receiver<(VizierSession, VizierRequest)>>,

    pub response_writer: Arc<flume::Sender<(VizierSession, VizierResponse)>>,
    pub response_reader: Arc<flume::Receiver<(VizierSession, VizierResponse)>>,
}

impl VizierTransport {
    pub fn new() -> Self {
        let (request_writer, request_reader) = flume::unbounded::<(VizierSession, VizierRequest)>();
        let (response_writer, response_reader) =
            flume::unbounded::<(VizierSession, VizierResponse)>();

        Self {
            request_writer: Arc::new(request_writer),
            request_reader: Arc::new(request_reader),

            response_writer: Arc::new(response_writer),
            response_reader: Arc::new(response_reader),
        }
    }
}
