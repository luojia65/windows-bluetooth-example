#![allow(non_camel_case_types, unused, non_snake_case)]
use winapi::{
    shared::{
        ws2def::AF_BTH,
        minwindef::{BOOL, DWORD, ULONG, UCHAR, LPVOID,
            MAKEWORD, LOBYTE, HIBYTE, BYTE, TRUE, FALSE},
        ntdef::{LPWSTR, LPCWSTR, WCHAR, ULONGLONG, HANDLE},
        windef::{HWND}
    },
    um::{
        minwinbase::{SYSTEMTIME},
        winsock2::{
            SOCK_STREAM, INVALID_SOCKET,
            WSADATA, 
            socket, connect,
            WSAStartup, WSAGetLastError, WSACleanup
        },
        errhandlingapi::GetLastError
    },
    STRUCT
};
use std::os::raw::c_int;
use std::ffi::CStr;

macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
        [$stype:ty; $ssize:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])*
        pub struct $name([$stype; $ssize]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
    ($(#[$attrs:meta])* union $name:ident {
        [$stype32:ty; $ssize32:expr] [$stype64:ty; $ssize64:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])* #[cfg(target_arch = "x86")]
        pub struct $name([$stype32; $ssize32]);
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "64")]
        pub struct $name([$stype64; $ssize64]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
}

pub const BLUETOOTH_MAX_NAME_SIZE: usize = 248;
pub type PFN_DEVICE_CALLBACK = Option<unsafe extern "system" fn(
    pvParam: LPVOID,
    pDevice: *const BLUETOOTH_DEVICE_INFO,
)>;
pub type PBLUETOOTH_DEVICE_INFO = *mut BLUETOOTH_DEVICE_INFO;
pub type BTH_ADDR = ULONGLONG;
pub type HBLUETOOTH_DEVICE_FIND = LPVOID;
pub type HBLUETOOTH_RADIO_FIND = LPVOID;
UNION!{union BLUETOOTH_ADDRESS{
  [u8; 8],
  ullLong ullLong_mut: BTH_ADDR,
  rgBytes rgBytes_mut: [BYTE; 6],
}}
STRUCT!{struct BLUETOOTH_DEVICE_INFO{ 
  dwSize: DWORD,
  _padding: [u8; 4], // todo?
  Address: BLUETOOTH_ADDRESS,
  ulClassofDevice: ULONG,
  fConnected: BOOL,
  fRemembered: BOOL,
  fAuthenticated: BOOL,
  stLastSeen: SYSTEMTIME,
  stLastUsed: SYSTEMTIME,
  szName: [WCHAR; BLUETOOTH_MAX_NAME_SIZE],
}}
STRUCT!{struct BLUETOOTH_COD_PAIRS{
  ulCODMask: ULONG,
  pcszDescription: LPCWSTR,
}}
STRUCT!{struct BLUETOOTH_SELECT_DEVICE_PARAMS {
  dwSize: DWORD,
  cNumOfClasses: ULONG,
  prgClassOfDevices: *mut BLUETOOTH_COD_PAIRS,
  pszInfo: LPWSTR,
  hwndParent: HWND,
  fForceAuthentication: BOOL,
  fShowAuthenticated: BOOL,
  fShowRemembered: BOOL,
  fShowUnknown: BOOL,
  fAddNewDeviceWizard: BOOL,
  fSkipServicesPage: BOOL,
  pfnDeviceCallback: PFN_DEVICE_CALLBACK,
  pvParam: LPVOID,
  cNumDevices: DWORD,
  pDevices: PBLUETOOTH_DEVICE_INFO,
}}
STRUCT!{struct BLUETOOTH_DEVICE_SEARCH_PARAMS {
  dwSize: DWORD,
  fReturnAuthenticated: BOOL,
  fReturnRemembered: BOOL,
  fReturnUnknown: BOOL,
  fReturnConnected: BOOL,
  fIssueInquiry: BOOL,
  cTimeoutMultiplier: UCHAR,
  hRadio: HANDLE,
}}
STRUCT!{struct BLUETOOTH_FIND_RADIO_PARAMS {
  dwSize: DWORD,
}}

extern "system" {
    pub fn BluetoothSelectDevices (
        pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS
    ) -> BOOL;
    pub fn BluetoothSelectDevicesFree (
        pbtsdp: *mut BLUETOOTH_SELECT_DEVICE_PARAMS
    ) -> BOOL;
    pub fn BluetoothFindFirstDevice (
        pbtsdp: *const BLUETOOTH_DEVICE_SEARCH_PARAMS,
        pbtdi: *mut BLUETOOTH_DEVICE_INFO,
    ) -> HBLUETOOTH_DEVICE_FIND;
    pub fn BluetoothFindNextDevice (
        hFind: HBLUETOOTH_DEVICE_FIND,
        pbtdi: *mut BLUETOOTH_DEVICE_INFO,
    ) -> BOOL;
    // there is a 'free' function
    pub fn BluetoothFindFirstRadio (
        pbtfrp: *const BLUETOOTH_FIND_RADIO_PARAMS,
        phRadio: *mut HANDLE,
    ) -> HBLUETOOTH_RADIO_FIND;
    // free function
}

const BTHPROTO_RFCOMM: c_int = 3;

fn main() {
    // unsafe { select_device_example() };
    // println!("== Startup ==");
    // unsafe { startup_example() };
    // println!("== Make Socket ==");
    // unsafe { make_socket_example() }; 
    // println!("== Cleanup ==");
    // unsafe { WSACleanup(); }
    // unsafe { find_device_example() };
    unsafe {
        let btfrp = Box::new(BLUETOOTH_FIND_RADIO_PARAMS {
            dwSize: core::mem::size_of::<BLUETOOTH_FIND_RADIO_PARAMS>() as DWORD,
        });
        let pbtfrp = Box::into_raw(btfrp);
        let hRadio = Box::new(core::ptr::null_mut());
        let phRadio = Box::into_raw(hRadio);
        let hFind = BluetoothFindFirstRadio(pbtfrp, phRadio);
        let mut hRadio = Box::from_raw(phRadio);
        // while hRadio != core::ptr::null_mut() {

        // }
    }
}

unsafe fn find_device_example() {
    let bdsp = Box::new(BLUETOOTH_DEVICE_SEARCH_PARAMS {
        dwSize: core::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32,
        fReturnAuthenticated: TRUE,
        fReturnRemembered: TRUE,
        fReturnUnknown: FALSE,
        fReturnConnected: TRUE,
        fIssueInquiry: FALSE,
        cTimeoutMultiplier: 10,
        hRadio: core::ptr::null_mut(),
    });
    let pbdsp = Box::into_raw(bdsp);
    let btdi = Box::new(BLUETOOTH_DEVICE_INFO {
        dwSize: core::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32,
        _padding: core::mem::zeroed(), 
        Address: core::mem::zeroed(),
        ulClassofDevice: 0,
        fConnected: 0,
        fRemembered: 0,
        fAuthenticated: 0,
        stLastSeen: core::mem::zeroed(),
        stLastUsed: core::mem::zeroed(),
        szName: [0; BLUETOOTH_MAX_NAME_SIZE],
    });
    let pbtdi = Box::into_raw(btdi); 
    let hFind = BluetoothFindFirstDevice(pbdsp, pbtdi);
    print_pbtdi(pbtdi);
    while BluetoothFindNextDevice(hFind, pbtdi) == TRUE {
        print_pbtdi(pbtdi);
    }

    unsafe fn print_pbtdi(pbtdi: *mut BLUETOOTH_DEVICE_INFO) {
        let btdi = Box::from_raw(pbtdi.clone());
        println!("{:X} ", btdi.dwSize);
        print!("{:X} ", btdi.Address.ullLong());
        print!("{:X} ", btdi.ulClassofDevice);
        print!("{:X} ", btdi.fConnected);
        print!("{:X} ", btdi.fRemembered);
        println!("{}", String::from_utf16(&btdi.szName as &[u16]).unwrap());
    }
}

unsafe fn select_device_example() {
    let pbtsdp = Box::new(BLUETOOTH_SELECT_DEVICE_PARAMS {
        dwSize: core::mem::size_of::<BLUETOOTH_SELECT_DEVICE_PARAMS>() as u32,
        cNumOfClasses: 0,
        prgClassOfDevices: core::ptr::null_mut(),
        pszInfo: core::ptr::null_mut(),
        hwndParent: core::ptr::null_mut(),
        fForceAuthentication: FALSE,
        fShowAuthenticated: TRUE,
        fShowRemembered: TRUE,
        fShowUnknown: TRUE,
        fAddNewDeviceWizard: FALSE,
        fSkipServicesPage: FALSE,
        pfnDeviceCallback: None,
        pvParam: core::ptr::null_mut(),
        cNumDevices: 0,
        pDevices: core::ptr::null_mut(),
    });
    let ptr = Box::into_raw(pbtsdp);
    let ans = BluetoothSelectDevices(ptr);
    if ans == FALSE {
        println!("Err: {}", GetLastError());
        return;
    }
    let pbtsdp = Box::from_raw(ptr);
    println!("cNumDevices: {}", pbtsdp.cNumDevices);
    let devices = pbtsdp.pDevices;
    for i in 0..pbtsdp.cNumDevices {
        print!("Device #{}:", i);
        let device = &*devices.offset(i as isize);
        print!("Size: [{}]", device.dwSize);
        print!("Class: [0x{:X}],", device.ulClassofDevice);
        print!("Name: [{}],", String::from_utf16(&device.szName as &[u16]).unwrap());
        print!("Address: [{:X}]", device.Address.ullLong());
        print!("Connected: [{:X}]", device.fConnected);
        print!("Remembered: [{:X}]", device.fRemembered);
        println!()
    }
    let ptr = Box::into_raw(pbtsdp);
    let ans = BluetoothSelectDevicesFree(ptr);
    println!("Free ans: {}", ans);
    drop(Box::from_raw(ptr));
}

unsafe fn startup_example() {
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

unsafe fn make_socket_example() {
    let sk = socket(AF_BTH, SOCK_STREAM, BTHPROTO_RFCOMM);
    if sk == INVALID_SOCKET {
        let err = WSAGetLastError();
        println!("Err: {}", err);
    }
    println!("socket: {}", sk);

    // let code = connect(sk, name: *const SOCKADDR, namelen: c_int);
}
