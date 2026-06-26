use sentinelone_api::models::Agent as AgentData;
use sentinelone_api::services::agent_actions::AgentFilter;
use sentinelone_api::services::agents::AgentsQuery;

use crate::{Error, SentinelOne};

/// High-level Agent. Bundles the raw [`AgentData`] with a [`SentinelOne`] handle
/// so actions like [`Agent::disconnect`] route through the configured backend.
pub struct Agent {
    s1: SentinelOne,
    data: AgentData,
}

impl Agent {
    /// Fetch a single agent by id/uuid. The mgmt API has no get-by-id route, so
    /// this filters the list endpoint by `ids` and takes the first match.
    pub(crate) async fn fetch(s1: SentinelOne, id: String) -> Result<Self, Error> {
        let query = AgentsQuery::default().ids([&id]).limit(1);
        let page = s1.management()?.agents().list(&query).await?;
        let data = page
            .data
            .into_iter()
            .next()
            .ok_or(Error::Management(sentinelone_api::Error::NotFound))?;
        Ok(Self { s1, data })
    }

    // --- accessors over cached data ---

    pub fn id(&self) -> &str {
        &self.data.id
    }
    pub fn computer_name(&self) -> Option<&str> {
        self.data.computer_name.as_deref()
    }
    pub fn is_active(&self) -> bool {
        self.data.is_active.unwrap_or(false)
    }
    /// Borrow the underlying raw model.
    pub fn data(&self) -> &AgentData {
        &self.data
    }

    // --- actions ---

    /// Disconnect this agent from the network.
    pub async fn disconnect(&self) -> Result<(), Error> {
        let filter = AgentFilter::ids([&self.data.id]);
        self.s1.management()?.agent_actions().disconnect(&filter).await?;
        Ok(())
    }

    /// Re-fetch this agent's state from the server.
    pub async fn refresh(&mut self) -> Result<(), Error> {
        let query = AgentsQuery::default().ids([&self.data.id]).limit(1);
        let page = self.s1.management()?.agents().list(&query).await?;
        if let Some(data) = page.data.into_iter().next() {
            self.data = data;
        }
        Ok(())
    }
}
