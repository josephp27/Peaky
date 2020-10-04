# Peaky
Screen peak your teammates

Uses sockets and UDP network calls to stream video between you and your teammates using minimal libraries, written in pure Rust

Since we should always have the fastest stream, dropping packets is more than ok. Packet ordering happens in O(1) time inserts, for O(n) runtime, where n is the number of packets in a frame

### TODO
- [X] Record screen
- [X] Number packets
- [X] Reorder packets on client side
- [X] Stream video
- [ ] Codec - H.264
- [ ] Display as small overlay on screen - DirectX?
- [ ] Upgrading connection to P2P via UDP Hole Punching - hard time with NAT


https://io7m.com/documents/udp-reliable/#ordering
