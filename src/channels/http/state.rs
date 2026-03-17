use std::sync::Arc;

use crate::{config::VizierConfig, database::VizierDatabases, transport::VizierTransport};

#[derive(Debug, Clone)]
pub struct HTTPState {
    pub config: Arc<VizierConfig>,
    pub transport: VizierTransport,
    pub db: VizierDatabases,
}
