# Peak | Screen Peak Your Teammates [![HitCount](http://hits.dwyl.com/josephp27/Peak.svg)](http://hits.dwyl.com/josephp27/Peak)

Similar to how Discord overlays chat over gameplay, I am setting out to write a tool to stream video over the game window, so teammates can see your screen. Think whenever you screen peaked as a kid to gain advantage over a friend or another team, but over the internet. 

The goal for this project is to write everything from scratch, using no outside libraries. Uses sockets and UDP network calls to stream video between you and your teammates using minimal libraries, written in pure Rust

Since we should always have the fastest stream, dropping packets is more than ok. Packet ordering happens in O(1) time inserts, for O(n) runtime, where n is the number of packets in a frame

### Running
The application works in 2 parts - server and client. 

#### Server
The server acts as a connection point between all clients, delegating streams to the appropriate location
```bash
cargo run server/src/main.rs
```

#### Client
The client records the screen and sends packets to the server and also listens to packets and displays them over the game. Don't forget to change the [destination](https://github.com/josephp27/Peak/blob/4d1315e181b467e445362f84e49fe0089fd62aba/client/src/utils/constants.rs#L4)
```bash
cargo run client/src/main.rs
```


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
- [ ] Cleanup dependencies


### Issues
Currently frames are about 8mb each, at 60 fps would require about 4GB/s internet - which is unfeasible. Therefore compression and resizing needs to be implemented




