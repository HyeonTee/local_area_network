use std::time::{Duration, Instant};

use clap::Parser;
use local_area_network::{
    args::arping_args::ArpingArgs,
    l2::{if_arp::ArpHdr, if_ether::EthHdr},
    nic::interface::get_interface_by_name,
    util::{fmt_duration, fmt_ip, fmt_mac},
};

fn main() {
    let args = ArpingArgs::parse();
    let dest_ip = args.dest_ip.octets();
    let count = args.count;
    let iface_name = args.iface_name;

    // socket
    let sock_fd = unsafe { libc::socket(libc::AF_PACKET, libc::SOCK_RAW, libc::ETH_P_ARP.to_be()) }; /* protocol should be big-endian */
    if sock_fd < 0 {
        panic!("socket failed")
    }

    // eth0 -> index
    let iface = get_interface_by_name(&iface_name);
    let if_index = iface.index;
    let src_mac = iface.mac;
    let src_ip = iface.ip;

    // socket-interface binding
    let mut sockaddr: libc::sockaddr_ll = unsafe { std::mem::zeroed() }; /* sockaddr_ll : link layer */
    sockaddr.sll_family = libc::AF_PACKET as u16;
    sockaddr.sll_ifindex = if_index as i32;
    sockaddr.sll_protocol = (libc::ETH_P_ARP as u16).to_be();

    let bind_result = unsafe {
        libc::bind(
            sock_fd,
            &sockaddr as *const libc::sockaddr_ll as *const libc::sockaddr, /* &sockaddr_ll -> *const sockaddr_ll -> *const sockaddr */
            std::mem::size_of_val(&sockaddr) as libc::socklen_t,
        )
    };

    if bind_result < 0 {
        panic!("bind failed");
    }

    set_nonblocking_socket(sock_fd);

    let mut index = 0;
    println!("ARPING {}", fmt_ip(dest_ip));
    while index < count {
        let start_time = Instant::now(); // monotonic time
        send_arp_request(sock_fd, dest_ip, src_ip, src_mac);

        while start_time.elapsed() < Duration::from_secs(1) {
            match recv_arp_reply(sock_fd, src_mac, index, start_time) {
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    println!("Timeout index: {index}");
                    std::thread::sleep(Duration::from_millis(200));
                }
            }
        }

        if index < count - 1 {
            if start_time.elapsed() < Duration::from_secs(1) {
                std::thread::sleep(Duration::from_secs(1) - start_time.elapsed());
            }
        }
        index += 1;
    }
}

fn set_sock_timeout(sock_fd: i32) {
    let mut recv_timeval: libc::timeval = unsafe { std::mem::zeroed() };
    recv_timeval.tv_sec = 0;
    recv_timeval.tv_usec = 999_999;

    let result = unsafe {
        libc::setsockopt(
            sock_fd,
            libc::SOL_SOCKET,
            libc::SO_RCVTIMEO,
            &recv_timeval as *const _ as *const libc::c_void,
            std::mem::size_of_val(&recv_timeval) as libc::socklen_t,
        )
    };

    if result < 0 {
        panic!("set socket timeout option failed");
    }
}

fn set_nonblocking_socket(sock_fd: i32) {
    let flags = unsafe { libc::fcntl(sock_fd, libc::F_GETFL, 0) };
    if flags < 0 {
        panic!("fcntl F_GETFL failed");
    }

    let result = unsafe { libc::fcntl(sock_fd, libc::F_SETFL, flags | libc::O_NONBLOCK) };
    if result < 0 {
        panic!("fcntl F_SETFL failed");
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

    let send_bytes =
        unsafe { libc::write(sock_fd, buf.as_ptr() as *const libc::c_void, buf.len()) };
    if send_bytes < 0 {
        panic!("send failed");
    }
}

fn recv_arp_reply(sock_fd: i32, mac: [u8; 6], index: u16, start_time: Instant) -> Result<(), i32> {
    let mut buf = [0u8; 42];

    loop {
        let recv_bytes =
            unsafe { libc::read(sock_fd, buf.as_ptr() as *mut libc::c_void, buf.len()) };

        if recv_bytes < 0 {
            let errno = unsafe { *libc::__errno_location() };
            if errno == libc::EWOULDBLOCK {
                return Err(errno);
            } else {
                panic!("recv failed");
            }
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

        let elapsed = start_time.elapsed();
        println!(
            "{} bytes from {} ({}): index={} time={}",
            recv_bytes,
            fmt_mac(arp_hdr.ar_sha),
            fmt_ip(arp_hdr.ar_sip),
            index,
            fmt_duration(elapsed)
        );

        return Ok(());
    }
}
