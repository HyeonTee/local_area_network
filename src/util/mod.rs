use std::time::Duration;

pub fn fmt_mac(mac: [u8; 6]) -> String {
	format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", mac[0], mac[1], mac[2], mac[3], mac[4], mac[5])
}

pub fn fmt_ip(ip: [u8; 4]) -> String {
	format!("{}:{}:{}:{}", ip[0], ip[1], ip[2], ip[3])
}

pub fn fmt_eth_proto(proto: u16) -> String {
	let proto_le = u16::from_be(proto) as i32;
	let proto_str = match proto_le {
		libc::ETH_P_ARP => "ETH_P_ARP",
		libc::ETH_P_IP => "ETH_P_IP",
		_ => "Unknown"
	};

	format!("{proto_str}(0x{proto_le:04x})")
}

pub fn fmt_hrd_type(t: u16) -> String {
	let type_le = u16::from_be(t);
	let type_str = match type_le {
		libc::ARPHRD_ETHER => "ARPHRD_ETHER",
		_ => "Unknown",
	};

	format!("{type_str}(0x{type_le:04x})")
}

pub fn fmt_arp_op(op: u16) -> String {
	let op_le = u16::from_be(op);
	let op_str = match op_le {
		libc::ARPOP_REQUEST => "ARPOP_REQUEST",
		libc::ARPOP_REPLY => "ARPOP_REPLY",
		_ => "Unknown",
	};

	format!("{op_str}(0x{op_le:04x})")
}

pub fn fmt_duration(duration: Duration) -> String {
	let secs = duration.as_secs();
	let millis = duration.subsec_millis();
	let micros = duration.subsec_micros();
	let nanos = duration.subsec_nanos();

	let (int, frac, unit) = if secs > 0 {
		(secs as u32, millis, "sec")
	} else if millis > 0 {
		(millis, micros % 1000, "msec")
	} else if micros > 0 {
		(micros, nanos % 1000, "usec")
	} else {
		(nanos, 0, "nsec")
	};

	format!("{}.{:0<3} {}", int, frac, unit)
}

#[cfg(test)]
mod tests {
	use std::time::Duration;

	use super::fmt_duration;

	#[test]
	fn test_fmt_duration() {
		let duration = Duration::from_secs(1) + Duration::from_millis(123);

		let result = fmt_duration(duration);
		assert_eq!(result, "1.123 sec");

		let duration = Duration::from_millis(1) + Duration::from_micros(123);

		let result = fmt_duration(duration);
		assert_eq!(result, "1.123 msec");

		let duration = Duration::from_micros(1) + Duration::from_nanos(12);

		let result = fmt_duration(duration);
		assert_eq!(result, "1.120 usec");

		let duration = Duration::from_nanos(123);

		let result = fmt_duration(duration);
		assert_eq!(result, "123.000 nsec");
	}
}