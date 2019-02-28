use winapi::{
    shared::{
        ws2def::AF_BTH,
        minwindef::{MAKEWORD, LOBYTE, HIBYTE},
    },
    um::winsock2::{
        SOCK_STREAM, INVALID_SOCKET,
        WSADATA, 
        socket, connect,
        WSAStartup, WSAGetLastError, WSACleanup
    }
};
use std::os::raw::c_int;
use std::ffi::CStr;

const BTHPROTO_RFCOMM: c_int = 3;

fn main() {
    println!("== Startup ==");
    unsafe {
        // low: major; high: minor
        let version_word = MAKEWORD(2, 2);
        let wsa_data = Box::new(WSADATA {
            wVersion: 0,
            wHighVersion: 0,
            szDescription: [0; 257],
            szSystemStatus: [0; 129],
            // ↓ These three lines should be ignored after 2.0 -- MSDN
            iMaxSockets: 0, 
            iMaxUdpDg: 0,
            lpVendorInfo: core::ptr::null_mut(), 
        });
        let ptr = Box::into_raw(wsa_data);
        let code = WSAStartup(version_word, ptr);
        println!("startup code: {}", code);
        // format message w?
        let wsa_data = Box::from_raw(ptr);
        println!("Version: {}.{}\nHigh Version: {}.{}\nDescription: {}\nSystem Status: {}", 
            LOBYTE(wsa_data.wVersion), HIBYTE(wsa_data.wVersion), 
            LOBYTE(wsa_data.wHighVersion), HIBYTE(wsa_data.wHighVersion),
            CStr::from_ptr(&wsa_data.szDescription as *const i8).to_string_lossy(), 
            CStr::from_ptr(&wsa_data.szSystemStatus as *const i8).to_string_lossy()
        );
        if wsa_data.wVersion != version_word {
            println!("Could not find a usable version of Winsock.dll\n");
            WSACleanup();
            return;
        }
    }
    println!("== Make Socket ==");
    unsafe {
        let sk = socket(AF_BTH, SOCK_STREAM, BTHPROTO_RFCOMM);
        if sk == INVALID_SOCKET {
            let err = WSAGetLastError();
            println!("Err: {}", err);
        }
        println!("socket: {}", sk);
        
        let code = connect(sk, name: *const SOCKADDR, namelen: c_int);
    } 
    println!("== Cleanup ==");
    unsafe {
        WSACleanup();
    }
}
