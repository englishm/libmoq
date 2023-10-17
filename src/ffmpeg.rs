use crate::ffmpeg_types::*;
use crate::media::*;
use crate::moq::*;

use std::{
    mem::size_of,
    os::raw::{c_char, c_int, c_uchar},
    ptr::null,
};

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
pub static mut ff_libmoq_protocol: URLProtocol = URLProtocol {
    name: "moq\0".as_ptr() as *const c_char,
    //url_open: moq_open as *const fn(*mut URLContext, *const c_char, c_int) -> c_int,
    url_open: ff_moq_open,
    url_write: Some(ff_moq_write),
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

//moq_open
#[no_mangle]
pub extern "C" fn ff_moq_open(
    url_ctx_ptr: *mut URLContext,
    _url: *const c_char,
    _flags: c_int,
) -> c_int {
    println!("moq_open");
    let url_context = unsafe { *url_ctx_ptr };

    unsafe {
        std::ptr::write(
            url_context.priv_data as *mut MoqContext,
            MoqContext::new(url_context).unwrap(),
        );
    }

    0
}

//moq_write
#[no_mangle]
pub extern "C" fn ff_moq_write(
    url_ctx_ptr: *mut URLContext,
    buf_ptr: *const c_uchar,
    size: c_int,
) -> c_int {
    println!("moq_write");
    //dbg!(size);
    let url_context = unsafe { *url_ctx_ptr };
    let moq_ctx_ptr = url_context.priv_data as *mut MoqContext;
    let mut moq_ctx = unsafe { &mut *moq_ctx_ptr };
    let buf: &[u8] = unsafe { std::slice::from_raw_parts(buf_ptr, size.try_into().unwrap()) };

    dbg!(moq_ctx.unread.len());
    // append new bytes from buf to unread
    moq_ctx.unread.append(&mut buf.to_vec());
    dbg!(moq_ctx.unread.len());

    println!("tracks: {:?}", moq_ctx.tracks.len());
    println!("size: {}", size);

    if moq_ctx._catalog.is_none() {
        println!("Populating init and .catalog tracks");
        let read_bytes = match init_tracks(&mut moq_ctx) {
            Ok(read_bytes) => read_bytes,
            Err(err) => {
                // Failed to parse init tracks
                dbg!(moq_ctx);
                todo!("Handle error: {:?}", err)
            }
        };
        // drain read bytes from unread
        moq_ctx.unread.drain(0..(read_bytes as usize));
        // report that we've read all the bytes (at least into our unread buffer)
        return size;
    }

    while moq_ctx.unread.len() > 8 {
        // Handle moof or mdat atoms
        match handle_atom(&mut moq_ctx) {
            Ok(read_bytes) => {
                // drain read bytes from unread
                moq_ctx.unread.drain(0..(read_bytes as usize));

                // Return after handling only one atom for some reason???
                return size;
            }
            Err(err) => {
                // Failed to parse init tracks
                todo!("Handle error: {:?}", err)
            }
        };
    }

    //dbg!(&moq_ctx.tracks);

    // report that we've read all the bytes (at least into our unread buffer)
    size
}
//moq_close
#[no_mangle]
pub extern "C" fn ff_moq_close(url_ctx_ptr: *mut URLContext) -> c_int {
    println!("moq_close");
    let url_context = unsafe { *url_ctx_ptr };
    let moq_ctx_ptr = url_context.priv_data as *mut MoqContext;
    let moq_ctx = unsafe { &mut *moq_ctx_ptr };

    // Take the publisher from moq_ctx
    let publisher = moq_ctx.publisher.take().unwrap();
    // close publisher
    publisher.close(moq_transport::Error::Closed).unwrap();

    // TODO: get result of session.run() from session_join_handle?

    0
}
