use super::environment::Configuration;
use crate::server::environment::DockerEnvironment;
use crate::server::Environment;
use crate::server::Filesystem;
use anyhow::Result;

#[derive(Debug)]
pub struct Server {
    uuid: String,
    environment: Box<dyn Environment>,
    filesystem: Filesystem,
}

pub enum Power {
    Start,
    Stop,
    Terminate,
}

impl Server {
    pub fn new() -> Result<Self> {
        Ok(Self {
            uuid: String::from("S O O N"),
            environment: Box::new(DockerEnvironment::new(Configuration {})?),
            filesystem: Filesystem::new(),
        })
    }

    pub fn uuid(&self) -> String {
        self.uuid.clone()
    }

    pub fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
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
