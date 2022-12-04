# Eval Args
param ([Parameter(Mandatory)]$gdlibrary,
       [Parameter(Mandatory)]$class)

# Make gdscript
(Get-Content -Path ".\cargo-build\templates\gdns.gdns" -Raw) -replace "_X_CLASS_X_", $class -replace "_X_LIBRARY_X_", $gdlibrary | Out-File -FilePath ".\scripts\$class.gdns"
