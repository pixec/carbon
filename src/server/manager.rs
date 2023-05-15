use super::Server;
use anyhow::{anyhow, Result};

pub struct Manager {
    servers: Vec<Server>,
}

impl Manager {
    pub fn new() -> Self {
        Self { servers: vec![] }
    }

    pub fn initialize(&mut self) -> Result<()> {
        self.servers.push(Server::new()?);

        Ok(())
    }

    pub fn find(&self, uuid: String) -> Result<&Server> {
        self.servers
            .iter()
            .find(|server| server.config().uuid() == uuid)
            .ok_or(anyhow!("server not found"))
    }
}
