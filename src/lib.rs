use std::{
    mem::size_of,
    os::raw::{c_char, c_int, c_uchar, c_void},
    ptr::null,
};

#[allow(non_camel_case_types)]
mod ffmpeg;
use ffmpeg::*;

/// cbindgen:ignore
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct MoqContext {
    pub av_class: *const AVClass,
    pub foo: c_int,
}

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut libmoqprotocol: AVClass = AVClass {
    class_name: "libmoqprotocol\0".as_ptr() as *const c_char,
    item_name: None,
    option: null(),
    version: 0,
    log_level_offset_offset: 0,
    parent_log_context_offset: 0,
    category: AVClassCategory::AV_CLASS_CATEGORY_OUTPUT,
    get_category: None,
    query_ranges: None,
    child_next: None,
    child_class_iterate: None,
};

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut moq_context: MoqContext = MoqContext {
    av_class: unsafe { &libmoqprotocol as *const AVClass }, // TODO: Uhhh...?
    foo: 0,
};

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
    priv_data_class: unsafe { &libmoqprotocol as *const AVClass }, // TODO: Uhhhhh...?
    priv_data_size: size_of::<MoqContext>() as c_int,
    flags: 0,
    url_check: None,
    url_open_dir: None,
    url_read_dir: None,
    url_close_dir: None,
    url_delete: None,
    url_move: None,
    default_whitelist: std::ptr::null(),
};

//pub static mut moq_context: Option<AVClass> = None;

//moq_open
#[no_mangle]
pub extern "C" fn moq_open(
    url_ctx_ptr: *mut URLContext,
    _url: *const c_char,
    _flags: c_int,
) -> c_int {
    println!("moq_open");
    // unsafe {
    //     moq_context = None;
    // }
    let mut url_context = unsafe { *url_ctx_ptr };
    dbg!(&url_context);
    let av_class = unsafe { *url_context.av_class };
    dbg!(&av_class);
    let class_name = unsafe { std::ffi::CStr::from_ptr(av_class.class_name) };
    dbg!(&class_name);
    let moq_context_size = size_of::<MoqContext>();
    dbg!(&moq_context_size);
    url_context.priv_data = unsafe { &mut moq_context as *mut _ as *mut c_void };

    // url_context.av_class = Some(&moq_context);
    0
}

//moq_write
#[no_mangle]
pub extern "C" fn moq_write(
    url_ctx_ptr: *mut URLContext,
    _buf: *const c_uchar,
    size: c_int,
) -> c_int {
    //println!("moq_write");
    //dbg!(size);
    let url_context = unsafe { *url_ctx_ptr };
    let moq_ctx_ptr = url_context.priv_data as *mut MoqContext;
    let moq_ctx = unsafe { &mut *moq_ctx_ptr };
    dbg!(&moq_ctx.foo);
    moq_ctx.foo += 1;
    size
}
//moq_close
#[no_mangle]
pub extern "C" fn moq_close(_url_ctx_ptr: *mut URLContext) -> c_int {
    0
}
