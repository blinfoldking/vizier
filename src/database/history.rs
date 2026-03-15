use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    database::VizierDatabases,
    schema::{SessionHistory, SessionHistoryContent, VizierSession},
};

impl VizierDatabases {
    pub async fn save_session_history(
        &self,
        session: VizierSession,
        content: SessionHistoryContent,
    ) -> Result<()> {
        let uuid = Uuid::new_v4();
        let _: Option<SessionHistory> = self
            .conn
            .create(("session_history", uuid.clone().to_string()))
            .content(SessionHistory {
                uuid,
                vizier_session: session.clone(),
                content,
                timestamp: Utc::now(),
            })
            .await?;

        Ok(())
    }

    // TODO: cursor based pagination
    pub async fn list_session_history(
        &self,
        session: VizierSession,
    ) -> Result<Vec<SessionHistory>> {
        let mut response = self
            .conn
            .query(format!(
                "SELECT * FROM session_history WHERE vizier_session == $vizier_session ORDER BY timestamp ASC",
            ))
            .bind(("vizier_session", session.clone()))
            .await?;

        let list: Vec<SessionHistory> = response.take(0)?;

        Ok(list)
    }
}
