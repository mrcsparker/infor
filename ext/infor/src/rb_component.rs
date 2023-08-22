#[derive(Clone, Debug)]
#[magnus::wrap(class = "Infor::Component", free_immediately, size)]
pub struct RbComponent {
    pub temperature: f32,
    pub max: f32,
    pub critical: Option<f32>,
    pub label: String,
}

impl RbComponent {
    pub fn new(temperature: f32, max: f32, critical: Option<f32>, label: String) -> Self {
        Self {
            temperature,
            max,
            critical,
            label,
        }
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn critical(&self) -> Option<f32> {
        self.critical
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
        let hash = magnus::RHash::new();
        hash.aset("temperature", self.temperature())?;
        hash.aset("max", self.max())?;
        hash.aset("critical", self.critical())?;
        hash.aset("label", self.label())?;
        Ok(hash)
    }

    pub fn to_str(&self) -> Result<String, magnus::Error> {
        Ok(format!("{self:?}"))
    }
}
