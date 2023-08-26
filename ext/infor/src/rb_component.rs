use magnus::{class, method, Module, RModule};
use sysinfo::{Component, ComponentExt};

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Infor::Component", free_immediately, size)]
pub struct RbComponent {
    pub temperature: f32,
    pub max: f32,
    pub critical: Option<f32>,
    pub label: String,
}

impl From<&sysinfo::Component> for RbComponent {
    fn from(component: &sysinfo::Component) -> Self {
        Self {
            temperature: component.temperature(),
            max: component.max(),
            critical: component.critical(),
            label: component.label().to_string(),
        }
    }
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

    fn temperature(&self) -> f32 {
        self.temperature
    }

    fn max(&self) -> f32 {
        self.max
    }

    fn critical(&self) -> Option<f32> {
        self.critical
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
        let hash = magnus::RHash::new();
        hash.aset("temperature", self.temperature())?;
        hash.aset("max", self.max())?;
        hash.aset("critical", self.critical())?;
        hash.aset("label", self.label())?;
        Ok(hash)
    }

    fn to_str(&self) -> Result<String, magnus::Error> {
        Ok(format!("{self:?}"))
    }
}

pub fn setup(namespace: RModule) -> Result<(), magnus::Error> {
    let component_class = namespace.define_class("Component", class::object())?;
    component_class.define_method("temperature", method!(RbComponent::temperature, 0))?;
    component_class.define_method("max", method!(RbComponent::max, 0))?;
    component_class.define_method("critical", method!(RbComponent::critical, 0))?;
    component_class.define_method("label", method!(RbComponent::label, 0))?;
    component_class.define_method("to_hash", method!(RbComponent::to_hash, 0))?;
    component_class.define_method("_to_str", method!(RbComponent::to_str, 0))?;
    Ok(())
}
