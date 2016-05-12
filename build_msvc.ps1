# Build the rust stuff
Push-Location lib\serialization
rustup update nightly-2016-05-10-x86_64-pc-windows-msvc
rustup run nightly-2016-05-10-x86_64-pc-windows-msvc cargo build --release
Pop-Location

