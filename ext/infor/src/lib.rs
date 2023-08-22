use magnus::{class, define_module, function, method, prelude::*, Error};

mod rb_disk;
mod rb_sysinfo;

use rb_disk::RbDisk;
use rb_sysinfo::RbSysinfo;

#[magnus::init]
fn init() -> Result<(), Error> {
    let namespace = define_module("Infor")?;

    // Sysinfo class
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

    sysinfo_class.define_method("total_memory", method!(RbSysinfo::total_memory, 0))?;
    sysinfo_class.define_method("free_memory", method!(RbSysinfo::free_memory, 0))?;
    sysinfo_class.define_method("available_memory", method!(RbSysinfo::available_memory, 0))?;
    sysinfo_class.define_method("used_memory", method!(RbSysinfo::used_memory, 0))?;
    sysinfo_class.define_method("total_swap", method!(RbSysinfo::total_swap, 0))?;
    sysinfo_class.define_method("free_swap", method!(RbSysinfo::free_swap, 0))?;
    sysinfo_class.define_method("used_swap", method!(RbSysinfo::used_swap, 0))?;

    sysinfo_class.define_method("disks", method!(RbSysinfo::disks, 0))?;

    sysinfo_class.define_method("uptime", method!(RbSysinfo::uptime, 0))?;
    sysinfo_class.define_method("boot_time", method!(RbSysinfo::boot_time, 0))?;
    sysinfo_class.define_method("name", method!(RbSysinfo::name, 0))?;
    sysinfo_class.define_method("kernel_version", method!(RbSysinfo::kernel_version, 0))?;
    sysinfo_class.define_method("os_version", method!(RbSysinfo::os_version, 0))?;
    sysinfo_class.define_method("long_os_version", method!(RbSysinfo::long_os_version, 0))?;
    sysinfo_class.define_method("distribution_id", method!(RbSysinfo::distribution_id, 0))?;
    sysinfo_class.define_method("host_name", method!(RbSysinfo::host_name, 0))?;

    // Disk class
    let disk_class = namespace.define_class("Disk", class::object())?;
    disk_class.define_method("to_string", method!(RbDisk::to_string, 0))?;

    Ok(())
}
