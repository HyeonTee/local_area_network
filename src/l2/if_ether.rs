use crate::util::{fmt_eth_proto, fmt_mac};

// repr(C): ensures the same field layout as a C struct
// packed: disables automatic padding between fields (no 2-byte alignment)
#[repr(C, packed)]
pub struct EthHdr {
    pub h_dest: [u8; libc::ETH_ALEN as usize],
    pub h_source: [u8; libc::ETH_ALEN as usize],
    pub h_proto: u16,
}

impl EthHdr {
    pub fn new(buf: &mut [u8]) -> &mut Self {
        unsafe { &mut *(buf.as_mut_ptr() as *mut Self) }
    }

    pub fn print_ethhdr(&self) {
        println!("--------------------- Ethernet Header ---------------------");
        println!("h_dest: {}", fmt_mac(self.h_dest));
        println!("h_source: {}", fmt_mac(self.h_source));
        println!("h_proto: {}", fmt_eth_proto(self.h_proto));
        println!("-----------------------------------------------------------");
        println!();
    }
}
