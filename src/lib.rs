use anyhow::Context;
use std::{
    mem::size_of,
    os::raw::{c_char, c_int, c_uchar, c_void},
    ptr::null,
    str::FromStr,
};

use moq_transport::model::broadcast;

#[allow(non_camel_case_types)]
mod ffmpeg;
use ffmpeg::*;

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

/// cbindgen:ignore
#[derive(Debug)]
#[repr(C)]
pub struct MoqContext {
    pub av_class: *const AVClass,
    pub foo: c_int,
    publisher: moq_transport::model::broadcast::Publisher,
    session: moq_transport::session::Publisher,
    rt: tokio::runtime::Runtime,
}

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

impl MoqContext {
    pub fn new(url_context: URLContext) -> Result<Self, Box<dyn std::error::Error>> {
        let bind_address = std::net::SocketAddr::from_str("[::]:0")?;
        let uri = http::Uri::from_str("https://localhost:4443")?;

        let av_class = unsafe { *url_context.av_class };

        // let rt = tokio::runtime::Builder::new_multi_thread()
        //     .enable_all()
        //     .build()?;
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let _enter_guard = rt.enter(); // Let quinn know we have a runtime?
        let mut roots = rustls::RootCertStore::empty();
        for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs")
        {
            roots.add(&rustls::Certificate(cert.0)).unwrap();
        }

        let mut tls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(roots)
            .with_no_client_auth();

        tls_config.alpn_protocols = vec![webtransport_quinn::ALPN.to_vec()]; // this one is important

        let arc_tls_config = std::sync::Arc::new(tls_config);
        let quinn_client_config = quinn::ClientConfig::new(arc_tls_config);

        let mut endpoint = quinn::Endpoint::client(bind_address)?;
        endpoint.set_default_client_config(quinn_client_config);

        let session = rt
            .block_on(webtransport_quinn::connect(&endpoint, &uri))
            .context("failed to create WebTransport session")?;

        let mut session = rt
            .block_on(moq_transport::Client::publisher(session))
            .context("failed to create MoQ Transport session")?;

        let (publisher, subscriber, _) = broadcast::new("quic.video");
        session
            .announce(subscriber)
            .context("failed to announce broadcast")?;

        Ok(Self {
            av_class: &av_class,
            foo: 0,
            publisher,
            session,
            rt,
        })
    }
}

//moq_open
#[no_mangle]
pub extern "C" fn moq_open(
    url_ctx_ptr: *mut URLContext,
    _url: *const c_char,
    _flags: c_int,
) -> c_int {
    println!("moq_open");
    let url_context = unsafe { *url_ctx_ptr };
    dbg!(&url_context);
    let av_class = unsafe { *url_context.av_class };
    dbg!(&av_class);
    let class_name = unsafe { std::ffi::CStr::from_ptr(av_class.class_name) };
    dbg!(&class_name);
    let moq_context_size = size_of::<MoqContext>();
    dbg!(&moq_context_size);

    let url = unsafe { std::ffi::CStr::from_ptr(url_context.filename) };
    dbg!(url);
    // TODO: parse URL
    // moq://<relay_hostname:relay_port>/<namespace>

    unsafe {
        std::ptr::write(
            url_context.priv_data as *mut MoqContext,
            MoqContext::new(url_context).unwrap(),
        );
    }

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
    dbg!(moq_ctx);
    size
}
//moq_close
#[no_mangle]
pub extern "C" fn moq_close(_url_ctx_ptr: *mut URLContext) -> c_int {
    0
}
