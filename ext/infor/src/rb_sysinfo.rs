use magnus::{class, function, method, Module, Object, RModule};
use magnus::{exception, Error, RArray};
use rayon::prelude::*;
use std::cell::RefCell;

use sysinfo::{
    ComponentExt, CpuExt, DiskExt, LoadAvg, NetworkExt, NetworksExt, System, SystemExt, UserExt,
};

use crate::rb_component::RbComponent;
use crate::rb_cpu::RbCpu;
use crate::rb_disk::RbDisk;
use crate::rb_load_avg::RbLoadAvg;
use crate::rb_network::RbNetwork;
use crate::rb_user::RbUser;

#[magnus::wrap(class = "Infor::Sysinfo", free_immediately, size)]
pub struct RbSysinfo(RefCell<System>);

impl RbSysinfo {
    fn new() -> Self {
        Self(RefCell::new(System::new_all()))
    }

    /// Refreshes all system, processes, disks and network interfaces information.
    fn refresh_all(&self) {
        self.0.borrow_mut().refresh_all()
    }

    /// Refreshes system information (RAM, swap, CPU usage and components' temperature).
    fn refresh_system(&self) {
        self.0.borrow_mut().refresh_system()
    }

    /// Refreshes RAM and SWAP usage.
    fn refresh_memory(&self) {
        self.0.borrow_mut().refresh_memory()
    }

    /// Refreshes CPUs information.
    fn refresh_cpu(&self) {
        self.0.borrow_mut().refresh_cpu()
    }

    /// Refreshes components' temperature.
    fn refresh_components(&self) {
        self.0.borrow_mut().refresh_components()
    }

    /// Refreshes components list.
    fn refresh_components_list(&self) {
        self.0.borrow_mut().refresh_components_list()
    }

    /// Gets all processes and updates their information.
    fn refresh_processes(&self) {
        self.0.borrow_mut().refresh_processes()
    }

    /// Refreshes the listed disks' information.
    fn refresh_disks(&self) {
        self.0.borrow_mut().refresh_disks()
    }

    /// The disk list will be emptied then completely recomputed.
    fn refresh_disks_list(&self) {
        self.0.borrow_mut().refresh_disks_list()
    }

    /// Refreshes users list.
    fn refresh_users_list(&self) {
        self.0.borrow_mut().refresh_users_list()
    }

    /// Refreshes networks data.
    fn refresh_networks(&self) {
        self.0.borrow_mut().refresh_networks()
    }

    /// The network list will be updated: removing not existing anymore interfaces and adding new
    /// ones.
    fn refresh_networks_list(&self) {
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
    fn cpus(&self) -> Result<RArray, Error> {
        let array = RArray::new();

        self.0.borrow().cpus().iter().for_each(|cpu| {
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

    /// Returns the number of physical cores on the CPU or `None` if it couldn't get it.
    fn physical_core_count(&self) -> Option<usize> {
        self.0.borrow().physical_core_count()
    }

    /// Returns the RAM size in bytes.
    fn total_memory(&self) -> u64 {
        self.0.borrow().total_memory()
    }

    /// Returns the amount of free RAM in bytes.
    fn free_memory(&self) -> u64 {
        self.0.borrow().free_memory()
    }

    /// Returns the amount of available RAM in bytes.
    fn available_memory(&self) -> u64 {
        self.0.borrow().available_memory()
    }

    /// Returns the amount of used RAM in bytes.
    fn used_memory(&self) -> u64 {
        self.0.borrow().used_memory()
    }

    /// Returns the SWAP size in bytes.
    fn total_swap(&self) -> u64 {
        self.0.borrow().total_swap()
    }

    /// Returns the amount of free SWAP in bytes.
    fn free_swap(&self) -> u64 {
        self.0.borrow().free_swap()
    }

    /// Returns the amount of used SWAP in bytes.
    fn used_swap(&self) -> u64 {
        self.0.borrow().free_swap()
    }

    /// Returns the components list.
    fn components(&self) -> Result<RArray, Error> {
        let array = RArray::new();

        self.0.borrow().components().iter().for_each(|component| {
            let rb_component = RbComponent::new(
                component.temperature(),
                component.max(),
                component.critical(),
                component.label().to_string(),
            );
            match array.push(rb_component) {
                Ok(_) => {}
                Err(e) => Err(Error::new(exception::runtime_error(), format!("{e:?}"))).unwrap(),
            }
        });
        Ok(array)
    }

    // Returns a mutable components list.
    // fn components_mut(&mut self) -> &mut [Component];

    /// Returns the users list.
    fn users(&self) -> Result<RArray, Error> {
        let array = RArray::new();

        self.0.borrow().users().iter().for_each(|u| {
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
    fn disks(&self) -> Result<RArray, Error> {
        let array = RArray::new();

        self.0.borrow().disks().iter().for_each(|disk| {
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

    // Returns the disks list.
    // fn disks_mut(&mut self) -> &mut Disks;

    // Sort the disk list with the provided callback.
    // fn sort_disks_by<F>(&mut self, compare: F)

    /// Returns the network interfaces object.
    fn networks(&self) -> Result<RArray, Error> {
        let array = RArray::new();

        self.0
            .borrow()
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

    // Returns a mutable access to network interfaces.
    // fn networks_mut(&mut self) -> &mut Networks;

    /// Returns system uptime (in seconds).
    fn uptime(&self) -> u64 {
        self.0.borrow().uptime()
    }

    /// Returns the time (in seconds) when the system booted since UNIX epoch.
    fn boot_time(&self) -> u64 {
        self.0.borrow().boot_time()
    }

    /// Returns the system load average value.
    fn load_average(&self) -> RbLoadAvg {
        let load_avg = self.0.borrow().load_average();
        RbLoadAvg::new(load_avg.one, load_avg.five, load_avg.fifteen)
    }

    /// Returns the system name.
    fn name(&self) -> Option<String> {
        self.0.borrow().name()
    }

    /// Returns the system's kernel version.
    fn kernel_version(&self) -> Option<String> {
        self.0.borrow().kernel_version()
    }

    /// Returns the system version (e.g. for MacOS this will return 11.1 rather than the kernel version).
    fn os_version(&self) -> Option<String> {
        self.0.borrow().os_version()
    }

    /// Returns the system long os version (e.g "MacOS 11.2 BigSur").
    fn long_os_version(&self) -> Option<String> {
        self.0.borrow().long_os_version()
    }

    /// Returns the distribution id as defined by os-release,
    fn distribution_id(&self) -> String {
        self.0.borrow().distribution_id()
    }

    /// Returns the system hostname based off DNS
    fn host_name(&self) -> Option<String> {
        self.0.borrow().host_name()
    }
}

pub fn setup(namespace: RModule) -> Result<(), magnus::Error> {
    let sysinfo_class = namespace.define_class("Sysinfo", class::object())?;

    sysinfo_class.define_singleton_method("new", function!(RbSysinfo::new, 0))?;

    sysinfo_class.define_method("refresh_all", method!(RbSysinfo::refresh_all, 0))?;
    sysinfo_class.define_method("refresh_system", method!(RbSysinfo::refresh_system, 0))?;
    sysinfo_class.define_method("refresh_memory", method!(RbSysinfo::refresh_memory, 0))?;
    sysinfo_class.define_method("refresh_cpu", method!(RbSysinfo::refresh_cpu, 0))?;
    sysinfo_class.define_method(
        "refresh_components",
        method!(RbSysinfo::refresh_components, 0),
    )?;
    sysinfo_class.define_method(
        "refresh_components_list",
        method!(RbSysinfo::refresh_components_list, 0),
    )?;
    sysinfo_class.define_method(
        "refresh_processes",
        method!(RbSysinfo::refresh_processes, 0),
    )?;
    sysinfo_class.define_method("refresh_disks", method!(RbSysinfo::refresh_disks, 0))?;
    sysinfo_class.define_method(
        "refresh_disks_list",
        method!(RbSysinfo::refresh_disks_list, 0),
    )?;
    sysinfo_class.define_method(
        "refresh_users_list",
        method!(RbSysinfo::refresh_users_list, 0),
    )?;
    sysinfo_class.define_method("refresh_networks", method!(RbSysinfo::refresh_networks, 0))?;
    sysinfo_class.define_method(
        "refresh_networks_list",
        method!(RbSysinfo::refresh_networks_list, 0),
    )?;

    sysinfo_class.define_method("cpus", method!(RbSysinfo::cpus, 0))?;

    sysinfo_class.define_method(
        "physical_core_count",
        method!(RbSysinfo::physical_core_count, 0),
    )?;
    sysinfo_class.define_method("total_memory", method!(RbSysinfo::total_memory, 0))?;
    sysinfo_class.define_method("free_memory", method!(RbSysinfo::free_memory, 0))?;
    sysinfo_class.define_method("available_memory", method!(RbSysinfo::available_memory, 0))?;
    sysinfo_class.define_method("used_memory", method!(RbSysinfo::used_memory, 0))?;
    sysinfo_class.define_method("total_swap", method!(RbSysinfo::total_swap, 0))?;
    sysinfo_class.define_method("free_swap", method!(RbSysinfo::free_swap, 0))?;
    sysinfo_class.define_method("used_swap", method!(RbSysinfo::used_swap, 0))?;

    sysinfo_class.define_method("components", method!(RbSysinfo::components, 0))?;
    sysinfo_class.define_method("users", method!(RbSysinfo::users, 0))?;
    sysinfo_class.define_method("disks", method!(RbSysinfo::disks, 0))?;
    sysinfo_class.define_method("networks", method!(RbSysinfo::networks, 0))?;

    sysinfo_class.define_method("uptime", method!(RbSysinfo::uptime, 0))?;
    sysinfo_class.define_method("boot_time", method!(RbSysinfo::boot_time, 0))?;
    sysinfo_class.define_method("load_average", method!(RbSysinfo::load_average, 0))?;
    sysinfo_class.define_method("name", method!(RbSysinfo::name, 0))?;
    sysinfo_class.define_method("kernel_version", method!(RbSysinfo::kernel_version, 0))?;
    sysinfo_class.define_method("os_version", method!(RbSysinfo::os_version, 0))?;
    sysinfo_class.define_method("long_os_version", method!(RbSysinfo::long_os_version, 0))?;
    sysinfo_class.define_method("distribution_id", method!(RbSysinfo::distribution_id, 0))?;
    sysinfo_class.define_method("host_name", method!(RbSysinfo::host_name, 0))?;

    Ok(())
}
