use anyhow::Context;
use std::{
    collections::HashMap,
    mem::size_of,
    os::raw::{c_char, c_int, c_uchar, c_void},
    ptr::null,
    str::FromStr,
    sync::{Arc, Mutex},
};

use moq_transport::model::broadcast;
use moq_transport::model::track;

/// cbindgen:ignore
#[allow(non_camel_case_types)]
mod ffmpeg_types;
use ffmpeg_types::*;

mod ffmpeg;
use ffmpeg::*;

mod moq;
use moq::*;

mod media;
use media::*;
