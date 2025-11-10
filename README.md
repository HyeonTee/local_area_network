# Linux Network Stack (TCP/IP)

L7 Application Layer — Data  
L4 Transport Layer — Segment  
L3 Network Layer — Packet  
L2 Data Link Layer — Frame (Ethernet Protocol)  
L1 Physical Layer

## L2 Frame Structure
- 14 bytes: Ethernet header  
- 1500 bytes: Ethernet payload (MTU) → IP header + IP payload (L3 packet)  
- 4 bytes: Ethernet trailer (FCS)

### ethernet header of linux ([if_ether.h](https://github.com/torvalds/linux/blob/0512e0134582ef85dee77d51aae77dcd1edec495/include/uapi/linux/if_ether.h))

```c
struct ethhdr {
	unsigned char	h_dest[ETH_ALEN];	/* destination eth addr	*/
	unsigned char	h_source[ETH_ALEN];	/* source ether addr	*/
	__be16			h_proto;			/* packet type ID field	*/
} __attribute__((packed));


#define ETH_ALEN	6		/* Octets in one ethernet addr	 */
#define ETH_HLEN	14		/* Total octets in header.	 */

#define ETH_P_ARP	0x0806		/* Address Resolution packet	*/
#define ETH_P_IP	0x0800		/* Internet Protocol packet	*/
```

### arp header of linux ([if_arp.h](https://github.com/torvalds/linux/blob/0512e0134582ef85dee77d51aae77dcd1edec495/include/uapi/linux/if_arp.h))
```c
struct arphdr {
	__be16			ar_hrd;			/* format of hardware address	*/
	__be16			ar_pro;			/* format of protocol address	*/
	unsigned char	ar_hln;			/* length of hardware(mac) address	*/
	unsigned char	ar_pln;			/* length of protocol address	*/
	__be16			ar_op;			/* ARP opcode (command)		*/

#if 0
	 /*
	  *	 Ethernet looks like this : This bit is variable sized however...
	  */
	unsigned char		ar_sha[ETH_ALEN];	/* sender hardware address	*/
	unsigned char		ar_sip[4];			/* sender IP address		*/
	unsigned char		ar_tha[ETH_ALEN];	/* target hardware address	*/
	unsigned char		ar_tip[4];			/* target IP address		*/
#endif

};

#define ARPHRD_ETHER 	1		/* Ethernet 10Mbps		*/
#define	ARPOP_REQUEST	1		/* ARP request			*/
#define	ARPOP_REPLY		2		/* ARP reply			*/
```

## arping
- Send ARP(Address Resolution Protocol) request for discovering L2 address (MAC address) associated with an L3 address (IP address)
```bash
>> tcpdump -vvv -xx -n arp
>> arping 192.168.0.1 -c 1
02:24:20.644457 ARP, Ethernet (len 6), IPv4 (len 4), Request who-has 192.168.0.1 tell 192.168.0.100, length 44
        0x0000:  ffff ffff ffff 0242 ac11 0000 0806 0001
        0x0010:  0800 0604 0001 0242 ac11 0000 c0a8 0064
        0x0020:  0000 0000 0000 c0a8 0001 0000 0000 0000
        0x0030:  0000 0000 0000 0000 0000
02:24:20.644463 ARP, Ethernet (len 6), IPv4 (len 4), Reply 192.168.0.1 is-at 02:42:d6:26:49:6d, length 28
        0x0000:  0242 ac11 0000 0242 d626 496d 0806 0001
        0x0010:  0800 0604 0002 0242 d626 496d c0a8 0001
        0x0020:  0242 ac11 0000 c0a8 0064
```

```bash
>> tcpdump -vvv -xx -n arp
>> cargo run --bin arping 192.168.0.1
02:39:31.687697 ARP, Ethernet (len 6), IPv4 (len 4), Request who-has 192.168.0.1 tell 192.168.0.100, length 28
        0x0000:  ffff ffff ffff 0242 ac11 0000 0806 0001
        0x0010:  0800 0604 0001 0242 ac11 0000 c0a8 0064
        0x0020:  0000 0000 0000 c0a8 0001
02:39:31.687698 ARP, Ethernet (len 6), IPv4 (len 4), Reply 192.168.0.1 is-at 02:42:d6:26:49:6d, length 28
        0x0000:  0242 ac11 0000 0242 d626 496d 0806 0001
        0x0010:  0800 0604 0002 0242 d626 496d c0a8 0001
        0x0020:  0242 ac11 0000 c0a8 0064
```

# Environment
- devcontainer.json: Defines a containerized development environment and language servers for VS Code.  
- Docker: Provides a virtual local area network environment based on Ubuntu 22.04 for network stack testing.
