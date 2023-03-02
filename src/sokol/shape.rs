// machine generated, do not edit

use super::gfx as sg;

#[repr(C)]
pub struct Range {
    pub ptr: *const std::ffi::c_void,
    pub size: usize,
}
#[repr(C)]
pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}
#[repr(C)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub normal: u32,
    pub u: u16,
    pub v: u16,
    pub color: u32,
}
#[repr(C)]
pub struct ElementRange {
    pub base_element: u32,
    pub num_elements: u32,
}
#[repr(C)]
pub struct SizesItem {
    pub num: u32,
    pub size: u32,
}
#[repr(C)]
pub struct Sizes {
    pub vertices: SizesItem,
    pub indices: SizesItem,
}
#[repr(C)]
pub struct BufferItem {
    pub buffer: Range,
    pub data_size: usize,
    pub shape_offset: usize,
}
#[repr(C)]
pub struct Buffer {
    pub valid: bool,
    pub vertices: BufferItem,
    pub indices: BufferItem,
}
#[repr(C)]
pub struct Plane {
    pub width: f32,
    pub depth: f32,
    pub tiles: u16,
    pub color: u32,
    pub random_colors: bool,
    pub merge: bool,
    pub transform: Mat4,
}
#[repr(C)]
pub struct Box {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub tiles: u16,
    pub color: u32,
    pub random_colors: bool,
    pub merge: bool,
    pub transform: Mat4,
}
#[repr(C)]
pub struct Sphere {
    pub radius: f32,
    pub slices: u16,
    pub stacks: u16,
    pub color: u32,
    pub random_colors: bool,
    pub merge: bool,
    pub transform: Mat4,
}
#[repr(C)]
pub struct Cylinder {
    pub radius: f32,
    pub height: f32,
    pub slices: u16,
    pub stacks: u16,
    pub color: u32,
    pub random_colors: bool,
    pub merge: bool,
    pub transform: Mat4,
}
#[repr(C)]
pub struct Torus {
    pub radius: f32,
    pub ring_radius: f32,
    pub sides: u16,
    pub rings: u16,
    pub color: u32,
    pub random_colors: bool,
    pub merge: bool,
    pub transform: Mat4,
}
extern { pub fn sshape_build_plane(buf: *const Buffer, params: *const Plane) -> Buffer; }
pub fn build_plane(buf: Buffer, params: Plane) -> Buffer { unsafe {
    return sshape_build_plane(&buf, &params);
} }
extern { pub fn sshape_build_box(buf: *const Buffer, params: *const Box) -> Buffer; }
pub fn build_box(buf: Buffer, params: Box) -> Buffer { unsafe {
    return sshape_build_box(&buf, &params);
} }
extern { pub fn sshape_build_sphere(buf: *const Buffer, params: *const Sphere) -> Buffer; }
pub fn build_sphere(buf: Buffer, params: Sphere) -> Buffer { unsafe {
    return sshape_build_sphere(&buf, &params);
} }
extern { pub fn sshape_build_cylinder(buf: *const Buffer, params: *const Cylinder) -> Buffer; }
pub fn build_cylinder(buf: Buffer, params: Cylinder) -> Buffer { unsafe {
    return sshape_build_cylinder(&buf, &params);
} }
extern { pub fn sshape_build_torus(buf: *const Buffer, params: *const Torus) -> Buffer; }
pub fn build_torus(buf: Buffer, params: Torus) -> Buffer { unsafe {
    return sshape_build_torus(&buf, &params);
} }
extern { pub fn sshape_plane_sizes(tiles: u32) -> Sizes; }
pub fn plane_sizes(tiles: u32) -> Sizes { unsafe {
    return sshape_plane_sizes(tiles);
} }
extern { pub fn sshape_box_sizes(tiles: u32) -> Sizes; }
pub fn box_sizes(tiles: u32) -> Sizes { unsafe {
    return sshape_box_sizes(tiles);
} }
extern { pub fn sshape_sphere_sizes(slices: u32, stacks: u32) -> Sizes; }
pub fn sphere_sizes(slices: u32, stacks: u32) -> Sizes { unsafe {
    return sshape_sphere_sizes(slices, stacks);
} }
extern { pub fn sshape_cylinder_sizes(slices: u32, stacks: u32) -> Sizes; }
pub fn cylinder_sizes(slices: u32, stacks: u32) -> Sizes { unsafe {
    return sshape_cylinder_sizes(slices, stacks);
} }
extern { pub fn sshape_torus_sizes(sides: u32, rings: u32) -> Sizes; }
pub fn torus_sizes(sides: u32, rings: u32) -> Sizes { unsafe {
    return sshape_torus_sizes(sides, rings);
} }
extern { pub fn sshape_element_range(buf: *const Buffer) -> ElementRange; }
pub fn element_range(buf: Buffer) -> ElementRange { unsafe {
    return sshape_element_range(&buf);
} }
extern { pub fn sshape_vertex_buffer_desc(buf: *const Buffer) -> sg::BufferDesc; }
pub fn vertex_buffer_desc(buf: Buffer) -> sg::BufferDesc { unsafe {
    return sshape_vertex_buffer_desc(&buf);
} }
extern { pub fn sshape_index_buffer_desc(buf: *const Buffer) -> sg::BufferDesc; }
pub fn index_buffer_desc(buf: Buffer) -> sg::BufferDesc { unsafe {
    return sshape_index_buffer_desc(&buf);
} }
extern { pub fn sshape_buffer_layout_desc() -> sg::BufferLayoutDesc; }
pub fn buffer_layout_desc() -> sg::BufferLayoutDesc { unsafe {
    return sshape_buffer_layout_desc();
} }
extern { pub fn sshape_position_attr_desc() -> sg::VertexAttrDesc; }
pub fn position_attr_desc() -> sg::VertexAttrDesc { unsafe {
    return sshape_position_attr_desc();
} }
extern { pub fn sshape_normal_attr_desc() -> sg::VertexAttrDesc; }
pub fn normal_attr_desc() -> sg::VertexAttrDesc { unsafe {
    return sshape_normal_attr_desc();
} }
extern { pub fn sshape_texcoord_attr_desc() -> sg::VertexAttrDesc; }
pub fn texcoord_attr_desc() -> sg::VertexAttrDesc { unsafe {
    return sshape_texcoord_attr_desc();
} }
extern { pub fn sshape_color_attr_desc() -> sg::VertexAttrDesc; }
pub fn color_attr_desc() -> sg::VertexAttrDesc { unsafe {
    return sshape_color_attr_desc();
} }
extern { pub fn sshape_color_4f(r: f32, g: f32, b: f32, a: f32) -> u32; }
pub fn color_4f(r: f32, g: f32, b: f32, a: f32) -> u32 { unsafe {
    return sshape_color_4f(r, g, b, a);
} }
extern { pub fn sshape_color_3f(r: f32, g: f32, b: f32) -> u32; }
pub fn color_3f(r: f32, g: f32, b: f32) -> u32 { unsafe {
    return sshape_color_3f(r, g, b);
} }
extern { pub fn sshape_color_4b(r: u8, g: u8, b: u8, a: u8) -> u32; }
pub fn color_4b(r: u8, g: u8, b: u8, a: u8) -> u32 { unsafe {
    return sshape_color_4b(r, g, b, a);
} }
extern { pub fn sshape_color_3b(r: u8, g: u8, b: u8) -> u32; }
pub fn color_3b(r: u8, g: u8, b: u8) -> u32 { unsafe {
    return sshape_color_3b(r, g, b);
} }
extern { pub fn sshape_mat4(m: *const f32) -> Mat4; }
pub fn mat4(m: *const f32) -> Mat4 { unsafe {
    return sshape_mat4(m);
} }
extern { pub fn sshape_mat4_transpose(m: *const f32) -> Mat4; }
pub fn mat4_transpose(m: *const f32) -> Mat4 { unsafe {
    return sshape_mat4_transpose(m);
} }
