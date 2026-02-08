use rig::{
    Embed,
    client::{Client, Nothing},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Embed, Clone)]
pub struct VizierMemory {
    #[embed]
    content: String,
    timestamp: String,
}

fn setup_mongo() {}
