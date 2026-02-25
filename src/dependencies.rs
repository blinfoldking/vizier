use anyhow::Result;

use crate::{config::VizierConfig, database::VizierDatabases, transport::VizierTransport};

#[derive(Debug, Clone)]
pub struct VizierDependencies {
    pub transport: VizierTransport,
    pub database: VizierDatabases,
}

impl VizierDependencies {
    pub async fn new(config: VizierConfig) -> Result<Self> {
        Ok(Self {
            database: VizierDatabases::new(config.workspace.clone()).await?,
            transport: VizierTransport::new(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        self.transport.run().await?;

        Ok(())
    }
}
