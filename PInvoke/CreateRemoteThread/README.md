# CreateRemoteThread

## Internals
Win32 API used:
* OpenProcess 
* VirtualAllocEx
* WriteProcessMemory
* CreateRemoteThread


## How to use
This project uses the official "windows" crate, compiling from Windows is recommended. 
- `git clone https://github.com/Nariod/Tartocitron.git`
- `cd CreateRemoteThread`
- use the "aes_encryption_decryption" tool in this repo to create an AES encrypted shellcode
- change the shellcode in "main.rs"
- `cargo build --release`


## Overview
Works, not flagged by Windows Defender (last test on 31/05/22). Binary final size is ~3Mo.