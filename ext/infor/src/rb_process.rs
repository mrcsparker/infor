use sysinfo::Pid;

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Infor::Process", free_immediately, size)]
pub struct RbProcess {
    pub pid: u32,
    pub name: String,
    pub cmd: Vec<String>,
    pub exe: String,
    pub environ: Vec<String>,
    pub cwd: String,
    pub root: String,
    pub memory: u64,
    pub virtual_memory: u64,
    pub parent: Option<Pid>,
}
