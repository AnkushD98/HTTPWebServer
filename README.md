Git clone the repo.
Install Rust. You can do this with curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh which I took from the Rust website https://www.rust-lang.org/tools/install
cd into the Git repo, and run cargo build --release
The binary is located at target/release/http_server.exe
