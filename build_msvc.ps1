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

##################################
# Build messageipc library

# x64
Push-Location lib\messageipc
rustup run $Rust64 cargo clean
rustup run $Rust64 cargo build --release
Pop-Location
Copy-Item -Path ".\lib\messageipc\target\release\messageipc.*" -Destination "bin\x64"

# x86
Push-Location lib\messageipc
rustup run $Rust32 cargo clean
rustup run $Rust32 cargo build --release
Pop-Location
Copy-Item -Path ".\lib\messageipc\target\release\messageipc.*" -Destination "bin\x86"

function Build-Import($dir, $lib, $arch)
{
    Enable-VSPrompt 14
    Push-Location $dir

    dumpbin /exports "$lib.dll" > "$lib.exports"
    echo "LIBRARY $lib" > "$lib.def"
    echo "EXPORTS" >> "$lib.def"

    $lines = [IO.File]::ReadAllLines("$(pwd)\$lib.exports")
    For ($i = 19; $i -lt $lines.Length; $i++)
    {
        If ($lines[$i] -eq "")
        {
            break;
        }

        $sym_split = $lines[$i].Split(" ")
        $sym = $sym_split[$sym_split.Length - 1]

        echo "  $sym" >> "$lib.def"
    }

    lib "/def:$lib.def" "/out:$lib.dll.lib" "/machine:$arch"
    rm "$lib.exports"
    rm "$lib.dll.exp"

    Pop-Location
}

Build-Import ".\bin\x64" "serialization" "x64"
Build-Import ".\bin\x86" "serialization" "x86"
Build-Import ".\bin\x64" "imageload" "x64"
Build-Import ".\bin\x86" "imageload" "x86"
Build-Import ".\bin\x64" "messageipc" "x64"
Build-Import ".\bin\x86" "messageipc" "x86"
