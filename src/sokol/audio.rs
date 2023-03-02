// machine generated, do not edit


#[repr(C)]
pub struct Allocator {
    pub alloc: *const extern fn(usize, *mut std::ffi::c_void) -> *mut std::ffi::c_void,
    pub free: *const extern fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> (),
    pub user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Logger {
    pub log_cb: *const extern fn(*const u8, *mut std::ffi::c_void) -> (),
    pub user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Desc {
    pub sample_rate: i32,
    pub num_channels: i32,
    pub buffer_frames: i32,
    pub packet_frames: i32,
    pub num_packets: i32,
    pub stream_cb: *const extern fn(*mut f32, i32, i32) -> (),
    pub stream_userdata_cb: *const extern fn(*mut f32, i32, i32, *mut std::ffi::c_void) -> (),
    pub user_data: *mut std::ffi::c_void,
    pub allocator: Allocator,
    pub logger: Logger,
}
extern { pub fn saudio_setup(desc: *const Desc) -> (); }
pub fn setup(desc: Desc) -> () { unsafe {
    return saudio_setup(&desc);
} }
extern { pub fn saudio_shutdown() -> (); }
pub fn shutdown() -> () { unsafe {
    return saudio_shutdown();
} }
extern { pub fn saudio_isvalid() -> bool; }
pub fn isvalid() -> bool { unsafe {
    return saudio_isvalid();
} }
extern { pub fn saudio_userdata() -> *mut std::ffi::c_void; }
pub fn userdata() -> *mut std::ffi::c_void { unsafe {
    return saudio_userdata();
} }
extern { pub fn saudio_query_desc() -> Desc; }
pub fn query_desc() -> Desc { unsafe {
    return saudio_query_desc();
} }
extern { pub fn saudio_sample_rate() -> i32; }
pub fn sample_rate() -> i32 { unsafe {
    return saudio_sample_rate();
} }
extern { pub fn saudio_buffer_frames() -> i32; }
pub fn buffer_frames() -> i32 { unsafe {
    return saudio_buffer_frames();
} }
extern { pub fn saudio_channels() -> i32; }
pub fn channels() -> i32 { unsafe {
    return saudio_channels();
} }
extern { pub fn saudio_suspended() -> bool; }
pub fn suspended() -> bool { unsafe {
    return saudio_suspended();
} }
extern { pub fn saudio_expect() -> i32; }
pub fn expect() -> i32 { unsafe {
    return saudio_expect();
} }
extern { pub fn saudio_push(frames: *const f32, num_frames: i32) -> i32; }
pub fn push(frames: *const f32, num_frames: i32) -> i32 { unsafe {
    return saudio_push(frames, num_frames);
} }
