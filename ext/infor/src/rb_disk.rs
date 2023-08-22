#[derive(Clone)]
#[magnus::wrap(class = "Infor::Disk", free_immediately, size)]
pub struct RbDisk {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
}

impl RbDisk {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn mount_point(&self) -> String {
        self.mount_point.clone()
    }

    pub fn total_space(&self) -> u64 {
        self.total_space
    }

    pub fn available_space(&self) -> u64 {
        self.available_space
    }

    pub fn is_removable(&self) -> bool {
        self.is_removable
    }

    pub fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
        let hash = magnus::RHash::new();
        hash.aset("name", self.name())?;
        hash.aset("mount_point", self.mount_point())?;
        hash.aset("total_space", self.total_space())?;
        hash.aset("available_space", self.available_space())?;
        hash.aset("is_removable", self.is_removable())?;
        Ok(hash)
    }
}
