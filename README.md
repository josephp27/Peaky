# Peak | Screen Peak Your Teammates

The goal for this project is to write everything from scratch, using no outside libraries

Uses sockets and UDP network calls to stream video between you and your teammates using minimal libraries, written in pure Rust

Since we should always have the fastest stream, dropping packets is more than ok. Packet ordering happens in O(1) time inserts, for O(n) runtime, where n is the number of packets in a frame

### TODO
- [X] Record screen
- [X] Unreliable Packet Sequencing
    - [X] Number packets
    - [X] Reorder packets on client side
    - [ ] Drop packets part of previous frame
- [X] Stream video
- [X] Limit Frame rate
    - [X] Sender
    - [X] Receiver
- [ ] Codec - H.264
- [ ] Display as small overlay on screen - DirectX?
- [ ] Upgrading connection to P2P via UDP Hole Punching - hard time with NAT


https://io7m.com/documents/udp-reliable/#ordering
