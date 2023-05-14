mod config;
mod docker;

pub use config::Configuration;
pub use docker::DockerEnvironment;

use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait Environment {
    async fn configure(&self) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn terminate(&self) -> Result<()>;
}

impl Debug for dyn Environment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Environment{{}}")
    }
}
