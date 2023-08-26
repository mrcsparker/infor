use magnus::{class, method, Module, RModule};
use sysinfo::UserExt;

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Infor::User", free_immediately, size)]
pub struct RbUser {
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub groups: Vec<String>,
}

impl From<&sysinfo::User> for RbUser {
    fn from(user: &sysinfo::User) -> Self {
        Self {
            id: user.id().to_string(),
            group_id: user.group_id().to_string(),
            name: user.name().to_string(),
            groups: user.groups().to_vec(),
        }
    }
}

impl RbUser {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn group_id(&self) -> String {
        self.group_id.to_string()
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn groups(&self) -> Vec<String> {
        self.groups.clone()
    }

    fn to_hash(&self) -> Result<magnus::RHash, magnus::Error> {
        let hash = magnus::RHash::new();
        hash.aset("id", self.id())?;
        hash.aset("group_id", self.group_id())?;
        hash.aset("name", self.name())?;
        hash.aset("groups", self.groups())?;
        Ok(hash)
    }

    fn to_str(&self) -> Result<String, magnus::Error> {
        Ok(format!("{self:?}"))
    }
}

pub fn setup(namespace: RModule) -> Result<(), magnus::Error> {
    let user_class = namespace.define_class("User", class::object())?;
    user_class.define_method("id", method!(RbUser::id, 0))?;
    user_class.define_method("group_id", method!(RbUser::group_id, 0))?;
    user_class.define_method("name", method!(RbUser::name, 0))?;
    user_class.define_method("groups", method!(RbUser::groups, 0))?;
    user_class.define_method("to_hash", method!(RbUser::to_hash, 0))?;
    user_class.define_method("_to_str", method!(RbUser::to_str, 0))?;
    Ok(())
}
