use std::sync::Arc;

use anyhow::Result;

use crate::{
    config::{VizierConfig, storage::StorageConfig},
    embedding::VizierEmbedder,
    shell::VizierShell,
    storage::{VizierStorage, fs::FileSystemStorage, surreal::SurrealStorage},
    transport::VizierTransport,
};

#[derive(Clone)]
pub struct VizierDependencies {
    pub config: Arc<VizierConfig>,
    pub embedder: Option<Arc<VizierEmbedder>>,
    pub transport: VizierTransport,
    pub storage: Arc<VizierStorage>,
    pub shell: Arc<VizierShell>,
}

impl VizierDependencies {
    pub async fn new(config: VizierConfig) -> Result<Self> {
        let embedder = if config.embedding.is_some() {
            Some(Arc::new(VizierEmbedder::new(&config).await?))
        } else {
            None
        };

        let storage = match config.storage {
            StorageConfig::Filesystem => {
                let fs = FileSystemStorage::new(config.workspace.clone(), embedder.clone()).await?;
                VizierStorage::new(fs)
            }
            StorageConfig::Surreal => {
                let surreal =
                    SurrealStorage::new(config.workspace.clone(), embedder.clone()).await?;
                VizierStorage::new(surreal)
            }
        };

        let shell = Arc::new(VizierShell::new(&config.shell).await?);

        Ok(Self {
            config: Arc::new(config.clone()),
            storage: Arc::new(VizierStorage::new(storage)),
            transport: VizierTransport::new(),
            embedder,
            shell,
        })
    }

    pub async fn run(&self) -> Result<()> {
        self.transport.run().await?;

        Ok(())
    }
}
