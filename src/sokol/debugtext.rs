// machine generated, do not edit

use super::gfx as sg;

#[repr(C)]
pub struct Context {
    id: u32,
}
#[repr(C)]
pub struct Range {
    ptr: *const std::ffi::c_void,
    size: usize,
}
#[repr(C)]
pub struct FontDesc {
    data: Range,
    first_char: u8,
    last_char: u8,
}
#[repr(C)]
pub struct ContextDesc {
    max_commands: i32,
    char_buf_size: i32,
    canvas_width: f32,
    canvas_height: f32,
    tab_width: i32,
    color_format: sg::PixelFormat,
    depth_format: sg::PixelFormat,
    sample_count: i32,
}
#[repr(C)]
pub struct Allocator {
    alloc: *const extern fn(usize, *mut std::ffi::c_void) -> *mut std::ffi::c_void,
    free: *const extern fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> (),
    user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Logger {
    log_cb: *const extern fn(*const u8, *mut std::ffi::c_void) -> (),
    user_data: *mut std::ffi::c_void,
}
#[repr(C)]
pub struct Desc {
    context_pool_size: i32,
    printf_buf_size: i32,
    fonts: [FontDesc; 8],
    context: ContextDesc,
    allocator: Allocator,
    logger: Logger,
}
extern { pub fn sdtx_setup(desc: *const Desc) -> (); }
pub fn setup(desc: Desc) -> () { unsafe {
    return sdtx_setup(&desc);
} }
extern { pub fn sdtx_shutdown() -> (); }
pub fn shutdown() -> () { unsafe {
    return sdtx_shutdown();
} }
extern { pub fn sdtx_font_kc853() -> FontDesc; }
pub fn font_kc853() -> FontDesc { unsafe {
    return sdtx_font_kc853();
} }
extern { pub fn sdtx_font_kc854() -> FontDesc; }
pub fn font_kc854() -> FontDesc { unsafe {
    return sdtx_font_kc854();
} }
extern { pub fn sdtx_font_z1013() -> FontDesc; }
pub fn font_z1013() -> FontDesc { unsafe {
    return sdtx_font_z1013();
} }
extern { pub fn sdtx_font_cpc() -> FontDesc; }
pub fn font_cpc() -> FontDesc { unsafe {
    return sdtx_font_cpc();
} }
extern { pub fn sdtx_font_c64() -> FontDesc; }
pub fn font_c64() -> FontDesc { unsafe {
    return sdtx_font_c64();
} }
extern { pub fn sdtx_font_oric() -> FontDesc; }
pub fn font_oric() -> FontDesc { unsafe {
    return sdtx_font_oric();
} }
extern { pub fn sdtx_make_context(desc: *const ContextDesc) -> Context; }
pub fn make_context(desc: ContextDesc) -> Context { unsafe {
    return sdtx_make_context(&desc);
} }
extern { pub fn sdtx_destroy_context(ctx: Context) -> (); }
pub fn destroy_context(ctx: Context) -> () { unsafe {
    return sdtx_destroy_context(ctx);
} }
extern { pub fn sdtx_set_context(ctx: Context) -> (); }
pub fn set_context(ctx: Context) -> () { unsafe {
    return sdtx_set_context(ctx);
} }
extern { pub fn sdtx_get_context() -> Context; }
pub fn get_context() -> Context { unsafe {
    return sdtx_get_context();
} }
extern { pub fn sdtx_default_context() -> Context; }
pub fn default_context() -> Context { unsafe {
    return sdtx_default_context();
} }
extern { pub fn sdtx_draw() -> (); }
pub fn draw() -> () { unsafe {
    return sdtx_draw();
} }
extern { pub fn sdtx_context_draw(ctx: Context) -> (); }
pub fn context_draw(ctx: Context) -> () { unsafe {
    return sdtx_context_draw(ctx);
} }
extern { pub fn sdtx_draw_layer(layer_id: i32) -> (); }
pub fn draw_layer(layer_id: i32) -> () { unsafe {
    return sdtx_draw_layer(layer_id);
} }
extern { pub fn sdtx_context_draw_layer(ctx: Context, layer_id: i32) -> (); }
pub fn context_draw_layer(ctx: Context, layer_id: i32) -> () { unsafe {
    return sdtx_context_draw_layer(ctx, layer_id);
} }
extern { pub fn sdtx_layer(layer_id: i32) -> (); }
pub fn layer(layer_id: i32) -> () { unsafe {
    return sdtx_layer(layer_id);
} }
extern { pub fn sdtx_font(font_index: u32) -> (); }
pub fn font(font_index: u32) -> () { unsafe {
    return sdtx_font(font_index);
} }
extern { pub fn sdtx_canvas(w: f32, h: f32) -> (); }
pub fn canvas(w: f32, h: f32) -> () { unsafe {
    return sdtx_canvas(w, h);
} }
extern { pub fn sdtx_origin(x: f32, y: f32) -> (); }
pub fn origin(x: f32, y: f32) -> () { unsafe {
    return sdtx_origin(x, y);
} }
extern { pub fn sdtx_home() -> (); }
pub fn home() -> () { unsafe {
    return sdtx_home();
} }
extern { pub fn sdtx_pos(x: f32, y: f32) -> (); }
pub fn pos(x: f32, y: f32) -> () { unsafe {
    return sdtx_pos(x, y);
} }
extern { pub fn sdtx_pos_x(x: f32) -> (); }
pub fn pos_x(x: f32) -> () { unsafe {
    return sdtx_pos_x(x);
} }
extern { pub fn sdtx_pos_y(y: f32) -> (); }
pub fn pos_y(y: f32) -> () { unsafe {
    return sdtx_pos_y(y);
} }
extern { pub fn sdtx_move(dx: f32, dy: f32) -> (); }
pub fn r#move(dx: f32, dy: f32) -> () { unsafe {
    return sdtx_move(dx, dy);
} }
extern { pub fn sdtx_move_x(dx: f32) -> (); }
pub fn move_x(dx: f32) -> () { unsafe {
    return sdtx_move_x(dx);
} }
extern { pub fn sdtx_move_y(dy: f32) -> (); }
pub fn move_y(dy: f32) -> () { unsafe {
    return sdtx_move_y(dy);
} }
extern { pub fn sdtx_crlf() -> (); }
pub fn crlf() -> () { unsafe {
    return sdtx_crlf();
} }
extern { pub fn sdtx_color3b(r: u8, g: u8, b: u8) -> (); }
pub fn color3b(r: u8, g: u8, b: u8) -> () { unsafe {
    return sdtx_color3b(r, g, b);
} }
extern { pub fn sdtx_color3f(r: f32, g: f32, b: f32) -> (); }
pub fn color3f(r: f32, g: f32, b: f32) -> () { unsafe {
    return sdtx_color3f(r, g, b);
} }
extern { pub fn sdtx_color4b(r: u8, g: u8, b: u8, a: u8) -> (); }
pub fn color4b(r: u8, g: u8, b: u8, a: u8) -> () { unsafe {
    return sdtx_color4b(r, g, b, a);
} }
extern { pub fn sdtx_color4f(r: f32, g: f32, b: f32, a: f32) -> (); }
pub fn color4f(r: f32, g: f32, b: f32, a: f32) -> () { unsafe {
    return sdtx_color4f(r, g, b, a);
} }
extern { pub fn sdtx_color1i(rgba: u32) -> (); }
pub fn color1i(rgba: u32) -> () { unsafe {
    return sdtx_color1i(rgba);
} }
extern { pub fn sdtx_putc(c: u8) -> (); }
pub fn putc(c: u8) -> () { unsafe {
    return sdtx_putc(c);
} }
extern { pub fn sdtx_puts(str: *const u8) -> (); }
pub fn puts(str: *const u8) -> () { unsafe {
    return sdtx_puts(str);
} }
extern { pub fn sdtx_putr(str: *const u8, len: i32) -> (); }
pub fn putr(str: *const u8, len: i32) -> () { unsafe {
    return sdtx_putr(str, len);
} }
