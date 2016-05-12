# Setup output directory
If (Test-Path "bin") {
    Remove-Item -Recurse -Force "bin"
}

New-Item "bin" -ItemType Directory
Push-Location "bin"
New-Item "x64" -ItemType Directory
New-Item "x86" -ItemType Directory
Pop-Location

###
# Build serialization library

# x64
Push-Location lib\serialization
rustup update nightly-2016-05-10-x86_64-pc-windows-msvc
rustup run nightly-2016-05-10-x86_64-pc-windows-msvc cargo clean
rustup run nightly-2016-05-10-x86_64-pc-windows-msvc cargo build --release
Pop-Location
Copy-Item -Path ".\lib\serialization\target\release\serialization.*" -Destination "bin\x64"

# x86
Push-Location lib\serialization
rustup update nightly-2016-05-10-i686-pc-windows-msvc
rustup run nightly-2016-05-10-i686-pc-windows-msvc cargo clean
rustup run nightly-2016-05-10-i686-pc-windows-msvc cargo build --release
Pop-Location
Copy-Item -Path ".\lib\serialization\target\release\serialization.*" -Destination "bin\x86"


