<p align="left">
Follow steps to run the application:

1.  Git clone the repo.
2.  Install Rust. You can do this with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` which I took from the Rust website [https://www.rust-lang.org/tools/install](https://github.com/RustScan/RustScan/wiki/Installation-Guide)
3.  cd into the Git repo, and run `cargo build --release`
4.  The binary is located at `target/release/http_server.exe`. Run the exe.
5.  Open up any web browser and goto `http://127.0.0.1:7878` and view the response from the server! Try with multiple tabs at once as this is multi-threaded.
</p>
