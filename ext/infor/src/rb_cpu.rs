#[derive(Clone)]
#[magnus::wrap(class = "Infor::Cpu", free_immediately, size)]
pub struct RbCpu {
    pub cpu_usage: f32,
    pub name: String,
    pub vendor_id: String,
    pub brand: String,
    pub frequency: u64,
}

impl RbCpu {
    pub fn cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn vendor_id(&self) -> String {
        self.vendor_id.to_string()
    }

    pub fn brand(&self) -> String {
        self.brand.to_string()
    }

    pub fn frequency(&self) -> u64 {
        self.frequency
    }

    pub fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
        let hash = magnus::RHash::new();
        hash.aset("cpu_usage", self.cpu_usage())?;
        hash.aset("name", self.name())?;
        hash.aset("vendor_id", self.vendor_id())?;
        hash.aset("brand", self.brand())?;
        hash.aset("frequency", self.frequency())?;
        Ok(hash)
    }
}
