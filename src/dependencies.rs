use std::sync::Arc;

use anyhow::Result;

use crate::{config::VizierConfig, database::VizierDatabases, transport::VizierTransport};

#[derive(Debug, Clone)]
pub struct VizierDependencies {
    pub config: Arc<VizierConfig>,
    pub transport: VizierTransport,
    pub database: VizierDatabases,
}

impl VizierDependencies {
    pub async fn new(config: VizierConfig) -> Result<Self> {
        Ok(Self {
            config: Arc::new(config.clone()),
            database: VizierDatabases::new(config.workspace.clone()).await?,
            transport: VizierTransport::new(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        self.transport.run().await?;

        Ok(())
    }
}
