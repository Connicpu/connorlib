# Build the rust stuff
Push-Location lib\serialization
rustup override add stable-x86_64-pc-windows-msvc
cargo build --release
rustup override remove
Pop-Location

