use magnus::{class, define_module, function, method, prelude::*, Error};

mod rb_component;
mod rb_cpu;
mod rb_disk;
mod rb_sysinfo;
mod rb_user;

use rb_component::RbComponent;
use rb_cpu::RbCpu;
use rb_disk::RbDisk;
use rb_sysinfo::RbSysinfo;
use rb_user::RbUser;

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

    sysinfo_class.define_method("cpus", method!(RbSysinfo::cpus, 0))?;

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

    sysinfo_class.define_method("uptime", method!(RbSysinfo::uptime, 0))?;
    sysinfo_class.define_method("boot_time", method!(RbSysinfo::boot_time, 0))?;
    sysinfo_class.define_method("name", method!(RbSysinfo::name, 0))?;
    sysinfo_class.define_method("kernel_version", method!(RbSysinfo::kernel_version, 0))?;
    sysinfo_class.define_method("os_version", method!(RbSysinfo::os_version, 0))?;
    sysinfo_class.define_method("long_os_version", method!(RbSysinfo::long_os_version, 0))?;
    sysinfo_class.define_method("distribution_id", method!(RbSysinfo::distribution_id, 0))?;
    sysinfo_class.define_method("host_name", method!(RbSysinfo::host_name, 0))?;

    // Cpu class
    let cpu_class = namespace.define_class("Cpu", class::object())?;
    cpu_class.define_method("cpu_usage", method!(RbCpu::cpu_usage, 0))?;
    cpu_class.define_method("name", method!(RbCpu::name, 0))?;
    cpu_class.define_method("vendor_id", method!(RbCpu::vendor_id, 0))?;
    cpu_class.define_method("brand", method!(RbCpu::brand, 0))?;
    cpu_class.define_method("frequency", method!(RbCpu::frequency, 0))?;
    cpu_class.define_method("to_hash", method!(RbCpu::to_hash, 0))?;
    cpu_class.define_method("_to_str", method!(RbCpu::to_str, 0))?;

    // Components class
    let component_class = namespace.define_class("Component", class::object())?;
    component_class.define_method("temperature", method!(RbComponent::temperature, 0))?;
    component_class.define_method("max", method!(RbComponent::max, 0))?;
    component_class.define_method("critical", method!(RbComponent::critical, 0))?;
    component_class.define_method("label", method!(RbComponent::label, 0))?;
    component_class.define_method("to_hash", method!(RbComponent::to_hash, 0))?;
    component_class.define_method("_to_str", method!(RbComponent::to_str, 0))?;

    // Users class
    let user_class = namespace.define_class("User", class::object())?;
    user_class.define_method("id", method!(RbUser::id, 0))?;
    user_class.define_method("group_id", method!(RbUser::group_id, 0))?;
    user_class.define_method("name", method!(RbUser::name, 0))?;
    user_class.define_method("groups", method!(RbUser::groups, 0))?;
    user_class.define_method("to_hash", method!(RbUser::to_hash, 0))?;
    user_class.define_method("_to_str", method!(RbUser::to_str, 0))?;

    // Disk class
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
