use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::hashes::HashVerdict;
use crate::pagination::Response;

/// `Hashes` tag — Hashes related endpoints.
pub struct HashesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

impl HashesService<'_> {
    /// `GET /web/api/v2.1/hashes/{hash}/verdict` — Hash Reputation verdict.
    ///
    /// [DEPRECATED] Get the verdict of the of a hash, given the required SHA1.
    /// A hash, either malicious or non-malicious, means it has been marked as
    /// such by the Reputation's sources. An unknown answer is given for hashes
    /// that are not yet known by Reputation.
    ///
    /// # Arguments
    ///
    /// * `hash` — Hash (path, required).
    pub async fn verdict(
        &self,
        hash: impl Into<String>,
    ) -> Result<Response<HashVerdict>, Error> {
        let path = format!("/web/api/v2.1/hashes/{}/verdict", hash.into());
        Ok(self.client.http().get(&path, None).await?)
    }
}
