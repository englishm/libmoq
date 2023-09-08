use std::os::raw::{c_char, c_int, c_uchar};

#[allow(non_camel_case_types)]
mod ffmpeg;
use ffmpeg::*;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut ff_libmoq_protocol: URLProtocol = URLProtocol {
    name: "moq\0".as_ptr() as *const c_char,
    url_open: moq_open as *const fn(*mut URLContext, *const c_char, c_int) -> c_int,
};

//moq_open
#[no_mangle]
pub extern "C" fn moq_open(
    _url_ctx_ptr: *mut URLContext,
    _url: *const c_char,
    _flags: c_int,
) -> c_int {
    println!("moq_open");
    0
}

//moq_write
#[no_mangle]
pub extern "C" fn moq_write(
    _url_ctx_ptr: *mut URLContext,
    _buf: *const c_uchar,
    _size: c_int,
) -> c_int {
    0
}
//moq_close
#[no_mangle]
pub extern "C" fn moq_close(_url_ctx_ptr: *mut URLContext) -> c_int {
    0
}
