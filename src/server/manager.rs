use anyhow::Result;

pub struct Manager {}

impl Manager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn initialize(&self) -> Result<()> {
        // TODO: Fetch server configurations.
        Ok(())
    }
}