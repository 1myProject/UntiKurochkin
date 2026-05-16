set SDKROOT=C:\SDKs\MacOSX11.3.sdk
if (-not (Test-Path "bins")) { New-Item -ItemType Directory -Path "bins" }


cargo zigbuild --target aarch64-apple-darwin --release
cargo zigbuild --target x86_64-apple-darwin --release
cargo build --release

move .\target\x86_64-apple-darwin\release\avto_reshala .\bins\avto_reshala_apple_intel
move .\target\aarch64-apple-darwin\release\avto_reshala .\bins\avto_reshala_apple_silicon
move .\target\release\avto_reshala.exe .\bins\avto_reshala_win_x64.exe


