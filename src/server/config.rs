#[derive(Debug, Clone)]
pub struct Configuration {
    uuid: String,
}

impl Configuration {
    pub fn new(uuid: String) -> Self {
        Self { uuid: uuid }
    }

    pub fn uuid(&self) -> String {
        self.uuid.clone()
    }

    pub fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }
}
