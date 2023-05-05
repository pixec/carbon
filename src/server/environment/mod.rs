mod config;
mod docker;

pub use config::Configuration;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Environment {
    async fn configure(&self) -> Result<()>;
}
