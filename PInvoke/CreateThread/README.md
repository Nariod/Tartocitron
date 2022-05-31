# CreateThread

## Internals
Win32 API used:
* VirtualAlloc 
* CreateThread
* WaitForSingleObject


## How to use
Cross-compile from Linux: 
- `git clone https://github.com/Nariod/Tartocitron.git`
- `cd CreateThread`
- change the shellcode in "main.rs"
- `cargo build --release --target x86_64-pc-windows-gnu`


## Overview
Works, but is sometimes detected by Windows Defender when executed (Last test on 17/02/22). Binary final size is ~1,6Mo.