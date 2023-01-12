// machine generated, do not edit

use super::gfx as sg;

#[repr(C)]
pub struct Pipeline {
    id: u32,
}
#[repr(C)]
pub struct Context {
    id: u32,
}
#[repr(C)]
pub enum Error {
    NoError = 0,
    VerticesFull,
    UniformsFull,
    CommandsFull,
    StackOverflow,
    StackUnderflow,
    NoContext,
}
#[repr(C)]
pub struct ContextDesc {
    max_vertices: i32,
    max_commands: i32,
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
    max_vertices: i32,
    max_commands: i32,
    context_pool_size: i32,
    pipeline_pool_size: i32,
    color_format: sg::PixelFormat,
    depth_format: sg::PixelFormat,
    sample_count: i32,
    face_winding: sg::FaceWinding,
    allocator: Allocator,
    logger: Logger,
}
extern { pub fn sgl_setup(desc: *const Desc) -> (); }
pub fn setup(desc: Desc) -> () { unsafe {
    return sgl_setup(&desc);
} }
extern { pub fn sgl_shutdown() -> (); }
pub fn shutdown() -> () { unsafe {
    return sgl_shutdown();
} }
extern { pub fn sgl_rad(deg: f32) -> f32; }
pub fn as_radians(deg: f32) -> f32 { unsafe {
    return sgl_rad(deg);
} }
extern { pub fn sgl_deg(rad: f32) -> f32; }
pub fn as_degrees(rad: f32) -> f32 { unsafe {
    return sgl_deg(rad);
} }
extern { pub fn sgl_error() -> Error; }
pub fn get_error() -> Error { unsafe {
    return sgl_error();
} }
extern { pub fn sgl_context_error(ctx: Context) -> Error; }
pub fn context_error(ctx: Context) -> Error { unsafe {
    return sgl_context_error(ctx);
} }
extern { pub fn sgl_make_context(desc: *const ContextDesc) -> Context; }
pub fn make_context(desc: ContextDesc) -> Context { unsafe {
    return sgl_make_context(&desc);
} }
extern { pub fn sgl_destroy_context(ctx: Context) -> (); }
pub fn destroy_context(ctx: Context) -> () { unsafe {
    return sgl_destroy_context(ctx);
} }
extern { pub fn sgl_set_context(ctx: Context) -> (); }
pub fn set_context(ctx: Context) -> () { unsafe {
    return sgl_set_context(ctx);
} }
extern { pub fn sgl_get_context() -> Context; }
pub fn get_context() -> Context { unsafe {
    return sgl_get_context();
} }
extern { pub fn sgl_default_context() -> Context; }
pub fn default_context() -> Context { unsafe {
    return sgl_default_context();
} }
extern { pub fn sgl_draw() -> (); }
pub fn draw() -> () { unsafe {
    return sgl_draw();
} }
extern { pub fn sgl_context_draw(ctx: Context) -> (); }
pub fn context_draw(ctx: Context) -> () { unsafe {
    return sgl_context_draw(ctx);
} }
extern { pub fn sgl_draw_layer(layer_id: i32) -> (); }
pub fn draw_layer(layer_id: i32) -> () { unsafe {
    return sgl_draw_layer(layer_id);
} }
extern { pub fn sgl_context_draw_layer(ctx: Context, layer_id: i32) -> (); }
pub fn context_draw_layer(ctx: Context, layer_id: i32) -> () { unsafe {
    return sgl_context_draw_layer(ctx, layer_id);
} }
extern { pub fn sgl_make_pipeline(desc: *const sg::PipelineDesc) -> Pipeline; }
pub fn make_pipeline(desc: sg::PipelineDesc) -> Pipeline { unsafe {
    return sgl_make_pipeline(&desc);
} }
extern { pub fn sgl_context_make_pipeline(ctx: Context, desc: *const sg::PipelineDesc) -> Pipeline; }
pub fn context_make_pipeline(ctx: Context, desc: sg::PipelineDesc) -> Pipeline { unsafe {
    return sgl_context_make_pipeline(ctx, &desc);
} }
extern { pub fn sgl_destroy_pipeline(pip: Pipeline) -> (); }
pub fn destroy_pipeline(pip: Pipeline) -> () { unsafe {
    return sgl_destroy_pipeline(pip);
} }
extern { pub fn sgl_defaults() -> (); }
pub fn defaults() -> () { unsafe {
    return sgl_defaults();
} }
extern { pub fn sgl_viewport(x: i32, y: i32, w: i32, h: i32, origin_top_left: bool) -> (); }
pub fn viewport(x: i32, y: i32, w: i32, h: i32, origin_top_left: bool) -> () { unsafe {
    return sgl_viewport(x, y, w, h, origin_top_left);
} }
extern { pub fn sgl_viewportf(x: f32, y: f32, w: f32, h: f32, origin_top_left: bool) -> (); }
pub fn viewportf(x: f32, y: f32, w: f32, h: f32, origin_top_left: bool) -> () { unsafe {
    return sgl_viewportf(x, y, w, h, origin_top_left);
} }
extern { pub fn sgl_scissor_rect(x: i32, y: i32, w: i32, h: i32, origin_top_left: bool) -> (); }
pub fn scissor_rect(x: i32, y: i32, w: i32, h: i32, origin_top_left: bool) -> () { unsafe {
    return sgl_scissor_rect(x, y, w, h, origin_top_left);
} }
extern { pub fn sgl_scissor_rectf(x: f32, y: f32, w: f32, h: f32, origin_top_left: bool) -> (); }
pub fn scissor_rectf(x: f32, y: f32, w: f32, h: f32, origin_top_left: bool) -> () { unsafe {
    return sgl_scissor_rectf(x, y, w, h, origin_top_left);
} }
extern { pub fn sgl_enable_texture() -> (); }
pub fn enable_texture() -> () { unsafe {
    return sgl_enable_texture();
} }
extern { pub fn sgl_disable_texture() -> (); }
pub fn disable_texture() -> () { unsafe {
    return sgl_disable_texture();
} }
extern { pub fn sgl_texture(img: sg::Image) -> (); }
pub fn texture(img: sg::Image) -> () { unsafe {
    return sgl_texture(img);
} }
extern { pub fn sgl_layer(layer_id: i32) -> (); }
pub fn layer(layer_id: i32) -> () { unsafe {
    return sgl_layer(layer_id);
} }
extern { pub fn sgl_load_default_pipeline() -> (); }
pub fn load_default_pipeline() -> () { unsafe {
    return sgl_load_default_pipeline();
} }
extern { pub fn sgl_load_pipeline(pip: Pipeline) -> (); }
pub fn load_pipeline(pip: Pipeline) -> () { unsafe {
    return sgl_load_pipeline(pip);
} }
extern { pub fn sgl_push_pipeline() -> (); }
pub fn push_pipeline() -> () { unsafe {
    return sgl_push_pipeline();
} }
extern { pub fn sgl_pop_pipeline() -> (); }
pub fn pop_pipeline() -> () { unsafe {
    return sgl_pop_pipeline();
} }
extern { pub fn sgl_matrix_mode_modelview() -> (); }
pub fn matrix_mode_modelview() -> () { unsafe {
    return sgl_matrix_mode_modelview();
} }
extern { pub fn sgl_matrix_mode_projection() -> (); }
pub fn matrix_mode_projection() -> () { unsafe {
    return sgl_matrix_mode_projection();
} }
extern { pub fn sgl_matrix_mode_texture() -> (); }
pub fn matrix_mode_texture() -> () { unsafe {
    return sgl_matrix_mode_texture();
} }
extern { pub fn sgl_load_identity() -> (); }
pub fn load_identity() -> () { unsafe {
    return sgl_load_identity();
} }
extern { pub fn sgl_load_matrix(m: *const f32) -> (); }
pub fn load_matrix(m: *const f32) -> () { unsafe {
    return sgl_load_matrix(m);
} }
extern { pub fn sgl_load_transpose_matrix(m: *const f32) -> (); }
pub fn load_transpose_matrix(m: *const f32) -> () { unsafe {
    return sgl_load_transpose_matrix(m);
} }
extern { pub fn sgl_mult_matrix(m: *const f32) -> (); }
pub fn mult_matrix(m: *const f32) -> () { unsafe {
    return sgl_mult_matrix(m);
} }
extern { pub fn sgl_mult_transpose_matrix(m: *const f32) -> (); }
pub fn mult_transpose_matrix(m: *const f32) -> () { unsafe {
    return sgl_mult_transpose_matrix(m);
} }
extern { pub fn sgl_rotate(angle_rad: f32, x: f32, y: f32, z: f32) -> (); }
pub fn rotate(angle_rad: f32, x: f32, y: f32, z: f32) -> () { unsafe {
    return sgl_rotate(angle_rad, x, y, z);
} }
extern { pub fn sgl_scale(x: f32, y: f32, z: f32) -> (); }
pub fn scale(x: f32, y: f32, z: f32) -> () { unsafe {
    return sgl_scale(x, y, z);
} }
extern { pub fn sgl_translate(x: f32, y: f32, z: f32) -> (); }
pub fn translate(x: f32, y: f32, z: f32) -> () { unsafe {
    return sgl_translate(x, y, z);
} }
extern { pub fn sgl_frustum(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> (); }
pub fn frustum(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> () { unsafe {
    return sgl_frustum(l, r, b, t, n, f);
} }
extern { pub fn sgl_ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> (); }
pub fn ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> () { unsafe {
    return sgl_ortho(l, r, b, t, n, f);
} }
extern { pub fn sgl_perspective(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> (); }
pub fn perspective(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> () { unsafe {
    return sgl_perspective(fov_y, aspect, z_near, z_far);
} }
extern { pub fn sgl_lookat(eye_x: f32, eye_y: f32, eye_z: f32, center_x: f32, center_y: f32, center_z: f32, up_x: f32, up_y: f32, up_z: f32) -> (); }
pub fn lookat(eye_x: f32, eye_y: f32, eye_z: f32, center_x: f32, center_y: f32, center_z: f32, up_x: f32, up_y: f32, up_z: f32) -> () { unsafe {
    return sgl_lookat(eye_x, eye_y, eye_z, center_x, center_y, center_z, up_x, up_y, up_z);
} }
extern { pub fn sgl_push_matrix() -> (); }
pub fn push_matrix() -> () { unsafe {
    return sgl_push_matrix();
} }
extern { pub fn sgl_pop_matrix() -> (); }
pub fn pop_matrix() -> () { unsafe {
    return sgl_pop_matrix();
} }
extern { pub fn sgl_t2f(u: f32, v: f32) -> (); }
pub fn t2f(u: f32, v: f32) -> () { unsafe {
    return sgl_t2f(u, v);
} }
extern { pub fn sgl_c3f(r: f32, g: f32, b: f32) -> (); }
pub fn c3f(r: f32, g: f32, b: f32) -> () { unsafe {
    return sgl_c3f(r, g, b);
} }
extern { pub fn sgl_c4f(r: f32, g: f32, b: f32, a: f32) -> (); }
pub fn c4f(r: f32, g: f32, b: f32, a: f32) -> () { unsafe {
    return sgl_c4f(r, g, b, a);
} }
extern { pub fn sgl_c3b(r: u8, g: u8, b: u8) -> (); }
pub fn c3b(r: u8, g: u8, b: u8) -> () { unsafe {
    return sgl_c3b(r, g, b);
} }
extern { pub fn sgl_c4b(r: u8, g: u8, b: u8, a: u8) -> (); }
pub fn c4b(r: u8, g: u8, b: u8, a: u8) -> () { unsafe {
    return sgl_c4b(r, g, b, a);
} }
extern { pub fn sgl_c1i(rgba: u32) -> (); }
pub fn c1i(rgba: u32) -> () { unsafe {
    return sgl_c1i(rgba);
} }
extern { pub fn sgl_point_size(s: f32) -> (); }
pub fn point_size(s: f32) -> () { unsafe {
    return sgl_point_size(s);
} }
extern { pub fn sgl_begin_points() -> (); }
pub fn begin_points() -> () { unsafe {
    return sgl_begin_points();
} }
extern { pub fn sgl_begin_lines() -> (); }
pub fn begin_lines() -> () { unsafe {
    return sgl_begin_lines();
} }
extern { pub fn sgl_begin_line_strip() -> (); }
pub fn begin_line_strip() -> () { unsafe {
    return sgl_begin_line_strip();
} }
extern { pub fn sgl_begin_triangles() -> (); }
pub fn begin_triangles() -> () { unsafe {
    return sgl_begin_triangles();
} }
extern { pub fn sgl_begin_triangle_strip() -> (); }
pub fn begin_triangle_strip() -> () { unsafe {
    return sgl_begin_triangle_strip();
} }
extern { pub fn sgl_begin_quads() -> (); }
pub fn begin_quads() -> () { unsafe {
    return sgl_begin_quads();
} }
extern { pub fn sgl_v2f(x: f32, y: f32) -> (); }
pub fn v2f(x: f32, y: f32) -> () { unsafe {
    return sgl_v2f(x, y);
} }
extern { pub fn sgl_v3f(x: f32, y: f32, z: f32) -> (); }
pub fn v3f(x: f32, y: f32, z: f32) -> () { unsafe {
    return sgl_v3f(x, y, z);
} }
extern { pub fn sgl_v2f_t2f(x: f32, y: f32, u: f32, v: f32) -> (); }
pub fn v2f_t2f(x: f32, y: f32, u: f32, v: f32) -> () { unsafe {
    return sgl_v2f_t2f(x, y, u, v);
} }
extern { pub fn sgl_v3f_t2f(x: f32, y: f32, z: f32, u: f32, v: f32) -> (); }
pub fn v3f_t2f(x: f32, y: f32, z: f32, u: f32, v: f32) -> () { unsafe {
    return sgl_v3f_t2f(x, y, z, u, v);
} }
extern { pub fn sgl_v2f_c3f(x: f32, y: f32, r: f32, g: f32, b: f32) -> (); }
pub fn v2f_c3f(x: f32, y: f32, r: f32, g: f32, b: f32) -> () { unsafe {
    return sgl_v2f_c3f(x, y, r, g, b);
} }
extern { pub fn sgl_v2f_c3b(x: f32, y: f32, r: u8, g: u8, b: u8) -> (); }
pub fn v2f_c3b(x: f32, y: f32, r: u8, g: u8, b: u8) -> () { unsafe {
    return sgl_v2f_c3b(x, y, r, g, b);
} }
extern { pub fn sgl_v2f_c4f(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) -> (); }
pub fn v2f_c4f(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) -> () { unsafe {
    return sgl_v2f_c4f(x, y, r, g, b, a);
} }
extern { pub fn sgl_v2f_c4b(x: f32, y: f32, r: u8, g: u8, b: u8, a: u8) -> (); }
pub fn v2f_c4b(x: f32, y: f32, r: u8, g: u8, b: u8, a: u8) -> () { unsafe {
    return sgl_v2f_c4b(x, y, r, g, b, a);
} }
extern { pub fn sgl_v2f_c1i(x: f32, y: f32, rgba: u32) -> (); }
pub fn v2f_c1i(x: f32, y: f32, rgba: u32) -> () { unsafe {
    return sgl_v2f_c1i(x, y, rgba);
} }
extern { pub fn sgl_v3f_c3f(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> (); }
pub fn v3f_c3f(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> () { unsafe {
    return sgl_v3f_c3f(x, y, z, r, g, b);
} }
extern { pub fn sgl_v3f_c3b(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8) -> (); }
pub fn v3f_c3b(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8) -> () { unsafe {
    return sgl_v3f_c3b(x, y, z, r, g, b);
} }
extern { pub fn sgl_v3f_c4f(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, a: f32) -> (); }
pub fn v3f_c4f(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, a: f32) -> () { unsafe {
    return sgl_v3f_c4f(x, y, z, r, g, b, a);
} }
extern { pub fn sgl_v3f_c4b(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8, a: u8) -> (); }
pub fn v3f_c4b(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8, a: u8) -> () { unsafe {
    return sgl_v3f_c4b(x, y, z, r, g, b, a);
} }
extern { pub fn sgl_v3f_c1i(x: f32, y: f32, z: f32, rgba: u32) -> (); }
pub fn v3f_c1i(x: f32, y: f32, z: f32, rgba: u32) -> () { unsafe {
    return sgl_v3f_c1i(x, y, z, rgba);
} }
extern { pub fn sgl_v2f_t2f_c3f(x: f32, y: f32, u: f32, v: f32, r: f32, g: f32, b: f32) -> (); }
pub fn v2f_t2f_c3f(x: f32, y: f32, u: f32, v: f32, r: f32, g: f32, b: f32) -> () { unsafe {
    return sgl_v2f_t2f_c3f(x, y, u, v, r, g, b);
} }
extern { pub fn sgl_v2f_t2f_c3b(x: f32, y: f32, u: f32, v: f32, r: u8, g: u8, b: u8) -> (); }
pub fn v2f_t2f_c3b(x: f32, y: f32, u: f32, v: f32, r: u8, g: u8, b: u8) -> () { unsafe {
    return sgl_v2f_t2f_c3b(x, y, u, v, r, g, b);
} }
extern { pub fn sgl_v2f_t2f_c4f(x: f32, y: f32, u: f32, v: f32, r: f32, g: f32, b: f32, a: f32) -> (); }
pub fn v2f_t2f_c4f(x: f32, y: f32, u: f32, v: f32, r: f32, g: f32, b: f32, a: f32) -> () { unsafe {
    return sgl_v2f_t2f_c4f(x, y, u, v, r, g, b, a);
} }
extern { pub fn sgl_v2f_t2f_c4b(x: f32, y: f32, u: f32, v: f32, r: u8, g: u8, b: u8, a: u8) -> (); }
pub fn v2f_t2f_c4b(x: f32, y: f32, u: f32, v: f32, r: u8, g: u8, b: u8, a: u8) -> () { unsafe {
    return sgl_v2f_t2f_c4b(x, y, u, v, r, g, b, a);
} }
extern { pub fn sgl_v2f_t2f_c1i(x: f32, y: f32, u: f32, v: f32, rgba: u32) -> (); }
pub fn v2f_t2f_c1i(x: f32, y: f32, u: f32, v: f32, rgba: u32) -> () { unsafe {
    return sgl_v2f_t2f_c1i(x, y, u, v, rgba);
} }
extern { pub fn sgl_v3f_t2f_c3f(x: f32, y: f32, z: f32, u: f32, v: f32, r: f32, g: f32, b: f32) -> (); }
pub fn v3f_t2f_c3f(x: f32, y: f32, z: f32, u: f32, v: f32, r: f32, g: f32, b: f32) -> () { unsafe {
    return sgl_v3f_t2f_c3f(x, y, z, u, v, r, g, b);
} }
extern { pub fn sgl_v3f_t2f_c3b(x: f32, y: f32, z: f32, u: f32, v: f32, r: u8, g: u8, b: u8) -> (); }
pub fn v3f_t2f_c3b(x: f32, y: f32, z: f32, u: f32, v: f32, r: u8, g: u8, b: u8) -> () { unsafe {
    return sgl_v3f_t2f_c3b(x, y, z, u, v, r, g, b);
} }
extern { pub fn sgl_v3f_t2f_c4f(x: f32, y: f32, z: f32, u: f32, v: f32, r: f32, g: f32, b: f32, a: f32) -> (); }
pub fn v3f_t2f_c4f(x: f32, y: f32, z: f32, u: f32, v: f32, r: f32, g: f32, b: f32, a: f32) -> () { unsafe {
    return sgl_v3f_t2f_c4f(x, y, z, u, v, r, g, b, a);
} }
extern { pub fn sgl_v3f_t2f_c4b(x: f32, y: f32, z: f32, u: f32, v: f32, r: u8, g: u8, b: u8, a: u8) -> (); }
pub fn v3f_t2f_c4b(x: f32, y: f32, z: f32, u: f32, v: f32, r: u8, g: u8, b: u8, a: u8) -> () { unsafe {
    return sgl_v3f_t2f_c4b(x, y, z, u, v, r, g, b, a);
} }
extern { pub fn sgl_v3f_t2f_c1i(x: f32, y: f32, z: f32, u: f32, v: f32, rgba: u32) -> (); }
pub fn v3f_t2f_c1i(x: f32, y: f32, z: f32, u: f32, v: f32, rgba: u32) -> () { unsafe {
    return sgl_v3f_t2f_c1i(x, y, z, u, v, rgba);
} }
extern { pub fn sgl_end() -> (); }
pub fn end() -> () { unsafe {
    return sgl_end();
} }
