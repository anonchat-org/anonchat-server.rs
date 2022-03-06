# anonchat-server.rs
A Rust implementation of anonchat v2 server protocol.


## Features
- Blazingly fast
- Simple codebase
- Easy to setup
- No external dependencies


## Building guide

### Manually
1. Clone this repository
2. Install latest Rust
3. Run `cargo build --release`
4. The binary will be available at `./target/release/anonchat-server`

### Via Docker
1. Install latest Docker
2. In the project directory run `sudo docker build -t anonchat-server.rs .`
3. Run `sudo docker run -i -p <port>:<port> anonchat-server.rs <host:port>`

   Example:
   ```sh
   sudo docker run -i -p 6969:6969 anonchat-server.rs 0.0.0.0:6969
   ```


## Usage example
```sh
$ anonchat-server 0.0.0.0:6969
```


## Contributing
Issues and PRs are welcome. Feel free to contribute!


## License
The project is licensed under MIT License, see the `LICENSE` file.
