use std::net::Ipv4Addr;

#[derive(clap::Parser)]
pub struct ArpingArgs {
    #[arg(required = true)]
    pub dest_ip: Ipv4Addr,

    #[arg(short = 'c', default_value = "65535")]
    pub count: u16,

    #[arg(short = 'i', default_value = "eth0")]
    pub iface_name: String,
}
