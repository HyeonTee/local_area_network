use std::time::Duration;

use clap::Parser;
use local_area_network::{args::arping_args::ArpingArgs, l2::{if_arp::ArpHdr, if_ether::EthHdr}};

// fn parse_args() -> [u8;4] {
// 	let args = std::env::args().collect::<Vec<String>>();
// 	if args.len() != 2 {
// 		panic!("args.len() should be 2");
// 	}

// 	let dest_ip = args[1].split(".").collect::<Vec<&str>>();
// 	if dest_ip.len() != 4 {
// 		panic!("Invalid dest ip");
// 	}

// 	let dest_ip: [u8;4] = dest_ip.iter().map(|bit| bit.parse::<u8>().unwrap()).collect::<Vec<u8>>().try_into().unwrap();

// 	dest_ip
// }

fn main() {
	// let dest_ip = parse_args();

	let args = ArpingArgs::parse();
	let dest_ip = args.dest_ip.octets();
	let count = args.count;

	// socket
	let sock_fd = unsafe { libc::socket(libc::AF_PACKET, libc::SOCK_RAW, libc::ETH_P_ARP.to_be()) }; /* protocol should be big-endian */
	if sock_fd < 0 {
		panic!("socket failed")
	}

	// eth0 -> index
	let interface = default_net::get_default_interface().unwrap();
	let if_index = interface.index as i32;
	let src_mac = interface.mac_addr.unwrap().octets();
	let src_ip = interface.ipv4[0].addr.octets();

	// socket-interface binding
	let mut sockaddr: libc::sockaddr_ll = unsafe { std::mem::zeroed() }; 	/* sockaddr_ll : link layer */ 
	sockaddr.sll_family = libc::AF_PACKET as u16;
	sockaddr.sll_ifindex = if_index as i32;
	sockaddr.sll_protocol = (libc::ETH_P_ARP as u16).to_be();				/* protocol should be big-endian */

	let bind_result = unsafe {
		libc::bind(
			sock_fd,
			&sockaddr as *const libc::sockaddr_ll as *const libc::sockaddr, /* &sockaddr_ll -> *const sockaddr_ll -> *const sockaddr */
			std::mem::size_of_val(&sockaddr) as libc::socklen_t
		)
	};

	if bind_result < 0 {
		panic!("bind failed");
	}

	let mut index = 0;
	while index < count {
		send_arp_request(sock_fd, dest_ip, src_ip, src_mac);
		recv_arp_reply(sock_fd, src_mac);
		println!("index={index}");

		if index < count - 1 {
			std::thread::sleep(Duration::from_secs(1));	
		}
		index += 1;
	}
	
}

fn send_arp_request(sock_fd: i32, dest_ip: [u8; 4], src_ip: [u8; 4], src_mac: [u8; 6]) {
	// Create L2 Frame
	let mut buf = [0u8; 42];
	let (eth_hdr, arp_hdr) = buf.split_at_mut(libc::ETH_HLEN as usize);

	let eth_hdr = EthHdr::new(eth_hdr);
	let arp_hdr = ArpHdr::new(arp_hdr);

	// mac addr
	eth_hdr.h_dest = [0xff; libc::ETH_ALEN as usize];
	eth_hdr.h_source = src_mac;
	eth_hdr.h_proto = (libc::ETH_P_ARP as u16).to_be();

	// arp header
	arp_hdr.ar_hrd = libc::ARPHRD_ETHER.to_be();
	arp_hdr.ar_pro = (libc::ETH_P_IP as u16).to_be();
	arp_hdr.ar_hln = libc::ETH_ALEN as u8;
	arp_hdr.ar_pln = 4;
	arp_hdr.ar_op = libc::ARPOP_REQUEST.to_be();
	arp_hdr.ar_sha = src_mac;
	arp_hdr.ar_sip = src_ip;
	arp_hdr.ar_tha = [0x00; libc::ETH_ALEN as usize];
	arp_hdr.ar_tip = dest_ip;

	let send_bytes = unsafe { libc::write(sock_fd, buf.as_ptr() as *const libc::c_void, buf.len()) };
	if send_bytes < 0 {
		panic!("send failed");
	}
}

fn recv_arp_reply(sock_fd: i32, mac: [u8; 6]) {
	let mut buf = [0u8; 42];

	loop {
		let recv_bytes = unsafe { libc::read(sock_fd, buf.as_ptr() as *mut libc::c_void, buf.len()) };
		if recv_bytes < 0 {
			panic!("recv failed");
		}

		let (eth_hdr, arp_hdr) = buf.split_at_mut(libc::ETH_HLEN as usize);
		let eth_hdr = EthHdr::new(eth_hdr);
		let arp_hdr = ArpHdr::new(arp_hdr);
		
		if eth_hdr.h_dest != mac {
			continue;
		}

		if arp_hdr.ar_op != libc::ARPOP_REPLY.to_be() {
			continue;
		}

		eth_hdr.print_ethhdr();
		arp_hdr.print_arp();

		break;
	}
}