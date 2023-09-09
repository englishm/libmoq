use std::os::raw::{c_char, c_int, c_uchar};

#[allow(non_camel_case_types)]
mod ffmpeg;
use ffmpeg::*;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut ff_libmoq_protocol: URLProtocol = URLProtocol {
    name: "moq\0".as_ptr() as *const c_char,
    //url_open: moq_open as *const fn(*mut URLContext, *const c_char, c_int) -> c_int,
    url_open: moq_open,
    url_write: Some(moq_write),
    url_open2: None,
    url_accept: None,
    url_handshake: None,
    url_read: None,
    url_seek: None,
    url_close: None,
    url_read_pause: None,
    url_read_seek: None,
    url_get_file_handle: None,
    url_get_multi_file_handle: None,
    url_get_short_seek: None,
    url_shutdown: None,
    priv_data_class: None,
    priv_data_size: 0,
    flags: 0,
    url_check: None,
    url_open_dir: None,
    url_read_dir: None,
    url_close_dir: None,
    url_delete: None,
    url_move: None,
    default_whitelist: std::ptr::null(),
};

pub static mut moq_context: Option<AVClass> = None;
//moq_open
#[no_mangle]
pub extern "C" fn moq_open(
    url_ctx_ptr: *mut URLContext,
    _url: *const c_char,
    _flags: c_int,
) -> c_int {
    println!("moq_open");
    unsafe {
        moq_context = None;
        // moq_context = Some(AVClass {
        //     class_name: (),
        //     item_name: (),
        //     option: (),
        //     version: (),
        //     log_level_offset_offset: (),
        //     parent_log_context_offset: (),
        //     category: (),
        //     get_category: (),
        //     query_ranges: (),
        //     child_next: (),
        //     child_class_iterate: (),
        // });
    }
    let url_context = unsafe { *url_ctx_ptr };
    // url_context.av_class = Some(&moq_context);
    0
}

//moq_write
#[no_mangle]
pub extern "C" fn moq_write(
    _url_ctx_ptr: *mut URLContext,
    _buf: *const c_uchar,
    _size: c_int,
) -> c_int {
    println!("moq_write");
    0
}
//moq_close
#[no_mangle]
pub extern "C" fn moq_close(_url_ctx_ptr: *mut URLContext) -> c_int {
    0
}
