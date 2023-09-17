use anyhow::Context;
use std::{
    io::Cursor,
    mem::size_of,
    os::raw::{c_char, c_int, c_uchar, c_void},
    ptr::null,
    str::FromStr, sync::{Arc, Mutex}, collections::HashMap,
};

use moq_transport::model::broadcast;
use moq_transport::model::track;

#[allow(non_camel_case_types)]
mod ffmpeg;
use ffmpeg::*;

mod media;
use media::*;

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
    pub tracks: HashMap<String, media::Track>,
    publisher: Arc<Mutex<broadcast::Publisher>>,
    session_join_handle: tokio::task::JoinHandle<()>,
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

        let url_cstr = unsafe { std::ffi::CStr::from_ptr(url_context.filename) };
        let url_str = String::from_utf8_lossy(url_cstr.to_bytes()).to_string();
        let url = http::Uri::from_str(&url_str)?;
        let mut url_parts = http::Uri::into_parts(url);
        // change uri scheme from moq to https
        url_parts.scheme = Some(http::uri::Scheme::HTTPS);
        let url = http::Uri::from_parts(url_parts)?;

        let av_class = unsafe { *url_context.av_class };

        // create a hashmap to hold tracks on MoqContext
        let mut tracks = HashMap::new();

        // let rt = tokio::runtime::Builder::new_multi_thread()
        //     .enable_all()
        //     .build()?;
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let _enter_guard = rt.enter(); // Let quinn know we have a runtime?

        let (publisher, subscriber) = broadcast::new();

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
            .block_on(webtransport_quinn::connect(&endpoint, &url))
            .context("failed to create WebTransport session")?;

        let session = rt
            .block_on(moq_transport::session::Client::publisher(
                session, subscriber,
            ))
            .context("failed to create MoQ Transport session")?;

        let session_join_handle = tokio::spawn(async move {
            session.run().await.unwrap();
        });

        Ok(Self {
            av_class: &av_class,
            foo: 0,
            tracks,
            publisher: Arc::new(Mutex::new(publisher)),
            session_join_handle,
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
    buf_ptr: *const c_uchar,
    size: c_int,
) -> c_int {
    println!("moq_write");
    //dbg!(size);
    let url_context = unsafe { *url_ctx_ptr };
    let moq_ctx_ptr = url_context.priv_data as *mut MoqContext;
    let mut moq_ctx = unsafe { &mut *moq_ctx_ptr };
    let mut buf: &[u8] = unsafe { std::slice::from_raw_parts(buf_ptr, size.try_into().unwrap()) };

    println!("tracks: {:?}", moq_ctx.tracks.len());

    if moq_ctx.tracks.len() == 0 {
        println!("Populating init and .catalog tracks");
        let read_bytes = match init_tracks(&mut moq_ctx, buf, size){
            Ok(read_bytes) => read_bytes,
            Err(err) => {
                // Failed to parse init tracks
                todo!("Handle error: {:?}", err)
            },
        };
        return read_bytes;
    }


    let atom = match read_atom(&mut buf) {
        Ok(atom) => atom,
        Err(err) => {
            // Failed to read a complete atom
            vec![]
            // TODO: maybe read into "unparsed" vec
        }
    };
    //rt.enter();
    // TODO: Figure out how to set up catalog and init tracks
    //

    let mut reader = Cursor::new(&atom);
    let _header = match mp4::BoxHeader::read(&mut reader) {
        Ok(header) => header,
        Err(err) => {
            // Failed to parse atom header
            todo!("Handle error: {:?}", err)
        },
    };



    let mut publisher = Arc::try_unwrap(moq_ctx.publisher.clone()).unwrap().into_inner().unwrap();
    let _track = publisher.create_track("1").unwrap();


    // TODO: Do stuff with this atom
    // let track = moq_ctx.publisher.create_track("1")?;
    // let segment = track.create_segment(sequence, order)?;
    // segment.bytes(data);
    //
    // TODO: handle error if connection dropped

    // TODO: Read more than one atom per write?

    // return number of bytes read
    atom.len().try_into().unwrap()
}
//moq_close
#[no_mangle]
pub extern "C" fn moq_close(url_ctx_ptr: *mut URLContext) -> c_int {
    println!("moq_close");
    let url_context = unsafe { *url_ctx_ptr };
    let moq_ctx_ptr = url_context.priv_data as *mut MoqContext;
    let moq_ctx = unsafe { &mut *moq_ctx_ptr };

    // get owned publisher out of Arc<Mutex<...>>
    let publisher = Arc::try_unwrap(moq_ctx.publisher.clone()).unwrap().into_inner().unwrap();
    // close publisher
    publisher.close(moq_transport::Error::Closed).unwrap();

    // TODO: get result of session.run() from session_join_handle?

    0

}
