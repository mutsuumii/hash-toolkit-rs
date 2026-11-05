# hash-toolkit-rs 🦀

CLI hash toolkit in Rust — compute, verify, and compare file hashes.

## Supported Algorithms
MD5, SHA-1, SHA-256, SHA-512, BLAKE2b

## Build & Run
```bash
cargo build --release
./target/release/hash-toolkit-rs file ./myfile.bin -a sha256
./target/release/hash-toolkit-rs verify ./myfile.bin abc123... -a sha256
./target/release/hash-toolkit-rs dir ./my_folder -a md5
./target/release/hash-toolkit-rs string "hello world"
```

## License
MIT
