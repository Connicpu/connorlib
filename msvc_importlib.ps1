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

    lib /nologo "/def:$lib.def" "/out:$lib.dll.lib" "/machine:$arch"
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

