
# Prerequisites
- rustc
- cargo

https://www.rust-lang.org/tools/install

# How to run

```bash
$ cargo build -r
$ ./target/release/server <agent_port> <public_port>
$ ./target/release/agent <server_addr:server_port> <redirect_port>
```

Agent port and server port must be the same, its port which using for communication beetwen agent and server.
