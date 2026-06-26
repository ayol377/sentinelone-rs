//! Quickstart example.
//!
//! ```bash
//! export S1_MGMT_HOST=https://apse1-2111-mssp.sentinelone.net
//! export S1_MGMT_TOKEN=...        # ApiToken
//! # optional XDR:
//! export S1_XDR_HOST=https://xdr.ap1.sentinelone.net
//! export S1_XDR_TOKEN=...         # Bearer
//! cargo run -p sentinelone-rs --example quickstart -- <AGENT_UUID>
//! ```

use sentinelone::SentinelOne;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = SentinelOne::builder()
        .management(std::env::var("S1_MGMT_HOST")?, std::env::var("S1_MGMT_TOKEN")?);

    if let (Ok(host), Ok(token)) = (std::env::var("S1_XDR_HOST"), std::env::var("S1_XDR_TOKEN")) {
        builder = builder.xdr(host, token);
    }
    let s1 = builder.build()?;

    if let Some(agent_id) = std::env::args().nth(1) {
        let agent = s1.agent(&agent_id).await?;
        println!(
            "agent {} = {:?} (active: {})",
            agent.id(),
            agent.computer_name(),
            agent.is_active()
        );
        // agent.disconnect().await?;   // uncomment to act
    } else {
        eprintln!("pass an agent UUID to fetch one; skipping.");
    }

    Ok(())
}
