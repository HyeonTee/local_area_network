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
}