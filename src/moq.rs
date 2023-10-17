use anyhow::Context;
use std::{
    collections::HashMap,
    os::raw::c_int,
    str::FromStr,
    sync::Arc,
};
use moq_transport::model::broadcast;
use moq_transport::model::track;
use crate::ffmpeg_types::*;
use crate::media;

/// cbindgen:ignore
#[derive(Debug)]
#[repr(C)]
pub struct MoqContext {
    pub av_class: *const AVClass,
    pub foo: c_int,
    pub tracks: HashMap<String, media::Track>,
    pub publisher: Option<broadcast::Publisher>,
    pub session_join_handle: tokio::task::JoinHandle<()>,
    pub rt: tokio::runtime::Runtime,
    pub unread: Vec<u8>,
    pub track_name: Option<String>,
    pub _catalog: Option<track::Publisher>,
    pub _init: Option<track::Publisher>,
}

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
        let tracks = HashMap::new();

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()?;
        // let rt = tokio::runtime::Builder::new_current_thread()
        //     .enable_all()
        //     .build()?;
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

        // TODO: REMOVE THIS
        tls_config.key_log = Arc::new(rustls::KeyLogFile::new());

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
            publisher: Some(publisher),
            session_join_handle,
            rt,
            unread: vec![],
            track_name: None,
            _catalog: None,
            _init: None,
        })
    }
}
