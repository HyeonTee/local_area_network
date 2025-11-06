#[repr(C, packed)]
pub struct EthHdr {
	pub h_dest: [u8; libc::ETH_ALEN as usize],
	pub h_source: [u8; libc::ETH_ALEN as usize],
	pub h_proto: u16
}

impl EthHdr {
	pub fn new(buf: &mut [u8]) -> &mut Self {
		unsafe { &mut *(buf.as_mut_ptr() as *mut Self) }
	}
}