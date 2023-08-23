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

    pub fn interface(&self) -> String {
        self.interface.to_string()
    }

    pub fn received(&self) -> u64 {
        self.received
    }

    pub fn total_received(&self) -> u64 {
        self.total_received
    }

    pub fn transmitted(&self) -> u64 {
        self.transmitted
    }

    pub fn total_transmitted(&self) -> u64 {
        self.total_transmitted
    }

    pub fn packets_received(&self) -> u64 {
        self.packets_received
    }

    pub fn total_packets_received(&self) -> u64 {
        self.total_packets_received
    }

    pub fn packets_transmitted(&self) -> u64 {
        self.packets_transmitted
    }

    pub fn total_packets_transmitted(&self) -> u64 {
        self.total_packets_transmitted
    }

    pub fn errors_on_received(&self) -> u64 {
        self.errors_on_received
    }

    pub fn total_errors_on_received(&self) -> u64 {
        self.total_errors_on_received
    }

    pub fn errors_on_transmitted(&self) -> u64 {
        self.errors_on_transmitted
    }

    pub fn total_errors_on_transmitted(&self) -> u64 {
        self.total_errors_on_transmitted
    }

    pub fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
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

    pub fn to_str(&self) -> Result<String, magnus::Error> {
        Ok(format!("{self:?}"))
    }
}
