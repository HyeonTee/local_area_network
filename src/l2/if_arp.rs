use crate::util::{fmt_arp_op, fmt_eth_proto, fmt_hrd_type, fmt_ip, fmt_mac};

#[repr(C, packed)]
pub struct ArpHdr {
    pub ar_hrd: u16,
    pub ar_pro: u16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: u16,

    pub ar_sha: [u8; libc::ETH_ALEN as usize],
    pub ar_sip: [u8; 4],
    pub ar_tha: [u8; libc::ETH_ALEN as usize],
    pub ar_tip: [u8; 4],
}

impl ArpHdr {
    pub fn new(buf: &mut [u8]) -> &mut Self {
        unsafe { &mut *(buf.as_mut_ptr() as *mut Self) }
    }

    pub fn print_arp(&self) {
        println!("--------------------- Arp Header ---------------------");
        println!("ar_hrd: {}", fmt_hrd_type(self.ar_hrd));
        println!("ar_pro: {}", fmt_eth_proto(self.ar_pro));
        println!("ar_hln: {}", self.ar_hln);
        println!("ar_pln: {}", self.ar_pln);
        println!("ar_op: {}", fmt_arp_op(self.ar_op));
        println!("ar_sha: {}", fmt_mac(self.ar_sha));
        println!("ar_sip: {}", fmt_ip(self.ar_sip));
        println!("ar_tha: {}", fmt_mac(self.ar_tha));
        println!("ar_tip: {}", fmt_ip(self.ar_tip));
        println!("------------------------------------------------------");
        println!();
    }
}
