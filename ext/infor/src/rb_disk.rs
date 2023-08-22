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
    pub fn to_string(&self) -> String {
        format!(
            "Disk(name={}, mount_point={}, total_space={}, available_space={}, is_removable={})",
            self.name, self.mount_point, self.total_space, self.available_space, self.is_removable
        )
    }
}
