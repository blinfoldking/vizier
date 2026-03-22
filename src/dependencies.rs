use std::sync::Arc;

use anyhow::Result;

use crate::{
    config::{VizierConfig, storage::StorageConfig},
    embedding::VizierEmbedder,
    storage::{VizierStorage, fs::FileSystemStorage, surreal::SurrealStorage},
    transport::VizierTransport,
};

#[derive(Clone)]
pub struct VizierDependencies {
    pub config: Arc<VizierConfig>,
    pub embedder: Option<Arc<VizierEmbedder>>,
    pub transport: VizierTransport,
    pub storage: Arc<VizierStorage>,
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

        Ok(Self {
            config: Arc::new(config.clone()),
            storage: Arc::new(VizierStorage::new(storage)),
            transport: VizierTransport::new(),
            embedder,
        })
    }

    pub async fn run(&self) -> Result<()> {
        self.transport.run().await?;

        Ok(())
    }
}
