use anyhow::Result;

use crate::{
    storage::{
        fs::{FileSystemStorage, STATE_PATH},
        state::StateStorage,
    },
    utils::build_path,
};

#[async_trait::async_trait]
impl StateStorage for FileSystemStorage {
    async fn save_state(&self, key: String, value: serde_json::Value) -> Result<()> {
        let mut path = build_path(&self.workspace, &[STATE_PATH]);
        let _ = std::fs::create_dir_all(&path)?;
        path.push(format!("{}.json", key));
        std::fs::write(path, serde_json::to_string_pretty(&value)?)?;

        Ok(())
    }

    async fn get_state(&self, key: String) -> Result<Option<serde_json::Value>> {
        let path = build_path(&self.workspace, &[STATE_PATH, &format!("{}.json", key)]);

        if let Ok(raw) = std::fs::read_to_string(&path) {
            let res = serde_json::from_str(&raw)?;

            return Ok(Some(res));
        }

        Ok(None)
    }
}
