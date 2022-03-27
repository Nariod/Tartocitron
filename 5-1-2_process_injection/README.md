# ProcessInjection

## Internals
Win32 API used:
* OpenProcess 
* VirtualAllocEx
* WriteProcessMemory
* CreateRemoteThread


## How to use
Cross-compile from Linux: 
- `git clone https://github.com/Nariod/Tartocitron.git`
- `cd 5-1-2_process_injection`
- change the shellcode in "main.rs"
- `cargo build --release --target x86_64-pc-windows-gnu`
- WIP : compile works great on Windows, but not cross compilation from Linux

## Overview
Works, not flagged by Windows Defender (last test on 27/03/22). Binary final size is ~360Ko.