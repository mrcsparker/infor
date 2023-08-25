use magnus::{class, method, Module, RModule};

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Infor::LoadAvg", free_immediately, size)]
pub struct RbLoadAvg {
    /// Average load within one minute.
    pub one: f64,
    /// Average load within five minutes.
    pub five: f64,
    /// Average load within fifteen minutes.
    pub fifteen: f64,
}

impl RbLoadAvg {
    pub fn new(one: f64, five: f64, fifteen: f64) -> Self {
        Self { one, five, fifteen }
    }

    fn one(&self) -> f64 {
        self.one
    }

    fn five(&self) -> f64 {
        self.five
    }

    fn fifteen(&self) -> f64 {
        self.fifteen
    }

    fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
        let hash = magnus::RHash::new();
        hash.aset("one", self.one)?;
        hash.aset("five", self.five)?;
        hash.aset("fifteen", self.fifteen)?;
        Ok(hash)
    }

    fn to_str(&self) -> Result<String, magnus::Error> {
        Ok(format!("{self:?}"))
    }
}

pub fn setup(namespace: RModule) -> Result<(), magnus::Error> {
    let load_avg_class = namespace.define_class("LoadAvg", class::object())?;
    load_avg_class.define_method("one", method!(RbLoadAvg::one, 0))?;
    load_avg_class.define_method("five", method!(RbLoadAvg::five, 0))?;
    load_avg_class.define_method("fifteen", method!(RbLoadAvg::fifteen, 0))?;
    load_avg_class.define_method("to_hash", method!(RbLoadAvg::to_hash, 0))?;
    load_avg_class.define_method("_to_str", method!(RbLoadAvg::to_str, 0))?;
    Ok(())
}
