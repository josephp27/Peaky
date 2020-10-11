# Peak | Screen Peak Your Teammates [![HitCount](http://hits.dwyl.com/josephp27/Peak.svg)](http://hits.dwyl.com/josephp27/Peak)

Similar to how Discord overlays chat over gameplay, I am setting out to write a tool to stream video over the game window, so teammates can see your screen. Think whenever you screen peaked as a kid to gain advantage over a friend or another team, but over the internet. 

The goal for this project is to write everything from scratch, using no outside libraries. Uses sockets and UDP network calls to stream video between you and your teammates using minimal libraries, written in pure Rust

Since we should always have the fastest stream, dropping packets is more than ok. Packet ordering happens in O(1) time inserts, for O(n) runtime, where n is the number of packets in a frame

### TODO
- [X] Record screen
- [X] [UDP - Unreliable Packet Sequencing](https://io7m.com/documents/udp-reliable/#ordering)
    - [X] Number packets
    - [X] Reorder packets on client side
    - [ ] Drop packets part of previous frame
- [X] Stream video
- [X] Limit Frame rate
    - [X] Sender
    - [X] Receiver
- [ ] Codec - H.264
- [ ] Resize screen capture
- [ ] Display as small overlay on screen - DirectX?
- [ ] Upgrading connection to P2P via UDP Hole Punching - hard time with NAT


### Issues
Currently frames are about 8mb each, at 60 fps would require about 4GB/s internet - which is unfeasible. Therefore compression and resizing needs to be implemented




