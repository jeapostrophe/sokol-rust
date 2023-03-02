// machine generated, do not edit


pub const SAPP_MAX_TOUCHPOINTS: i32 = 8;
pub const SAPP_MAX_MOUSEBUTTONS: i32 = 3;
pub const SAPP_MAX_KEYCODES: i32 = 512;
pub const SAPP_MAX_ICONIMAGES: i32 = 8;
#[repr(C)]
pub enum EventType {
    Invalid,
    KeyDown,
    KeyUp,
    Char,
    MouseDown,
    MouseUp,
    MouseScroll,
    MouseMove,
    MouseEnter,
    MouseLeave,
    TouchesBegan,
    TouchesMoved,
    TouchesEnded,
    TouchesCancelled,
    Resized,
    Iconified,
    Restored,
    Focused,
    Unfocused,
    Suspended,
    Resumed,
    QuitRequested,
    ClipboardPasted,
    FilesDropped,
    Num,
    ForceU32 = 2147483647,
}
#[repr(C)]
pub enum Keycode {
    Invalid = 0,
    Space = 32,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Num0 = 48,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
    Semicolon = 59,
    Equal = 61,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    GraveAccent = 96,
    World1 = 161,
    World2 = 162,
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    F13 = 302,
    F14 = 303,
    F15 = 304,
    F16 = 305,
    F17 = 306,
    F18 = 307,
    F19 = 308,
    F20 = 309,
    F21 = 310,
    F22 = 311,
    F23 = 312,
    F24 = 313,
    F25 = 314,
    Kp0 = 320,
    Kp1 = 321,
    Kp2 = 322,
    Kp3 = 323,
    Kp4 = 324,
    Kp5 = 325,
    Kp6 = 326,
    Kp7 = 327,
    Kp8 = 328,
    Kp9 = 329,
    KpDecimal = 330,
    KpDivide = 331,
    KpMultiply = 332,
    KpSubtract = 333,
    KpAdd = 334,
    KpEnter = 335,
    KpEqual = 336,
    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    Menu = 348,
}
#[repr(C)]
pub enum AndroidTooltype {
    Unknown = 0,
    Finger = 1,
    Stylus = 2,
    Mouse = 3,
}
#[repr(C)]
pub struct Touchpoint {
    pub identifier: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub android_tooltype: AndroidTooltype,
    pub changed: bool,
}
#[repr(C)]
pub enum Mousebutton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Invalid = 256,
}
pub const SAPP_MODIFIER_SHIFT: i32 = 1;
pub const SAPP_MODIFIER_CTRL: i32 = 2;
pub const SAPP_MODIFIER_ALT: i32 = 4;
pub const SAPP_MODIFIER_SUPER: i32 = 8;
pub const SAPP_MODIFIER_LMB: i32 = 256;
pub const SAPP_MODIFIER_RMB: i32 = 512;
pub const SAPP_MODIFIER_MMB: i32 = 1024;
#[repr(C)]
pub struct Event {
    pub frame_count: u64,
    pub r#type: EventType,
    pub key_code: Keycode,
    pub char_code: u32,
    pub key_repeat: bool,
    pub modifiers: u32,
    pub mouse_button: Mousebutton,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_dx: f32,
    pub mouse_dy: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub num_touches: i32,
    pub touches: [Touchpoint; 8],
    pub window_width: i32,
    pub window_height: i32,
    pub framebuffer_width: i32,
    pub framebuffer_height: i32,
}
#[repr(C)]
pub struct Range {
    pub ptr: *const std::ffi::c_void,
    pub size: usize,
}
#[repr(C)]
pub struct ImageDesc {
    pub width: i32,
    pub height: i32,
    pub pixels: Range,
}
#[repr(C)]
pub struct IconDesc {
    pub sokol_default: bool,
    pub images: [ImageDesc; 8],
}
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
    pub init_cb: *const extern fn() -> (),
    pub frame_cb: *const extern fn() -> (),
    pub cleanup_cb: *const extern fn() -> (),
    pub event_cb: *const extern fn(*const Event) -> (),
    pub fail_cb: *const extern fn(*const u8) -> (),
    pub user_data: *mut std::ffi::c_void,
    pub init_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> (),
    pub frame_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> (),
    pub cleanup_userdata_cb: *const extern fn(*mut std::ffi::c_void) -> (),
    pub event_userdata_cb: *const extern fn(*const Event, *mut std::ffi::c_void) -> (),
    pub fail_userdata_cb: *const extern fn(*const u8, *mut std::ffi::c_void) -> (),
    pub width: i32,
    pub height: i32,
    pub sample_count: i32,
    pub swap_interval: i32,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub window_title: *mut u8,
    pub enable_clipboard: bool,
    pub clipboard_size: i32,
    pub enable_dragndrop: bool,
    pub max_dropped_files: i32,
    pub max_dropped_file_path_length: i32,
    pub icon: IconDesc,
    pub allocator: Allocator,
    pub logger: Logger,
    pub gl_force_gles2: bool,
    pub gl_major_version: i32,
    pub gl_minor_version: i32,
    pub win32_console_utf8: bool,
    pub win32_console_create: bool,
    pub win32_console_attach: bool,
    pub html5_canvas_name: *mut u8,
    pub html5_canvas_resize: bool,
    pub html5_preserve_drawing_buffer: bool,
    pub html5_premultiplied_alpha: bool,
    pub html5_ask_leave_site: bool,
    pub ios_keyboard_resizes_canvas: bool,
}
#[repr(C)]
pub enum Html5FetchError {
    FetchErrorNoError,
    FetchErrorBufferTooSmall,
    FetchErrorOther,
}
#[repr(C)]
pub struct Html5FetchResponse {
    pub succeeded: bool,
    pub error_code: Html5FetchError,
    pub file_index: i32,
    pub data: Range,
    pub buffer: Range,
    pub user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Html5FetchRequest {
    pub dropped_file_index: i32,
    pub callback: *const extern fn(*const Html5FetchResponse) -> (),
    pub buffer: Range,
    pub user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub enum MouseCursor {
    Default = 0,
    Arrow,
    Ibeam,
    Crosshair,
    PointingHand,
    ResizeEw,
    ResizeNs,
    ResizeNwse,
    ResizeNesw,
    ResizeAll,
    NotAllowed,
    Num,
}
extern { pub fn sapp_isvalid() -> bool; }
pub fn isvalid() -> bool { unsafe {
    return sapp_isvalid();
} }
extern { pub fn sapp_width() -> i32; }
pub fn width() -> i32 { unsafe {
    return sapp_width();
} }
extern { pub fn sapp_widthf() -> f32; }
pub fn widthf() -> f32 { unsafe {
    return sapp_widthf();
} }
extern { pub fn sapp_height() -> i32; }
pub fn height() -> i32 { unsafe {
    return sapp_height();
} }
extern { pub fn sapp_heightf() -> f32; }
pub fn heightf() -> f32 { unsafe {
    return sapp_heightf();
} }
extern { pub fn sapp_color_format() -> i32; }
pub fn color_format() -> i32 { unsafe {
    return sapp_color_format();
} }
extern { pub fn sapp_depth_format() -> i32; }
pub fn depth_format() -> i32 { unsafe {
    return sapp_depth_format();
} }
extern { pub fn sapp_sample_count() -> i32; }
pub fn sample_count() -> i32 { unsafe {
    return sapp_sample_count();
} }
extern { pub fn sapp_high_dpi() -> bool; }
pub fn high_dpi() -> bool { unsafe {
    return sapp_high_dpi();
} }
extern { pub fn sapp_dpi_scale() -> f32; }
pub fn dpi_scale() -> f32 { unsafe {
    return sapp_dpi_scale();
} }
extern { pub fn sapp_show_keyboard(show: bool) -> (); }
pub fn show_keyboard(show: bool) -> () { unsafe {
    return sapp_show_keyboard(show);
} }
extern { pub fn sapp_keyboard_shown() -> bool; }
pub fn keyboard_shown() -> bool { unsafe {
    return sapp_keyboard_shown();
} }
extern { pub fn sapp_is_fullscreen() -> bool; }
pub fn is_fullscreen() -> bool { unsafe {
    return sapp_is_fullscreen();
} }
extern { pub fn sapp_toggle_fullscreen() -> (); }
pub fn toggle_fullscreen() -> () { unsafe {
    return sapp_toggle_fullscreen();
} }
extern { pub fn sapp_show_mouse(show: bool) -> (); }
pub fn show_mouse(show: bool) -> () { unsafe {
    return sapp_show_mouse(show);
} }
extern { pub fn sapp_mouse_shown() -> bool; }
pub fn mouse_shown() -> bool { unsafe {
    return sapp_mouse_shown();
} }
extern { pub fn sapp_lock_mouse(lock: bool) -> (); }
pub fn lock_mouse(lock: bool) -> () { unsafe {
    return sapp_lock_mouse(lock);
} }
extern { pub fn sapp_mouse_locked() -> bool; }
pub fn mouse_locked() -> bool { unsafe {
    return sapp_mouse_locked();
} }
extern { pub fn sapp_set_mouse_cursor(cursor: MouseCursor) -> (); }
pub fn set_mouse_cursor(cursor: MouseCursor) -> () { unsafe {
    return sapp_set_mouse_cursor(cursor);
} }
extern { pub fn sapp_get_mouse_cursor() -> MouseCursor; }
pub fn get_mouse_cursor() -> MouseCursor { unsafe {
    return sapp_get_mouse_cursor();
} }
extern { pub fn sapp_userdata() -> *mut std::ffi::c_void; }
pub fn userdata() -> *mut std::ffi::c_void { unsafe {
    return sapp_userdata();
} }
extern { pub fn sapp_query_desc() -> Desc; }
pub fn query_desc() -> Desc { unsafe {
    return sapp_query_desc();
} }
extern { pub fn sapp_request_quit() -> (); }
pub fn request_quit() -> () { unsafe {
    return sapp_request_quit();
} }
extern { pub fn sapp_cancel_quit() -> (); }
pub fn cancel_quit() -> () { unsafe {
    return sapp_cancel_quit();
} }
extern { pub fn sapp_quit() -> (); }
pub fn quit() -> () { unsafe {
    return sapp_quit();
} }
extern { pub fn sapp_consume_event() -> (); }
pub fn consume_event() -> () { unsafe {
    return sapp_consume_event();
} }
extern { pub fn sapp_frame_count() -> u64; }
pub fn frame_count() -> u64 { unsafe {
    return sapp_frame_count();
} }
extern { pub fn sapp_frame_duration() -> f64; }
pub fn frame_duration() -> f64 { unsafe {
    return sapp_frame_duration();
} }
extern { pub fn sapp_set_clipboard_string(str: *const u8) -> (); }
pub fn set_clipboard_string(str: *const u8) -> () { unsafe {
    return sapp_set_clipboard_string(str);
} }
extern { pub fn sapp_get_clipboard_string() -> *const u8; }
pub fn get_clipboard_string() -> *const u8 { unsafe {
    return sapp_get_clipboard_string();
} }
extern { pub fn sapp_set_window_title(str: *const u8) -> (); }
pub fn set_window_title(str: *const u8) -> () { unsafe {
    return sapp_set_window_title(str);
} }
extern { pub fn sapp_set_icon(icon_desc: *const IconDesc) -> (); }
pub fn set_icon(icon_desc: IconDesc) -> () { unsafe {
    return sapp_set_icon(&icon_desc);
} }
extern { pub fn sapp_get_num_dropped_files() -> i32; }
pub fn get_num_dropped_files() -> i32 { unsafe {
    return sapp_get_num_dropped_files();
} }
extern { pub fn sapp_get_dropped_file_path(index: i32) -> *const u8; }
pub fn get_dropped_file_path(index: i32) -> *const u8 { unsafe {
    return sapp_get_dropped_file_path(index);
} }
extern { pub fn sapp_run(desc: *const Desc) -> (); }
pub fn run(desc: Desc) -> () { unsafe {
    return sapp_run(&desc);
} }
extern { pub fn sapp_egl_get_display() -> *const std::ffi::c_void; }
pub fn egl_get_display() -> *const std::ffi::c_void { unsafe {
    return sapp_egl_get_display();
} }
extern { pub fn sapp_egl_get_context() -> *const std::ffi::c_void; }
pub fn egl_get_context() -> *const std::ffi::c_void { unsafe {
    return sapp_egl_get_context();
} }
extern { pub fn sapp_gles2() -> bool; }
pub fn gles2() -> bool { unsafe {
    return sapp_gles2();
} }
extern { pub fn sapp_html5_ask_leave_site(ask: bool) -> (); }
pub fn html5_ask_leave_site(ask: bool) -> () { unsafe {
    return sapp_html5_ask_leave_site(ask);
} }
extern { pub fn sapp_html5_get_dropped_file_size(index: i32) -> u32; }
pub fn html5_get_dropped_file_size(index: i32) -> u32 { unsafe {
    return sapp_html5_get_dropped_file_size(index);
} }
extern { pub fn sapp_html5_fetch_dropped_file(request: *const Html5FetchRequest) -> (); }
pub fn html5_fetch_dropped_file(request: Html5FetchRequest) -> () { unsafe {
    return sapp_html5_fetch_dropped_file(&request);
} }
extern { pub fn sapp_metal_get_device() -> *const std::ffi::c_void; }
pub fn metal_get_device() -> *const std::ffi::c_void { unsafe {
    return sapp_metal_get_device();
} }
extern { pub fn sapp_metal_get_renderpass_descriptor() -> *const std::ffi::c_void; }
pub fn metal_get_renderpass_descriptor() -> *const std::ffi::c_void { unsafe {
    return sapp_metal_get_renderpass_descriptor();
} }
extern { pub fn sapp_metal_get_drawable() -> *const std::ffi::c_void; }
pub fn metal_get_drawable() -> *const std::ffi::c_void { unsafe {
    return sapp_metal_get_drawable();
} }
extern { pub fn sapp_macos_get_window() -> *const std::ffi::c_void; }
pub fn macos_get_window() -> *const std::ffi::c_void { unsafe {
    return sapp_macos_get_window();
} }
extern { pub fn sapp_ios_get_window() -> *const std::ffi::c_void; }
pub fn ios_get_window() -> *const std::ffi::c_void { unsafe {
    return sapp_ios_get_window();
} }
extern { pub fn sapp_d3d11_get_device() -> *const std::ffi::c_void; }
pub fn d3d11_get_device() -> *const std::ffi::c_void { unsafe {
    return sapp_d3d11_get_device();
} }
extern { pub fn sapp_d3d11_get_device_context() -> *const std::ffi::c_void; }
pub fn d3d11_get_device_context() -> *const std::ffi::c_void { unsafe {
    return sapp_d3d11_get_device_context();
} }
extern { pub fn sapp_d3d11_get_swap_chain() -> *const std::ffi::c_void; }
pub fn d3d11_get_swap_chain() -> *const std::ffi::c_void { unsafe {
    return sapp_d3d11_get_swap_chain();
} }
extern { pub fn sapp_d3d11_get_render_target_view() -> *const std::ffi::c_void; }
pub fn d3d11_get_render_target_view() -> *const std::ffi::c_void { unsafe {
    return sapp_d3d11_get_render_target_view();
} }
extern { pub fn sapp_d3d11_get_depth_stencil_view() -> *const std::ffi::c_void; }
pub fn d3d11_get_depth_stencil_view() -> *const std::ffi::c_void { unsafe {
    return sapp_d3d11_get_depth_stencil_view();
} }
extern { pub fn sapp_win32_get_hwnd() -> *const std::ffi::c_void; }
pub fn win32_get_hwnd() -> *const std::ffi::c_void { unsafe {
    return sapp_win32_get_hwnd();
} }
extern { pub fn sapp_wgpu_get_device() -> *const std::ffi::c_void; }
pub fn wgpu_get_device() -> *const std::ffi::c_void { unsafe {
    return sapp_wgpu_get_device();
} }
extern { pub fn sapp_wgpu_get_render_view() -> *const std::ffi::c_void; }
pub fn wgpu_get_render_view() -> *const std::ffi::c_void { unsafe {
    return sapp_wgpu_get_render_view();
} }
extern { pub fn sapp_wgpu_get_resolve_view() -> *const std::ffi::c_void; }
pub fn wgpu_get_resolve_view() -> *const std::ffi::c_void { unsafe {
    return sapp_wgpu_get_resolve_view();
} }
extern { pub fn sapp_wgpu_get_depth_stencil_view() -> *const std::ffi::c_void; }
pub fn wgpu_get_depth_stencil_view() -> *const std::ffi::c_void { unsafe {
    return sapp_wgpu_get_depth_stencil_view();
} }
extern { pub fn sapp_android_get_native_activity() -> *const std::ffi::c_void; }
pub fn android_get_native_activity() -> *const std::ffi::c_void { unsafe {
    return sapp_android_get_native_activity();
} }
