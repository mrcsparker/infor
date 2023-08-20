use std::cell::RefCell;

use sysinfo::{System, SystemExt};

pub struct RbSysinfo {
    sys: System,
}

#[magnus::wrap(class = "Infor::Sysinfo")]
pub struct MutRbSysinfo(RefCell<RbSysinfo>);

impl MutRbSysinfo {
    pub fn new() -> Self {
        Self(RefCell::new(RbSysinfo {
            sys: System::new_all(),
        }))
    }

    /// Refreshes all system, processes, disks and network interfaces information.
    pub fn refresh_all(&self) {
        self.0.borrow_mut().sys.refresh_all()
    }

    /// Refreshes system information (RAM, swap, CPU usage and components' temperature).
    pub fn refresh_system(&self) {
        self.0.borrow_mut().sys.refresh_system()
    }

    /// Refreshes RAM and SWAP usage.
    pub fn refresh_memory(&self) {
        self.0.borrow_mut().sys.refresh_memory()
    }

    /// Refreshes CPUs information.
    pub fn refresh_cpu(&self) {
        self.0.borrow_mut().sys.refresh_cpu()
    }

    /// Refreshes components' temperature.
    pub fn refresh_components(&self) {
        self.0.borrow_mut().sys.refresh_components()
    }

    /// Refreshes components list.
    pub fn refresh_components_list(&self) {
        self.0.borrow_mut().sys.refresh_components_list()
    }

    /// Gets all processes and updates their information.
    pub fn refresh_processes(&self) {
        self.0.borrow_mut().sys.refresh_processes()
    }

    /// Refreshes the listed disks' information.
    pub fn refresh_disks(&self) {
        self.0.borrow_mut().sys.refresh_disks()
    }

    /// The disk list will be emptied then completely recomputed.
    pub fn refresh_disks_list(&self) {
        self.0.borrow_mut().sys.refresh_disks_list()
    }

    /// Refreshes users list.
    pub fn refresh_users_list(&self) {
        self.0.borrow_mut().sys.refresh_users_list()
    }

    /// Refreshes networks data.
    pub fn refresh_networks(&self) {
        self.0.borrow_mut().sys.refresh_networks()
    }

    /// The network list will be updated: removing not existing anymore interfaces and adding new
    /// ones.
    pub fn refresh_networks_list(&self) {
        self.0.borrow_mut().sys.refresh_networks_list()
    }

    // Returns the process list.
    // fn processes(&self) -> &HashMap<Pid, Process>;

    // Returns the process corresponding to the given `pid` or `None` if no such process exists.
    // fn process(&self, pid: Pid) -> Option<&Process>;

    // Returns an iterator of process containing the given `name`.
    // fn processes_by_name<'a: 'b, 'b>(

    // Returns an iterator of processes with exactly the given `name`.
    // fn processes_by_exact_name<'a: 'b, 'b>(

    // Returns "global" CPUs information (aka the addition of all the CPUs).
    // fn global_cpu_info(&self) -> &Cpu;

    // Returns the list of the CPUs.
    // pub fn cpus(&self) -> Vec<PyCpu> {

    // Returns the number of physical cores on the CPU or `None` if it couldn't get it.
    // fn physical_core_count(&self) -> Option<usize>;

    /// Returns the RAM size in bytes.
    pub fn total_memory(&self) -> u64 {
        self.0.borrow_mut().sys.total_memory()
    }

    /// Returns the amount of free RAM in bytes.
    pub fn free_memory(&self) -> u64 {
        self.0.borrow_mut().sys.free_memory()
    }

    /// Returns the amount of available RAM in bytes.
    pub fn available_memory(&self) -> u64 {
        self.0.borrow_mut().sys.available_memory()
    }

    /// Returns the amount of used RAM in bytes.
    pub fn used_memory(&self) -> u64 {
        self.0.borrow_mut().sys.used_memory()
    }

    /// Returns the SWAP size in bytes.
    pub fn total_swap(&self) -> u64 {
        self.0.borrow_mut().sys.total_swap()
    }

    /// Returns the amount of free SWAP in bytes.
    pub fn free_swap(&self) -> u64 {
        self.0.borrow_mut().sys.free_swap()
    }

    /// Returns the amount of used SWAP in bytes.
    pub fn used_swap(&self) -> u64 {
        self.0.borrow_mut().sys.free_swap()
    }

    /// Returns system uptime (in seconds).
    pub fn uptime(&self) -> u64 {
        self.0.borrow_mut().sys.uptime()
    }

    /// Returns the time (in seconds) when the system booted since UNIX epoch.
    pub fn boot_time(&self) -> u64 {
        self.0.borrow_mut().sys.boot_time()
    }

    // Returns the system load average value.
    // fn load_average(&self) -> LoadAvg;

    /// Returns the system name.
    pub fn name(&self) -> Option<String> {
        self.0.borrow_mut().sys.name()
    }

    /// Returns the system's kernel version.
    pub fn kernel_version(&self) -> Option<String> {
        self.0.borrow_mut().sys.kernel_version()
    }

    /// Returns the system version (e.g. for MacOS this will return 11.1 rather than the kernel version).
    pub fn os_version(&self) -> Option<String> {
        self.0.borrow_mut().sys.os_version()
    }

    /// Returns the system long os version (e.g "MacOS 11.2 BigSur").

    pub fn long_os_version(&self) -> Option<String> {
        self.0.borrow_mut().sys.long_os_version()
    }

    /// Returns the distribution id as defined by os-release,
    pub fn distribution_id(&self) -> String {
        self.0.borrow_mut().sys.distribution_id()
    }

    /// Returns the system hostname based off DNS
    pub fn host_name(&self) -> Option<String> {
        self.0.borrow_mut().sys.host_name()
    }

    fn __repr__(&self) -> String {
        format!(
            "Sysinfo(total_memory={}, free_memory={}, available_memory={}, used_memory={}, total_swap={}, free_swap={}, used_swap={})",
            self.0.borrow_mut().sys.total_memory(),
            self.0.borrow_mut().sys.free_memory(),
            self.0.borrow_mut().sys.available_memory(),
            self.0.borrow_mut().sys.used_memory(),
            self.0.borrow_mut().sys.total_swap(),
            self.0.borrow_mut().sys.free_swap(),
            self.0.borrow_mut().sys.used_swap()
        )
    }
}
