# Builds a Windows .exe distribution for hangboard_timer under .\target
# Run from PowerShell on Windows: .\build_app.ps1
$ErrorActionPreference = "Stop"

Set-Location $PSScriptRoot

$AppName = "hangboard_timer"
$OutDir = "target\$AppName-windows"

cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

if (Test-Path $OutDir) { Remove-Item -Recurse -Force $OutDir }
New-Item -ItemType Directory -Path $OutDir | Out-Null
Copy-Item "target\release\$AppName.exe" $OutDir
# Sounds are loaded via the relative path "assets/sound/...", so keep assets next to the .exe.
Copy-Item -Recurse "assets" "$OutDir\assets"

$Zip = "$OutDir.zip"
if (Test-Path $Zip) { Remove-Item -Force $Zip }
Compress-Archive -Path "$OutDir\*" -DestinationPath $Zip

Write-Host "Created: $OutDir\$AppName.exe"
Write-Host "Created: $Zip"
