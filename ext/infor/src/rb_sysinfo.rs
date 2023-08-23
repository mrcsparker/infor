use magnus::{exception, Error, RArray};
use rayon::prelude::*;
use std::cell::RefCell;

use sysinfo::{ComponentExt, CpuExt, DiskExt, NetworkExt, NetworksExt, System, SystemExt, UserExt};

use crate::rb_component::RbComponent;
use crate::rb_cpu::RbCpu;
use crate::rb_disk::RbDisk;
use crate::rb_network::RbNetwork;
use crate::rb_user::RbUser;

#[magnus::wrap(class = "Infor::Sysinfo", free_immediately, size)]
pub struct RbSysinfo(RefCell<System>);

impl RbSysinfo {
    pub fn new() -> Self {
        Self(RefCell::new(System::new_all()))
    }

    /// Refreshes all system, processes, disks and network interfaces information.
    pub fn refresh_all(&self) {
        self.0.borrow_mut().refresh_all()
    }

    /// Refreshes system information (RAM, swap, CPU usage and components' temperature).
    pub fn refresh_system(&self) {
        self.0.borrow_mut().refresh_system()
    }

    /// Refreshes RAM and SWAP usage.
    pub fn refresh_memory(&self) {
        self.0.borrow_mut().refresh_memory()
    }

    /// Refreshes CPUs information.
    pub fn refresh_cpu(&self) {
        self.0.borrow_mut().refresh_cpu()
    }

    /// Refreshes components' temperature.
    pub fn refresh_components(&self) {
        self.0.borrow_mut().refresh_components()
    }

    /// Refreshes components list.
    pub fn refresh_components_list(&self) {
        self.0.borrow_mut().refresh_components_list()
    }

    /// Gets all processes and updates their information.
    pub fn refresh_processes(&self) {
        self.0.borrow_mut().refresh_processes()
    }

    /// Refreshes the listed disks' information.
    pub fn refresh_disks(&self) {
        self.0.borrow_mut().refresh_disks()
    }

    /// The disk list will be emptied then completely recomputed.
    pub fn refresh_disks_list(&self) {
        self.0.borrow_mut().refresh_disks_list()
    }

    /// Refreshes users list.
    pub fn refresh_users_list(&self) {
        self.0.borrow_mut().refresh_users_list()
    }

    /// Refreshes networks data.
    pub fn refresh_networks(&self) {
        self.0.borrow_mut().refresh_networks()
    }

    /// The network list will be updated: removing not existing anymore interfaces and adding new
    /// ones.
    pub fn refresh_networks_list(&self) {
        self.0.borrow_mut().refresh_networks_list()
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
    pub fn cpus(&self) -> Result<RArray, Error> {
        let array = RArray::new();

        self.0.borrow_mut().cpus().iter().for_each(|cpu| {
            let rb_cpu = RbCpu::new(
                cpu.cpu_usage(),
                cpu.name().to_string(),
                cpu.vendor_id().to_string(),
                cpu.brand().to_string(),
                cpu.frequency(),
            );
            match array.push(rb_cpu) {
                Ok(_) => {}
                Err(e) => Err(Error::new(exception::runtime_error(), format!("{e:?}"))).unwrap(),
            }
        });
        Ok(array)
    }

    // Returns the number of physical cores on the CPU or `None` if it couldn't get it.
    // fn physical_core_count(&self) -> Option<usize>;

    /// Returns the RAM size in bytes.
    pub fn total_memory(&self) -> u64 {
        self.0.borrow_mut().total_memory()
    }

    /// Returns the amount of free RAM in bytes.
    pub fn free_memory(&self) -> u64 {
        self.0.borrow_mut().free_memory()
    }

    /// Returns the amount of available RAM in bytes.
    pub fn available_memory(&self) -> u64 {
        self.0.borrow_mut().available_memory()
    }

    /// Returns the amount of used RAM in bytes.
    pub fn used_memory(&self) -> u64 {
        self.0.borrow_mut().used_memory()
    }

    /// Returns the SWAP size in bytes.
    pub fn total_swap(&self) -> u64 {
        self.0.borrow_mut().total_swap()
    }

    /// Returns the amount of free SWAP in bytes.
    pub fn free_swap(&self) -> u64 {
        self.0.borrow_mut().free_swap()
    }

    /// Returns the amount of used SWAP in bytes.
    pub fn used_swap(&self) -> u64 {
        self.0.borrow_mut().free_swap()
    }

    /// Returns the components list.
    pub fn components(&self) -> Result<RArray, Error> {
        let array = RArray::new();

        self.0
            .borrow_mut()
            .components()
            .iter()
            .for_each(|component| {
                let rb_component = RbComponent::new(
                    component.temperature(),
                    component.max(),
                    component.critical(),
                    component.label().to_string(),
                );
                match array.push(rb_component) {
                    Ok(_) => {}
                    Err(e) => {
                        Err(Error::new(exception::runtime_error(), format!("{e:?}"))).unwrap()
                    }
                }
            });
        Ok(array)
    }

    /// Returns the users list.
    pub fn users(&self) -> Result<RArray, Error> {
        let array = RArray::new();

        self.0.borrow_mut().users().iter().for_each(|u| {
            let user = RbUser::new(
                u.id().to_string(),
                u.group_id().to_string(),
                u.name().to_string(),
                u.groups().to_vec(),
            );
            match array.push(user) {
                Ok(_) => {}
                Err(e) => Err(Error::new(exception::runtime_error(), format!("{e:?}"))).unwrap(),
            }
        });
        Ok(array)
    }

    /// Returns the disks list.
    pub fn disks(&self) -> Result<RArray, Error> {
        let array = RArray::new();

        self.0.borrow_mut().disks().iter().for_each(|disk| {
            let rb_disk = RbDisk::new(
                disk.name().to_str().unwrap_or("").to_string(),
                disk.mount_point().to_str().unwrap_or("").to_string(),
                disk.total_space(),
                disk.available_space(),
                disk.is_removable(),
            );
            match array.push(rb_disk) {
                Ok(_) => {}
                Err(e) => Err(Error::new(exception::runtime_error(), format!("{e:?}"))).unwrap(),
            }
        });
        Ok(array)
    }

    /// Returns the network interfaces object.
    pub fn networks(&self) -> Result<RArray, Error> {
        let array = RArray::new();

        self.0
            .borrow_mut()
            .networks()
            .iter()
            .for_each(|(interface, network)| {
                let rb_network = RbNetwork::new(
                    interface.to_string(),
                    network.received(),
                    network.total_received(),
                    network.transmitted(),
                    network.total_transmitted(),
                    network.packets_received(),
                    network.total_packets_received(),
                    network.packets_transmitted(),
                    network.total_packets_transmitted(),
                    network.errors_on_received(),
                    network.total_errors_on_received(),
                    network.errors_on_transmitted(),
                    network.total_errors_on_transmitted(),
                );
                match array.push(rb_network) {
                    Ok(_) => {}
                    Err(e) => {
                        Err(Error::new(exception::runtime_error(), format!("{e:?}"))).unwrap()
                    }
                }
            });
        Ok(array)
    }

    /// Returns system uptime (in seconds).
    pub fn uptime(&self) -> u64 {
        self.0.borrow_mut().uptime()
    }

    /// Returns the time (in seconds) when the system booted since UNIX epoch.
    pub fn boot_time(&self) -> u64 {
        self.0.borrow_mut().boot_time()
    }

    // Returns the system load average value.
    // fn load_average(&self) -> LoadAvg;

    /// Returns the system name.
    pub fn name(&self) -> Option<String> {
        self.0.borrow_mut().name()
    }

    /// Returns the system's kernel version.
    pub fn kernel_version(&self) -> Option<String> {
        self.0.borrow_mut().kernel_version()
    }

    /// Returns the system version (e.g. for MacOS this will return 11.1 rather than the kernel version).
    pub fn os_version(&self) -> Option<String> {
        self.0.borrow_mut().os_version()
    }

    /// Returns the system long os version (e.g "MacOS 11.2 BigSur").
    pub fn long_os_version(&self) -> Option<String> {
        self.0.borrow_mut().long_os_version()
    }

    /// Returns the distribution id as defined by os-release,
    pub fn distribution_id(&self) -> String {
        self.0.borrow_mut().distribution_id()
    }

    /// Returns the system hostname based off DNS
    pub fn host_name(&self) -> Option<String> {
        self.0.borrow_mut().host_name()
    }
}
