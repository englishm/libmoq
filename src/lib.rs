use std::os::raw::{c_char, c_int, c_uchar};

#[allow(non_camel_case_types)]
mod ffmpeg;
use ffmpeg::*;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut ff_libmoq_protocol: URLProtocol = URLProtocol {
    name: "moq\0".as_ptr() as *const c_char,
    url_open: moq_open as *const fn(*mut URLContext, *const c_char, c_int) -> c_int,
    url_write: moq_write as *const fn(*mut URLContext, *const c_char, c_int) -> c_int,
    url_open2: std::ptr::null(),
    url_accept: std::ptr::null(),
    url_handshake: std::ptr::null(),
    url_read: std::ptr::null(),
    url_seek: std::ptr::null(),
    url_close: std::ptr::null(),
    url_read_pause: std::ptr::null(),
    url_read_seek: std::ptr::null(),
    url_get_file_handle: std::ptr::null(),
    url_get_multi_file_handle: std::ptr::null(),
    url_get_short_seek: std::ptr::null(),
    url_shutdown: std::ptr::null(),
    priv_data_class: None,
    priv_data_size: 0,
    flags: 0,
    url_check: std::ptr::null(),
    url_open_dir: std::ptr::null(),
    url_read_dir: std::ptr::null(),
    url_close_dir: std::ptr::null(),
    url_delete: std::ptr::null(),
    url_move: std::ptr::null(),
    default_whitelist: std::ptr::null(),
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
