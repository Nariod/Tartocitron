# SimpleInjectorDLL

## Internals
Win32 API used:
* VirtualAlloc 
* CreateThread
* WaitForSingleObject


## How to use
Cross-compile from Linux: 
- `git clone https://github.com/Nariod/Tartocitron.git`
- `cd 4-2-5_simple_injector_dll`
- change the shellcode in "lib.rs"
- `cargo build --release --target x86_64-pc-windows-gnu`
On target machine:
- `rundll32.exe .\simple_injector_dll.dll, main`


## Overview
Works, not flagged by Windows Defender (last test on 18/02/22). Binary final size is ~250Ko.