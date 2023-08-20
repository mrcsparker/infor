use std::cell::RefCell;

use sysinfo::{System, SystemExt};

pub struct RbSysinfo {
    sys: System,
}

#[magnus::wrap(class = "Sysinfo")]
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
    // pub fn cpus(&mut self) -> Vec<PyCpu> {

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
}
