# CreateRemoteThread

## Internals
Win32 API used:
* OpenProcess 
* VirtualAllocEx
* WriteProcessMemory
* CreateRemoteThread


## How to use
Cross-compile from Linux: 
- `git clone https://github.com/Nariod/Tartocitron.git`
- `cd CreateRemoteThread`
- change the shellcode in "main.rs"
- `cargo build --release --target x86_64-pc-windows-gnu`


## Overview
Works, not flagged by Windows Defender (last test on 19/02/22). Binary final size is ~1,9Mo.