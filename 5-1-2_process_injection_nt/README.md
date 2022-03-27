# WIP ProcessInjectionNt

## Internals
Win32 API used:
* NtCreateSection
* NtMapViewOfSection
* NtOpenProcess
* NtUnmapViewOfSection
* NtClose
* NtCreateThreadEx
All these APIs are supported by ntapi Rust crate.


## How to use
Cross-compile from Linux: 
- `git clone https://github.com/Nariod/Tartocitron.git`
- `cd 5-1-2_process_injection_nt`
- change the shellcode in "main.rs"
- `cargo build --release --target x86_64-pc-windows-gnu`


## Overview
