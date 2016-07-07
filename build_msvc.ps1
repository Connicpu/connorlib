# Setup output directory
If (Test-Path "bin") {
    Remove-Item -Recurse -Force "bin"
}

New-Item "bin" -ItemType Directory > $null
Push-Location "bin"
New-Item "x64" -ItemType Directory > $null
New-Item "x86" -ItemType Directory > $null
Pop-Location

##################################
# Ensure rust is updated

$Rust64 = "nightly-x86_64-pc-windows-msvc"
$Rust32 = "nightly-i686-pc-windows-msvc"

rustup update $Rust64
rustup update $Rust32

##################################
# Build serialization library

# x64
Push-Location lib\serialization
rustup run $Rust64 cargo clean
rustup run $Rust64 cargo build --release
Pop-Location
Copy-Item -Path ".\lib\serialization\target\release\serialization.*" -Destination "bin\x64"

# x86
Push-Location lib\serialization
rustup run $Rust32 cargo clean
rustup run $Rust32 cargo build --release
Pop-Location
Copy-Item -Path ".\lib\serialization\target\release\serialization.*" -Destination "bin\x86"

##################################
# Build imageload library

# x64
Push-Location lib\imageload
rustup run $Rust64 cargo clean
rustup run $Rust64 cargo build --release
Pop-Location
Copy-Item -Path ".\lib\imageload\target\release\imageload.*" -Destination "bin\x64"

# x86
Push-Location lib\imageload
rustup run $Rust32 cargo clean
rustup run $Rust32 cargo build --release
Pop-Location
Copy-Item -Path ".\lib\imageload\target\release\imageload.*" -Destination "bin\x86"


