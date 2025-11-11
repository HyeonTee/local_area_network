use std::ffi::CString;

pub struct Interface {
	pub name: String,
	pub index: u32,
	pub mac: [u8; libc::ETH_ALEN as usize],
	pub ip: [u8; 4],
}

pub fn get_interface_by_name(iface_name: &str) -> Interface {
	// ioctl (Input Output Controller)

	// C -> ['e', 't', 'h', '0', '\0']
	// Rust -> { ptr: ['e', 't', 'h', '0'], len: 4 }
	let iface_name_c = CString::new(iface_name).unwrap();
	let mut ifr: libc::ifreq = unsafe { std::mem::zeroed() };

	unsafe {
		std::ptr::copy_nonoverlapping(iface_name_c.as_ptr(), ifr.ifr_name.as_mut_ptr(), libc::IFNAMSIZ - 1);

		let sock_fd = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
		if sock_fd < 0 {
			panic!("ifreq socket failed");
		}

		if libc::ioctl(sock_fd, libc::SIOCGIFINDEX, &mut ifr) < 0 {
			panic!("ioctl to get interface index failed");
		}
		let if_index = ifr.ifr_ifru.ifru_ifindex as u32;

		if libc::ioctl(sock_fd, libc::SIOCGIFHWADDR, &mut ifr) < 0 {
			panic!("ioctl to get interface mac address failed");
		}
		let mut mac_addr: [u8; libc::ETH_ALEN as usize] = std::mem::zeroed();
		std::ptr::copy_nonoverlapping(ifr.ifr_ifru.ifru_hwaddr.sa_data.as_ptr(), mac_addr.as_mut_ptr(), mac_addr.len());

		if libc::ioctl(sock_fd, libc::SIOCGIFADDR, &mut ifr) < 0 {
			panic!("ioctl to get interface ip address failed");
		}
		let mut ip_addr: [u8; 4] = std::mem::zeroed();
		std::ptr::copy_nonoverlapping(ifr.ifr_ifru.ifru_addr.sa_data.as_ptr().add(2), ip_addr.as_mut_ptr(), ip_addr.len());

		Interface {
			name: iface_name.to_string(),
			index: if_index,
			mac: mac_addr,
			ip: ip_addr,
		}
	}
}

#[cfg(test)]
mod tests {
    use crate::nic::interface::get_interface_by_name;

	#[test]
	fn test_get_interface_by_name() {
		let iface = get_interface_by_name("eth0");

		assert!(iface.index > 0);
		assert_eq!(iface.name, "eth0");
	}
}