// machine generated, do not edit

use super::gfx as sg;

#[repr(C)]
pub struct Range {
    ptr: *const std::ffi::c_void,
    size: usize,
}
#[repr(C)]
pub struct Mat4 {
    m: [[f32; 4]; 4],
}
#[repr(C)]
pub struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    normal: u32,
    u: u16,
    v: u16,
    color: u32,
}
#[repr(C)]
pub struct ElementRange {
    base_element: u32,
    num_elements: u32,
    __pad: [u32; 3],
}
#[repr(C)]
pub struct SizesItem {
    num: u32,
    size: u32,
    __pad: [u32; 3],
}
#[repr(C)]
pub struct Sizes {
    vertices: SizesItem,
    indices: SizesItem,
}
#[repr(C)]
pub struct BufferItem {
    buffer: Range,
    data_size: usize,
    shape_offset: usize,
}
#[repr(C)]
pub struct Buffer {
    valid: bool,
    vertices: BufferItem,
    indices: BufferItem,
}
#[repr(C)]
pub struct Plane {
    width: f32,
    depth: f32,
    tiles: u16,
    color: u32,
    random_colors: bool,
    merge: bool,
    transform: Mat4,
}
#[repr(C)]
pub struct Box {
    width: f32,
    height: f32,
    depth: f32,
    tiles: u16,
    color: u32,
    random_colors: bool,
    merge: bool,
    transform: Mat4,
}
#[repr(C)]
pub struct Sphere {
    radius: f32,
    slices: u16,
    stacks: u16,
    color: u32,
    random_colors: bool,
    merge: bool,
    transform: Mat4,
}
#[repr(C)]
pub struct Cylinder {
    radius: f32,
    height: f32,
    slices: u16,
    stacks: u16,
    color: u32,
    random_colors: bool,
    merge: bool,
    transform: Mat4,
}
#[repr(C)]
pub struct Torus {
    radius: f32,
    ring_radius: f32,
    sides: u16,
    rings: u16,
    color: u32,
    random_colors: bool,
    merge: bool,
    transform: Mat4,
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
