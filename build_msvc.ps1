# Setup output directory
If (Test-Path "bin") {
    Remove-Item -Recurse -Force "bin"
}

New-Item "bin" -ItemType Directory > $null
Push-Location "bin"
New-Item "x64" -ItemType Directory > $null
New-Item "x86" -ItemType Directory > $null
Pop-Location

###
# Build serialization library

rustup update stable-x86_64-pc-windows-msvc
rustup update stable-i686-pc-windows-msvc

# x64
Push-Location lib\serialization
rustup run stable-x86_64-pc-windows-msvc cargo clean
rustup run stable-x86_64-pc-windows-msvc cargo build --release
Pop-Location
Copy-Item -Path ".\lib\serialization\target\release\serialization.*" -Destination "bin\x64"

# x86
Push-Location lib\serialization
rustup run stable-i686-pc-windows-msvc cargo clean
rustup run stable-i686-pc-windows-msvc cargo build --release
Pop-Location
Copy-Item -Path ".\lib\serialization\target\release\serialization.*" -Destination "bin\x86"


