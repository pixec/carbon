use super::environment::Configuration as EnvironmentConfiguration;
use crate::server::environment::DockerEnvironment;
use crate::server::Configuration;
use crate::server::Environment;
use crate::server::Filesystem;
use anyhow::Result;
use parking_lot::Mutex;
use parking_lot::MutexGuard;

#[derive(Debug)]
pub struct Server {
    configuration: Mutex<Configuration>,
    pub environment: Box<dyn Environment>,
    pub filesystem: Filesystem,
}

pub enum Power {
    Start,
    Stop,
    Terminate,
}

impl Server {
    pub fn new() -> Result<Self> {
        Ok(Self {
            configuration: Mutex::new(Configuration::new("SOON".to_string())),
            environment: Box::new(DockerEnvironment::new(EnvironmentConfiguration {})?),
            filesystem: Filesystem::new(),
        })
    }

    pub fn config(&self) -> MutexGuard<Configuration> {
        self.configuration.lock()
    }

    pub async fn handle_power(&self, power: Power) -> Result<()> {
        match power {
            Power::Start => self.environment.start().await?,
            Power::Stop => self.environment.start().await?,
            Power::Terminate => self.environment.start().await?,
        };

        Ok(())
    }
}
