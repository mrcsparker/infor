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

    Ok(())
}
