use magnus::{class, define_module, function, method, prelude::*, Error};

mod rb_sysinfo;

use rb_sysinfo::MutRbSysinfo;

#[magnus::init]
fn init() -> Result<(), Error> {
    let namespace = define_module("Infor")?;
    let sysinfo_class = namespace.define_class("Sysinfo", class::object())?;
    sysinfo_class.define_singleton_method("new", function!(MutRbSysinfo::new, 0))?;
    sysinfo_class.define_method("refresh_all", method!(MutRbSysinfo::refresh_all, 0))?;
    sysinfo_class.define_method("refresh_memory", method!(MutRbSysinfo::refresh_memory, 0))?;
    sysinfo_class.define_method("refresh_cpu", method!(MutRbSysinfo::refresh_cpu, 0))?;
    sysinfo_class.define_method("total_memory", method!(MutRbSysinfo::total_memory, 0))?;
    sysinfo_class.define_method("free_memory", method!(MutRbSysinfo::free_memory, 0))?;
    sysinfo_class.define_method(
        "available_memory",
        method!(MutRbSysinfo::available_memory, 0),
    )?;
    sysinfo_class.define_method("used_memory", method!(MutRbSysinfo::used_memory, 0))?;
    sysinfo_class.define_method("total_swap", method!(MutRbSysinfo::total_swap, 0))?;
    sysinfo_class.define_method("free_swap", method!(MutRbSysinfo::free_swap, 0))?;
    sysinfo_class.define_method("used_swap", method!(MutRbSysinfo::used_swap, 0))?;

    sysinfo_class.define_method("uptime", method!(MutRbSysinfo::uptime, 0))?;
    sysinfo_class.define_method("boot_time", method!(MutRbSysinfo::boot_time, 0))?;
    sysinfo_class.define_method("name", method!(MutRbSysinfo::name, 0))?;
    sysinfo_class.define_method("kernel_version", method!(MutRbSysinfo::kernel_version, 0))?;
    sysinfo_class.define_method("os_version", method!(MutRbSysinfo::os_version, 0))?;
    sysinfo_class.define_method("long_os_version", method!(MutRbSysinfo::long_os_version, 0))?;
    sysinfo_class.define_method("distribution_id", method!(MutRbSysinfo::distribution_id, 0))?;
    sysinfo_class.define_method("host_name", method!(MutRbSysinfo::host_name, 0))?;

    Ok(())
}
