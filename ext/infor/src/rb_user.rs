#[derive(Clone, Debug)]
#[magnus::wrap(class = "Infor::User", free_immediately, size)]
pub struct RbUser {
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub groups: Vec<String>,
}

impl RbUser {
    pub fn new(id: String, group_id: String, name: String, groups: Vec<String>) -> Self {
        Self {
            id,
            group_id,
            name,
            groups,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn group_id(&self) -> String {
        self.group_id.to_string()
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn groups(&self) -> Vec<String> {
        self.groups.clone()
    }

    pub fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
        let hash = magnus::RHash::new();
        hash.aset("id", self.id())?;
        hash.aset("group_id", self.group_id())?;
        hash.aset("name", self.name())?;
        hash.aset("groups", self.groups())?;
        Ok(hash)
    }

    pub fn to_str(&self) -> Result<String, magnus::Error> {
        Ok(format!("{self:?}"))
    }
}
