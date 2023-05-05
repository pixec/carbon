use super::{Configuration, Environment};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bollard::{errors::Error::DockerResponseServerError, network::InspectNetworkOptions, Docker};

pub struct DockerEnvironment {
    config: Configuration,
    docker: Docker,
}

impl DockerEnvironment {
    fn new(config: Configuration) -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;

        Ok(Self {
            config: config,
            docker: docker,
        })
    }

    async fn configure_network(&self) -> Result<()> {
        match self
            .docker
            .inspect_network(
                "carbon_nw",
                Some(InspectNetworkOptions::<&str> {
                    ..Default::default()
                }),
            )
            .await
        {
            Ok(..) => Ok(()),
            Err(error) => match error {
                DockerResponseServerError { status_code, .. } => {
                    if status_code == 404 {
                        // TODO: Create Docker network if not found.
                    }

                    Ok(())
                }
                _ => Err(anyhow!(error)),
            },
        }
    }
}

#[async_trait]
impl Environment for DockerEnvironment {
    async fn configure(&self) -> Result<()> {
        self.configure_network().await?;

        Ok(())
    }
}
