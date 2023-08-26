use magnus::{class, method, Module, RModule};
use sysinfo::DiskExt;

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Infor::Disk", free_immediately, size)]
pub struct RbDisk {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
}

impl From<&sysinfo::Disk> for RbDisk {
    fn from(disk: &sysinfo::Disk) -> Self {
        Self {
            name: disk.name().to_str().unwrap_or("").to_string(),
            mount_point: disk.mount_point().to_str().unwrap_or("").to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable: disk.is_removable(),
        }
    }
}

impl RbDisk {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn mount_point(&self) -> String {
        self.mount_point.clone()
    }

    fn total_space(&self) -> u64 {
        self.total_space
    }

    fn available_space(&self) -> u64 {
        self.available_space
    }

    fn is_removable(&self) -> bool {
        self.is_removable
    }

    fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
        let hash = magnus::RHash::new();
        hash.aset("name", self.name())?;
        hash.aset("mount_point", self.mount_point())?;
        hash.aset("total_space", self.total_space())?;
        hash.aset("available_space", self.available_space())?;
        hash.aset("is_removable", self.is_removable())?;
        Ok(hash)
    }

    fn to_str(&self) -> Result<String, magnus::Error> {
        Ok(format!("{self:?}"))
    }
}

pub fn setup(namespace: RModule) -> Result<(), magnus::Error> {
    let disk_class = namespace.define_class("Disk", class::object())?;
    disk_class.define_method("name", method!(RbDisk::name, 0))?;
    disk_class.define_method("mount_point", method!(RbDisk::mount_point, 0))?;
    disk_class.define_method("total_space", method!(RbDisk::total_space, 0))?;
    disk_class.define_method("available_space", method!(RbDisk::available_space, 0))?;
    disk_class.define_method("is_removable", method!(RbDisk::is_removable, 0))?;
    disk_class.define_method("to_hash", method!(RbDisk::to_hash, 0))?;
    disk_class.define_method("_to_str", method!(RbDisk::to_str, 0))?;
    Ok(())
}
