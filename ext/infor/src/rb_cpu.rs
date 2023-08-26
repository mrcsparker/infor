use magnus::{class, method, Module, RModule};
use sysinfo::CpuExt;

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Infor::Cpu", free_immediately, size)]
pub struct RbCpu {
    pub cpu_usage: f32,
    pub name: String,
    pub vendor_id: String,
    pub brand: String,
    pub frequency: u64,
}

impl From<&sysinfo::Cpu> for RbCpu {
    fn from(cpu: &sysinfo::Cpu) -> Self {
        Self {
            cpu_usage: cpu.cpu_usage(),
            name: cpu.name().to_string(),
            vendor_id: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
        }
    }
}

impl RbCpu {
    fn cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn vendor_id(&self) -> String {
        self.vendor_id.to_string()
    }

    fn brand(&self) -> String {
        self.brand.to_string()
    }

    fn frequency(&self) -> u64 {
        self.frequency
    }

    fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
        let hash = magnus::RHash::new();
        hash.aset("cpu_usage", self.cpu_usage())?;
        hash.aset("name", self.name())?;
        hash.aset("vendor_id", self.vendor_id())?;
        hash.aset("brand", self.brand())?;
        hash.aset("frequency", self.frequency())?;
        Ok(hash)
    }

    fn to_str(&self) -> Result<String, magnus::Error> {
        Ok(format!("{self:?}"))
    }
}

pub fn setup(namespace: RModule) -> Result<(), magnus::Error> {
    let cpu_class = namespace.define_class("Cpu", class::object())?;
    cpu_class.define_method("cpu_usage", method!(RbCpu::cpu_usage, 0))?;
    cpu_class.define_method("name", method!(RbCpu::name, 0))?;
    cpu_class.define_method("vendor_id", method!(RbCpu::vendor_id, 0))?;
    cpu_class.define_method("brand", method!(RbCpu::brand, 0))?;
    cpu_class.define_method("frequency", method!(RbCpu::frequency, 0))?;
    cpu_class.define_method("to_hash", method!(RbCpu::to_hash, 0))?;
    cpu_class.define_method("_to_str", method!(RbCpu::to_str, 0))?;
    Ok(())
}
