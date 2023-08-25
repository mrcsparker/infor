use magnus::{class, method, Module, RModule};

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Infor::Network", free_immediately, size)]
pub struct RbNetwork {
    pub interface: String,
    pub received: u64,
    pub total_received: u64,
    pub transmitted: u64,
    pub total_transmitted: u64,
    pub packets_received: u64,
    pub total_packets_received: u64,
    pub packets_transmitted: u64,
    pub total_packets_transmitted: u64,
    pub errors_on_received: u64,
    pub total_errors_on_received: u64,
    pub errors_on_transmitted: u64,
    pub total_errors_on_transmitted: u64,
    //mac_address: RbMacAddr;
}

impl RbNetwork {
    pub fn new(
        interface: String,
        received: u64,
        total_received: u64,
        transmitted: u64,
        total_transmitted: u64,
        packets_received: u64,
        total_packets_received: u64,
        packets_transmitted: u64,
        total_packets_transmitted: u64,
        errors_on_received: u64,
        total_errors_on_received: u64,
        errors_on_transmitted: u64,
        total_errors_on_transmitted: u64,
    ) -> Self {
        Self {
            interface,
            received,
            total_received,
            transmitted,
            total_transmitted,
            packets_received,
            total_packets_received,
            packets_transmitted,
            total_packets_transmitted,
            errors_on_received,
            total_errors_on_received,
            errors_on_transmitted,
            total_errors_on_transmitted,
        }
    }

    fn interface(&self) -> String {
        self.interface.to_string()
    }

    fn received(&self) -> u64 {
        self.received
    }

    fn total_received(&self) -> u64 {
        self.total_received
    }

    fn transmitted(&self) -> u64 {
        self.transmitted
    }

    fn total_transmitted(&self) -> u64 {
        self.total_transmitted
    }

    fn packets_received(&self) -> u64 {
        self.packets_received
    }

    fn total_packets_received(&self) -> u64 {
        self.total_packets_received
    }

    fn packets_transmitted(&self) -> u64 {
        self.packets_transmitted
    }

    fn total_packets_transmitted(&self) -> u64 {
        self.total_packets_transmitted
    }

    fn errors_on_received(&self) -> u64 {
        self.errors_on_received
    }

    fn total_errors_on_received(&self) -> u64 {
        self.total_errors_on_received
    }

    fn errors_on_transmitted(&self) -> u64 {
        self.errors_on_transmitted
    }

    fn total_errors_on_transmitted(&self) -> u64 {
        self.total_errors_on_transmitted
    }

    fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
        let hash = magnus::RHash::new();
        hash.aset("interface", self.interface())?;
        hash.aset("received", self.received())?;
        hash.aset("total_received", self.total_received())?;
        hash.aset("transmitted", self.transmitted())?;
        hash.aset("total_transmitted", self.total_transmitted())?;
        hash.aset("packets_received", self.packets_received())?;
        hash.aset("total_packets_received", self.total_packets_received())?;
        hash.aset("packets_transmitted", self.packets_transmitted())?;
        hash.aset(
            "total_packets_transmitted",
            self.total_packets_transmitted(),
        )?;
        hash.aset("errors_on_received", self.errors_on_received())?;
        hash.aset("total_errors_on_received", self.total_errors_on_received())?;
        hash.aset("errors_on_transmitted", self.errors_on_transmitted())?;
        hash.aset(
            "total_errors_on_transmitted",
            self.total_errors_on_transmitted(),
        )?;
        Ok(hash)
    }

    fn to_str(&self) -> Result<String, magnus::Error> {
        Ok(format!("{self:?}"))
    }
}

pub fn setup(namespace: RModule) -> Result<(), magnus::Error> {
    let network_class = namespace.define_class("Network", class::object())?;
    network_class.define_method("interface", method!(RbNetwork::interface, 0))?;
    network_class.define_method("received", method!(RbNetwork::received, 0))?;
    network_class.define_method("total_received", method!(RbNetwork::total_received, 0))?;
    network_class.define_method("transmitted", method!(RbNetwork::transmitted, 0))?;
    network_class.define_method(
        "total_transmitted",
        method!(RbNetwork::total_transmitted, 0),
    )?;
    network_class.define_method("packets_received", method!(RbNetwork::packets_received, 0))?;
    network_class.define_method(
        "total_packets_received",
        method!(RbNetwork::total_packets_received, 0),
    )?;
    network_class.define_method(
        "packets_transmitted",
        method!(RbNetwork::packets_transmitted, 0),
    )?;
    network_class.define_method(
        "total_packets_transmitted",
        method!(RbNetwork::total_packets_transmitted, 0),
    )?;
    network_class.define_method(
        "errors_on_received",
        method!(RbNetwork::errors_on_received, 0),
    )?;
    network_class.define_method(
        "total_errors_on_received",
        method!(RbNetwork::total_errors_on_received, 0),
    )?;
    network_class.define_method(
        "errors_on_transmitted",
        method!(RbNetwork::errors_on_transmitted, 0),
    )?;
    network_class.define_method(
        "total_errors_on_transmitted",
        method!(RbNetwork::total_errors_on_transmitted, 0),
    )?;
    network_class.define_method("to_hash", method!(RbNetwork::to_hash, 0))?;
    network_class.define_method("_to_str", method!(RbNetwork::to_str, 0))?;
    Ok(())
}
