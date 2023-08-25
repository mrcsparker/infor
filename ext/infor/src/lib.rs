use magnus::{define_module, Error};

mod rb_component;
mod rb_cpu;
mod rb_disk;
mod rb_load_avg;
mod rb_network;
mod rb_sysinfo;
mod rb_user;

#[magnus::init]
fn init() -> Result<(), Error> {
    let namespace = define_module("Infor")?;
    rb_sysinfo::setup(namespace)?;
    rb_cpu::setup(namespace)?;
    rb_component::setup(namespace)?;
    rb_user::setup(namespace)?;
    rb_disk::setup(namespace)?;
    rb_network::setup(namespace)?;
    rb_load_avg::setup(namespace)?;

    Ok(())
}
