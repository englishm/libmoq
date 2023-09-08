use std::os::raw::{c_char, c_int, c_uchar, c_void};

// Placeholder artisinally hand transcribed definitions
// until we create proper bindgen generated ones
// and/or update an existing ffmpeg-sys/ffmpeg-sys-next crate

/// cbindgen:ignore
#[repr(C)]
pub struct URLProtocol {
    //const char *name;
    pub name: *const c_char,
    //int     (*url_open)( URLContext *h, const char *url, int flags);
    pub url_open: *const fn(*mut URLContext, *const c_char, c_int) -> c_int,
    // int     (*url_open2)(URLContext *h, const char *url, int flags, AVDictionary **options);
    pub url_open2: *const fn(*mut URLContext) -> c_int,
    // int     (*url_accept)(URLContext *s, URLContext **c);
    pub url_accept: *const fn(*mut URLContext, *mut URLContext) -> c_int,
    // int     (*url_handshake)(URLContext *c);
    pub url_handshake: *const fn(*mut URLContext) -> c_int,
    // int     (*url_read)( URLContext *h, unsigned char *buf, int size);
    pub url_read: *const fn(*mut URLContext, *mut c_uchar, c_int) -> c_int,
    // int     (*url_write)(URLContext *h, const unsigned char *buf, int size);
    pub url_write: *const fn(*mut URLContext, *const c_char, c_int) -> c_int,
    // int64_t (*url_seek)( URLContext *h, int64_t pos, int whence);
    pub url_seek: *const fn(*mut URLContext, stdint::int64_t, c_int) -> c_int,
    // int     (*url_close)(URLContext *h);
    pub url_close: *const fn(*mut URLContext),
    // int (*url_read_pause)(URLContext *h, int pause);
    pub url_read_pause: *const fn(*mut URLContext, c_int) -> c_int,
    // int64_t (*url_read_seek)(URLContext *h, int stream_index,
    //                          int64_t timestamp, int flags);
    pub url_read_seek: *const fn(*mut URLContext, c_int, stdint::int64_t, c_int),
    // int (*url_get_file_handle)(URLContext *h);
    pub url_get_file_handle: *const fn(*mut URLContext) -> c_int,
    // int (*url_get_multi_file_handle)(URLContext *h, int **handles,
    //                                  int *numhandles);
    pub url_get_multi_file_handle: *const fn(*mut URLContext, c_int, c_int) -> c_int,
    // int (*url_get_short_seek)(URLContext *h);
    pub url_get_short_seek: *const fn(*mut URLContext) -> c_int,
    // int (*url_shutdown)(URLContext *h, int flags);
    pub url_shutdown: *const fn(*mut URLContext, c_int) -> c_int,
    // const AVClass *priv_data_class;
    pub priv_data_class: Option<AVClass>,
    // int priv_data_size;
    pub priv_data_size: c_int,
    // int flags;
    pub flags: c_int,
    // int (*url_check)(URLContext *h, int mask);
    pub url_check: *const fn(*mut URLContext, c_int) -> c_int,
    // int (*url_open_dir)(URLContext *h);
    pub url_open_dir: *const fn(*mut URLContext) -> c_int,
    // int (*url_read_dir)(URLContext *h, AVIODirEntry **next);
    pub url_read_dir: *const fn(*mut URLContext, AVIODirEntry) -> c_int,
    // int (*url_close_dir)(URLContext *h);
    pub url_close_dir: *const fn(*mut URLContext) -> c_int,
    // int (*url_delete)(URLContext *h);
    pub url_delete: *const fn(*mut URLContext) -> c_int,
    // int (*url_move)(URLContext *h_src, URLContext *h_dst);
    pub url_move: *const fn(*mut URLContext, *mut URLContext),
    // const char *default_whitelist;
    pub default_whitelist: *const c_char,
}

/// cbindgen:ignore
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct URLContext {
    // const AVClass *av_class;    /**< information for av_log(). Set by url_open(). */
    #[doc = "information for av_log(). Set by url_open()."]
    pub av_class: *const AVClass,
    // const struct URLProtocol *prot;
    pub prot: *const URLProtocol,
    // void *priv_data;
    pub priv_data: *mut c_void,
    // char *filename;             /**< specified URL */
    #[doc = "specified URL"]
    pub filename: *mut c_char,
    // int flags;
    pub flags: c_int,
    // int max_packet_size;        /**< if non zero, the stream is packetized with this max packet size */
    #[doc = "if non zero, the stream is packetized with this max packet size"]
    pub max_packet_size: c_int,
    // int is_streamed;            /**< true if streamed (no seek possible), default = false */
    #[doc = "true if streamed (no seek possible), default = false"]
    pub is_streamed: c_int,
    // int is_connected;
    pub is_connected: c_int,
    // AVIOInterruptCB interrupt_callback;
    pub interrupt_callback: AVIOInterruptCB,
    // int64_t rw_timeout;         /**< maximum time to wait for (network) read/write operation completion, in mcs */
    #[doc = "maximum time to wait for (network) read/write operation completion, in mcs"]
    pub rw_timeout: stdint::int64_t,
    // const char *protocol_whitelist;
    pub protocol_whitelist: *const c_char,
    // const char *protocol_blacklist;
    pub protocol_blacklist: *const c_char,
    // int min_packet_size;        /**< if non zero, the stream is packetized with this min packet size */
    #[doc = "if non zero, the stream is packetized with this min packet size"]
    pub min_packet_size: c_int,
}

/// cbindgen:ignore
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVIOInterruptCB {
    //     int (*callback)(void*);
    pub callback: *const fn() -> c_int,
    //     void *opaque;
    pub opaque: *mut c_void,
}

// below copied from ffmpeg-sys-next bindgen output as needed to support above
#[doc = " Describe the class of an AVClass context structure. That is an\n arbitrary struct of which the first field is a pointer to an\n AVClass struct (e.g. AVCodecContext, AVFormatContext etc.)."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVClass {
    #[doc = " The name of the class; usually it is the same name as the\n context structure type to which the AVClass is associated."]
    pub class_name: *const c_char,
    #[doc = " A pointer to a function which returns the name of a context\n instance ctx associated with the class."]
    pub item_name: ::std::option::Option<unsafe extern "C" fn(ctx: *mut c_void) -> *const c_char>,
    #[doc = " a pointer to the first option specified in the class if any or NULL\n\n @see av_set_default_options()"]
    pub option: *const AVOption,
    #[doc = " LIBAVUTIL_VERSION with which this structure was created.\n This is used to allow fields to be added without requiring major\n version bumps everywhere."]
    pub version: c_int,
    #[doc = " Offset in the structure where log_level_offset is stored.\n 0 means there is no such variable"]
    pub log_level_offset_offset: c_int,
    #[doc = " Offset in the structure where a pointer to the parent context for\n logging is stored. For example a decoder could pass its AVCodecContext\n to eval as such a parent context, which an av_log() implementation\n could then leverage to display the parent context.\n The offset can be NULL."]
    pub parent_log_context_offset: c_int,
    #[doc = " Category used for visualization (like color)\n This is only set if the category is equal for all objects using this class.\n available since version (51 << 16 | 56 << 8 | 100)"]
    pub category: AVClassCategory,
    #[doc = " Callback to return the category.\n available since version (51 << 16 | 59 << 8 | 100)"]
    pub get_category:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut c_void) -> AVClassCategory>,
    #[doc = " Callback to return the supported/allowed ranges.\n available since version (52.12)"]
    pub query_ranges: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut *mut AVOptionRanges,
            obj: *mut c_void,
            key: *const c_char,
            flags: c_int,
        ) -> c_int,
    >,
    #[doc = " Return next AVOptions-enabled child or NULL"]
    pub child_next: ::std::option::Option<
        unsafe extern "C" fn(obj: *mut c_void, prev: *mut c_void) -> *mut c_void,
    >,
    #[doc = " Iterate over the AVClasses corresponding to potential AVOptions-enabled\n children.\n\n @param iter pointer to opaque iteration state. The caller must initialize\n             *iter to NULL before the first call.\n @return AVClass for the next AVOptions-enabled child or NULL if there are\n         no more such children.\n\n @note The difference between child_next and this is that child_next\n       iterates over _already existing_ objects, while child_class_iterate\n       iterates over _all possible_ children."]
    pub child_class_iterate:
        ::std::option::Option<unsafe extern "C" fn(iter: *mut *mut c_void) -> *const AVClass>,
}

#[doc = " AVOption"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AVOption {
    pub name: *const c_char,
    #[doc = " short English help text\n @todo What about other languages?"]
    pub help: *const c_char,
    #[doc = " The offset relative to the context structure where the option\n value is stored. It should be 0 for named constants."]
    pub offset: c_int,
    pub type_: AVOptionType,
    pub default_val: AVOption__bindgen_ty_1,
    #[doc = "< minimum valid value for the option"]
    pub min: f64,
    #[doc = "< maximum valid value for the option"]
    pub max: f64,
    pub flags: c_int,
    #[doc = " The logical unit to which the option belongs. Non-constant\n options and corresponding named constants share the same\n unit. May be NULL."]
    pub unit: *const c_char,
}

#[doc = " the default value for scalar options"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union AVOption__bindgen_ty_1 {
    pub i64_: i64,
    pub dbl: f64,
    pub str_: *const c_char,
    pub q: AVRational,
}
#[repr(u32)]
#[doc = " @defgroup avoptions AVOptions\n @ingroup lavu_data\n @{\n AVOptions provide a generic system to declare options on arbitrary structs\n (\"objects\"). An option can have a help text, a type and a range of possible\n values. Options may then be enumerated, read and written to.\n\n @section avoptions_implement Implementing AVOptions\n This section describes how to add AVOptions capabilities to a struct.\n\n All AVOptions-related information is stored in an AVClass. Therefore\n the first member of the struct should be a pointer to an AVClass describing it.\n The option field of the AVClass must be set to a NULL-terminated static array\n of AVOptions. Each AVOption must have a non-empty name, a type, a default\n value and for number-type AVOptions also a range of allowed values. It must\n also declare an offset in bytes from the start of the struct, where the field\n associated with this AVOption is located. Other fields in the AVOption struct\n should also be set when applicable, but are not required.\n\n The following example illustrates an AVOptions-enabled struct:\n @code\n typedef struct test_struct {\n     const AVClass *class;\n     int      int_opt;\n     char    *str_opt;\n     uint8_t *bin_opt;\n     int      bin_len;\n } test_struct;\n\n static const AVOption test_options[] = {\n   { \"test_int\", \"This is a test option of int type.\", offsetof(test_struct, int_opt),\n     AV_OPT_TYPE_INT, { .i64 = -1 }, INT_MIN, INT_MAX },\n   { \"test_str\", \"This is a test option of string type.\", offsetof(test_struct, str_opt),\n     AV_OPT_TYPE_STRING },\n   { \"test_bin\", \"This is a test option of binary type.\", offsetof(test_struct, bin_opt),\n     AV_OPT_TYPE_BINARY },\n   { NULL },\n };\n\n static const AVClass test_class = {\n     .class_name = \"test class\",\n     .item_name  = av_default_item_name,\n     .option     = test_options,\n     .version    = LIBAVUTIL_VERSION_INT,\n };\n @endcode\n\n Next, when allocating your struct, you must ensure that the AVClass pointer\n is set to the correct value. Then, av_opt_set_defaults() can be called to\n initialize defaults. After that the struct is ready to be used with the\n AVOptions API.\n\n When cleaning up, you may use the av_opt_free() function to automatically\n free all the allocated string and binary options.\n\n Continuing with the above example:\n\n @code\n test_struct *alloc_test_struct(void)\n {\n     test_struct *ret = av_mallocz(sizeof(*ret));\n     ret->class = &test_class;\n     av_opt_set_defaults(ret);\n     return ret;\n }\n void free_test_struct(test_struct **foo)\n {\n     av_opt_free(*foo);\n     av_freep(foo);\n }\n @endcode\n\n @subsection avoptions_implement_nesting Nesting\n      It may happen that an AVOptions-enabled struct contains another\n      AVOptions-enabled struct as a member (e.g. AVCodecContext in\n      libavcodec exports generic options, while its priv_data field exports\n      codec-specific options). In such a case, it is possible to set up the\n      parent struct to export a child's options. To do that, simply\n      implement AVClass.child_next() and AVClass.child_class_iterate() in the\n      parent struct's AVClass.\n      Assuming that the test_struct from above now also contains a\n      child_struct field:\n\n      @code\n      typedef struct child_struct {\n          AVClass *class;\n          int flags_opt;\n      } child_struct;\n      static const AVOption child_opts[] = {\n          { \"test_flags\", \"This is a test option of flags type.\",\n            offsetof(child_struct, flags_opt), AV_OPT_TYPE_FLAGS, { .i64 = 0 }, INT_MIN, INT_MAX },\n          { NULL },\n      };\n      static const AVClass child_class = {\n          .class_name = \"child class\",\n          .item_name  = av_default_item_name,\n          .option     = child_opts,\n          .version    = LIBAVUTIL_VERSION_INT,\n      };\n\n      void *child_next(void *obj, void *prev)\n      {\n          test_struct *t = obj;\n          if (!prev && t->child_struct)\n              return t->child_struct;\n          return NULL\n      }\n      const AVClass child_class_iterate(void **iter)\n      {\n          const AVClass *c = *iter ? NULL : &child_class;\n          *iter = (void*)(uintptr_t)c;\n          return c;\n      }\n      @endcode\n      Putting child_next() and child_class_iterate() as defined above into\n      test_class will now make child_struct's options accessible through\n      test_struct (again, proper setup as described above needs to be done on\n      child_struct right after it is created).\n\n      From the above example it might not be clear why both child_next()\n      and child_class_iterate() are needed. The distinction is that child_next()\n      iterates over actually existing objects, while child_class_iterate()\n      iterates over all possible child classes. E.g. if an AVCodecContext\n      was initialized to use a codec which has private options, then its\n      child_next() will return AVCodecContext.priv_data and finish\n      iterating. OTOH child_class_iterate() on AVCodecContext.av_class will\n      iterate over all available codecs with private options.\n\n @subsection avoptions_implement_named_constants Named constants\n      It is possible to create named constants for options. Simply set the unit\n      field of the option the constants should apply to a string and\n      create the constants themselves as options of type AV_OPT_TYPE_CONST\n      with their unit field set to the same string.\n      Their default_val field should contain the value of the named\n      constant.\n      For example, to add some named constants for the test_flags option\n      above, put the following into the child_opts array:\n      @code\n      { \"test_flags\", \"This is a test option of flags type.\",\n        offsetof(child_struct, flags_opt), AV_OPT_TYPE_FLAGS, { .i64 = 0 }, INT_MIN, INT_MAX, \"test_unit\" },\n      { \"flag1\", \"This is a flag with value 16\", 0, AV_OPT_TYPE_CONST, { .i64 = 16 }, 0, 0, \"test_unit\" },\n      @endcode\n\n @section avoptions_use Using AVOptions\n This section deals with accessing options in an AVOptions-enabled struct.\n Such structs in FFmpeg are e.g. AVCodecContext in libavcodec or\n AVFormatContext in libavformat.\n\n @subsection avoptions_use_examine Examining AVOptions\n The basic functions for examining options are av_opt_next(), which iterates\n over all options defined for one object, and av_opt_find(), which searches\n for an option with the given name.\n\n The situation is more complicated with nesting. An AVOptions-enabled struct\n may have AVOptions-enabled children. Passing the AV_OPT_SEARCH_CHILDREN flag\n to av_opt_find() will make the function search children recursively.\n\n For enumerating there are basically two cases. The first is when you want to\n get all options that may potentially exist on the struct and its children\n (e.g.  when constructing documentation). In that case you should call\n av_opt_child_class_iterate() recursively on the parent struct's AVClass.  The\n second case is when you have an already initialized struct with all its\n children and you want to get all options that can be actually written or read\n from it. In that case you should call av_opt_child_next() recursively (and\n av_opt_next() on each result).\n\n @subsection avoptions_use_get_set Reading and writing AVOptions\n When setting options, you often have a string read directly from the\n user. In such a case, simply passing it to av_opt_set() is enough. For\n non-string type options, av_opt_set() will parse the string according to the\n option type.\n\n Similarly av_opt_get() will read any option type and convert it to a string\n which will be returned. Do not forget that the string is allocated, so you\n have to free it with av_free().\n\n In some cases it may be more convenient to put all options into an\n AVDictionary and call av_opt_set_dict() on it. A specific case of this\n are the format/codec open functions in lavf/lavc which take a dictionary\n filled with option as a parameter. This makes it possible to set some options\n that cannot be set otherwise, since e.g. the input file format is not known\n before the file is actually opened."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AVOptionType {
    AV_OPT_TYPE_FLAGS = 0,
    AV_OPT_TYPE_INT = 1,
    AV_OPT_TYPE_INT64 = 2,
    AV_OPT_TYPE_DOUBLE = 3,
    AV_OPT_TYPE_FLOAT = 4,
    AV_OPT_TYPE_STRING = 5,
    AV_OPT_TYPE_RATIONAL = 6,
    #[doc = "< offset must point to a pointer immediately followed by an int for the length"]
    AV_OPT_TYPE_BINARY = 7,
    AV_OPT_TYPE_DICT = 8,
    AV_OPT_TYPE_UINT64 = 9,
    AV_OPT_TYPE_CONST = 10,
    #[doc = "< offset must point to two consecutive integers"]
    AV_OPT_TYPE_IMAGE_SIZE = 11,
    AV_OPT_TYPE_PIXEL_FMT = 12,
    AV_OPT_TYPE_SAMPLE_FMT = 13,
    #[doc = "< offset must point to AVRational"]
    AV_OPT_TYPE_VIDEO_RATE = 14,
    AV_OPT_TYPE_DURATION = 15,
    AV_OPT_TYPE_COLOR = 16,
    AV_OPT_TYPE_CHANNEL_LAYOUT = 17,
    AV_OPT_TYPE_BOOL = 18,
    AV_OPT_TYPE_CHLAYOUT = 19,
}

#[doc = " Rational number (pair of numerator and denominator)."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVRational {
    #[doc = "< Numerator"]
    pub num: c_int,
    #[doc = "< Denominator"]
    pub den: c_int,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AVClassCategory {
    AV_CLASS_CATEGORY_NA = 0,
    AV_CLASS_CATEGORY_INPUT = 1,
    AV_CLASS_CATEGORY_OUTPUT = 2,
    AV_CLASS_CATEGORY_MUXER = 3,
    AV_CLASS_CATEGORY_DEMUXER = 4,
    AV_CLASS_CATEGORY_ENCODER = 5,
    AV_CLASS_CATEGORY_DECODER = 6,
    AV_CLASS_CATEGORY_FILTER = 7,
    AV_CLASS_CATEGORY_BITSTREAM_FILTER = 8,
    AV_CLASS_CATEGORY_SWSCALER = 9,
    AV_CLASS_CATEGORY_SWRESAMPLER = 10,
    AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT = 40,
    AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT = 41,
    AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT = 42,
    AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT = 43,
    AV_CLASS_CATEGORY_DEVICE_OUTPUT = 44,
    AV_CLASS_CATEGORY_DEVICE_INPUT = 45,
    #[doc = "< not part of ABI/API"]
    AV_CLASS_CATEGORY_NB = 46,
}

#[doc = " A single allowed range of values, or a single allowed value."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AVOptionRange {
    pub str_: *const c_char,
    #[doc = " Value range.\n For string ranges this represents the min/max length.\n For dimensions this represents the min/max pixel count or width/height in multi-component case."]
    pub value_min: f64,
    #[doc = " Value range.\n For string ranges this represents the min/max length.\n For dimensions this represents the min/max pixel count or width/height in multi-component case."]
    pub value_max: f64,
    #[doc = " Value's component range.\n For string this represents the unicode range for chars, 0-127 limits to ASCII."]
    pub component_min: f64,
    #[doc = " Value's component range.\n For string this represents the unicode range for chars, 0-127 limits to ASCII."]
    pub component_max: f64,
    #[doc = " Range flag.\n If set to 1 the struct encodes a range, if set to 0 a single value."]
    pub is_range: c_int,
}

#[doc = " List of AVOptionRange structs."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVOptionRanges {
    #[doc = " Array of option ranges.\n\n Most of option types use just one component.\n Following describes multi-component option types:\n\n AV_OPT_TYPE_IMAGE_SIZE:\n component index 0: range of pixel count (width * height).\n component index 1: range of width.\n component index 2: range of height.\n\n @note To obtain multi-component version of this structure, user must\n       provide AV_OPT_MULTI_COMPONENT_RANGE to av_opt_query_ranges or\n       av_opt_query_ranges_default function.\n\n Multi-component range can be read as in following example:\n\n @code\n int range_index, component_index;\n AVOptionRanges *ranges;\n AVOptionRange *range[3]; //may require more than 3 in the future.\n av_opt_query_ranges(&ranges, obj, key, AV_OPT_MULTI_COMPONENT_RANGE);\n for (range_index = 0; range_index < ranges->nb_ranges; range_index++) {\n     for (component_index = 0; component_index < ranges->nb_components; component_index++)\n         range[component_index] = ranges->range[ranges->nb_ranges * component_index + range_index];\n     //do something with range here.\n }\n av_opt_freep_ranges(&ranges);\n @endcode"]
    pub range: *mut *mut AVOptionRange,
    #[doc = " Number of ranges per component."]
    pub nb_ranges: c_int,
    #[doc = " Number of componentes."]
    pub nb_components: c_int,
}

#[doc = " Describes single entry of the directory.\n\n Only name and type fields are guaranteed be set.\n Rest of fields are protocol or/and platform dependent and might be unknown."]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AVIODirEntry {
    #[doc = "< Filename"]
    pub name: *mut c_char,
    #[doc = "< Type of the entry"]
    pub type_: c_int,
    #[doc = "< Set to 1 when name is encoded with UTF-8, 0 otherwise.\nName can be encoded with UTF-8 even though 0 is set."]
    pub utf8: c_int,
    #[doc = "< File size in bytes, -1 if unknown."]
    pub size: i64,
    #[doc = "< Time of last modification in microseconds since unix\nepoch, -1 if unknown."]
    pub modification_timestamp: i64,
    #[doc = "< Time of last access in microseconds since unix epoch,\n-1 if unknown."]
    pub access_timestamp: i64,
    #[doc = "< Time of last status change in microseconds since unix\nepoch, -1 if unknown."]
    pub status_change_timestamp: i64,
    #[doc = "< User ID of owner, -1 if unknown."]
    pub user_id: i64,
    #[doc = "< Group ID of owner, -1 if unknown."]
    pub group_id: i64,
    #[doc = "< Unix file mode, -1 if unknown."]
    pub filemode: i64,
}
