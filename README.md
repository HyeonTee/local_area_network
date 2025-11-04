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

# Environment
- devcontainer.json: Defines a containerized development environment and language servers for VS Code.  
- Docker: Provides a virtual local area network environment based on Ubuntu 22.04 for network stack testing.
