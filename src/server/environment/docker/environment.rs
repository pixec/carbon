use crate::server::{environment::Configuration, Environment};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bollard::{
    errors::Error::DockerResponseServerError,
    network::{CreateNetworkOptions, InspectNetworkOptions},
    service::{Ipam, IpamConfig},
    Docker,
};

#[derive(Debug)]
pub struct DockerEnvironment {
    config: Configuration,
    docker: Docker,
}

impl DockerEnvironment {
    pub fn new(config: Configuration) -> Result<Self> {
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
                        self.docker
                            .create_network(CreateNetworkOptions::<&str> {
                                name: "carbon_nw",
                                driver: "bridge",
                                ipam: Ipam {
                                    config: Some(vec![
                                        IpamConfig {
                                            subnet: Some("172.18.0.0/16".to_string()),
                                            gateway: Some("172.18.0.1".to_string()),
                                            ..Default::default()
                                        },
                                        IpamConfig {
                                            subnet: Some("fdba:17c8:6c94::/64".to_string()),
                                            gateway: Some("fdba:17c8:6c94::1011".to_string()),
                                            ..Default::default()
                                        },
                                    ]),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .await?;
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

    async fn start(&self) -> Result<()> {
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }

    async fn terminate(&self) -> Result<()> {
        Ok(())
    }
}
