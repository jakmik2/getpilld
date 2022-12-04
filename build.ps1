# Build Cargo Libraries
Set-Location "cargo-build"
cargo build
Set-Location ..

# Get Libraries
Write-Output "Process Libraries"

$libraries = Get-ChildItem -Path ".\cargo-build" -Directory | select Name

Write-Output "Beginning deployment."

foreach($folder in $libraries) {
    $name = $folder.Name

    if (($name -eq "target") -or ($name -eq "templates")) {
        continue
    }
    
    Write-Output "Processing $name"

    $dll_name = $name -Replace '-', '_'

    # Make GDNative Library
    (Get-Content -Path ".\cargo-build\templates\gdnlib.gdnlib" -Raw) -replace "_X_PATH_X_", "$dll_name.dll" > ".\bin\$dll_name.gdnlib"
}
