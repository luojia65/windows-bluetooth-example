use winapi::{
    shared::{
        ws2def::AF_BTH,
        minwindef::MAKEWORD,
    },
    um::winsock2::{
        SOCK_STREAM, INVALID_SOCKET,
        WSADATA, 
        socket, 
        WSAStartup, WSAGetLastError,
    }
};
use std::os::raw::c_int;
use std::ffi::CStr;

const BTHPROTO_RFCOMM: c_int = 3;

fn main() {
    println!("== Startup ==");
    unsafe {
        let data = Box::new(WSADATA {
            wVersion: 0,
            wHighVersion: 0,
            szDescription: [0; 257],
            szSystemStatus: [0; 129],
            // ↓ These three lines should be ignored -- MSDN
            iMaxSockets: 0, 
            iMaxUdpDg: 0,
            lpVendorInfo: core::ptr::null_mut(), 
        });
        let ptr = Box::into_raw(data);
        let code = WSAStartup(MAKEWORD(2, 2), ptr);
        println!("startup code: {}", code);
        let data = Box::from_raw(ptr);
        println!("Version: {}.{}\nHigh Version: {}.{}\nDescription: {}\nSystem Status: {}", 
            data.wVersion >> 8, data.wVersion & 0xFF, 
            data.wHighVersion >> 8, data.wHighVersion & 0xFF,
            CStr::from_ptr(&data.szDescription as *const i8).to_string_lossy(), 
            CStr::from_ptr(&data.szSystemStatus as *const i8).to_string_lossy()
        );
    }
    println!("== Make Socket ==");
    unsafe {
        let sk = socket(AF_BTH, SOCK_STREAM, BTHPROTO_RFCOMM);
        if sk == INVALID_SOCKET {
            let err = WSAGetLastError();
            println!("Err: {}", err);
        }
        println!("socket: {}", sk);
    } 
}
