use std::ffi::CStr;
use std::{ptr, slice, mem};
use libc;
use IpcClient;

const MIPC_SUCCESS: libc::c_int = 0;
const MIPC_EMPTY: libc::c_int = 1;
const MIPC_DISCONNECTED: libc::c_int = 2; 

#[no_mangle]
pub extern "C" fn mipc_open_server(name: *const i8) -> *mut IpcClient {
    let name = match unsafe { CStr::from_ptr(name) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    
    match IpcClient::open_server(name) {
        Ok(server) => Box::into_raw(Box::new(server)),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mipc_open_client(name: *const i8, pid: u32) -> *mut IpcClient {
    let name = match unsafe { CStr::from_ptr(name) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    
    match IpcClient::open_client(name, pid) {
        Ok(client) => Box::into_raw(Box::new(client)),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mipc_close(client: *mut IpcClient) {
    unsafe { Box::from_raw(client) };
}

#[no_mangle]
pub extern "C" fn mipc_send(client: *mut IpcClient, data: *const u8, len: usize) -> libc::c_int {
    let client = unsafe { &*client };
    let buf = unsafe { slice::from_raw_parts(data, len) };
    if client.send(Vec::from(buf)) {
        MIPC_SUCCESS
    } else {
        MIPC_DISCONNECTED
    }
}

#[no_mangle]
pub extern "C" fn mipc_recv(client: *mut IpcClient, data: *mut *mut u8, len: *mut usize) -> libc::c_int {
    let client = unsafe { &*client };
    match client.recv() {
        Some(mut buf) => unsafe {
            buf.shrink_to_fit();
            assert!(buf.capacity() == buf.len());
            *data = buf.as_mut_ptr();
            *len = buf.len();
            mem::forget(buf);
            MIPC_SUCCESS
        },
        None => MIPC_DISCONNECTED,
    }
}

#[no_mangle]
pub extern "C" fn mipc_try_recv(client: *mut IpcClient, data: *mut *mut u8, len: *mut usize) -> libc::c_int {
    let client = unsafe { &*client };
    match client.try_recv() {
        Some(Some(mut buf)) => unsafe {
            buf.shrink_to_fit();
            assert!(buf.capacity() == buf.len());
            *data = buf.as_mut_ptr();
            *len = buf.len();
            mem::forget(buf);
            MIPC_SUCCESS
        },
        Some(None) => MIPC_EMPTY,
        None => MIPC_DISCONNECTED,
    }
}

#[no_mangle]
pub extern "C" fn mipc_recv_free(data: *mut u8, len: usize) {
    unsafe { Vec::from_raw_parts(data, len, len) };
}
