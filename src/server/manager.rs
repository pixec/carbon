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

    pub fn find_mut(&mut self, uuid: String) -> Result<&mut Server> {
        self.servers
            .iter_mut()
            .find(|server| server.uuid() == uuid)
            .ok_or(anyhow!("server not found"))
    }
}
